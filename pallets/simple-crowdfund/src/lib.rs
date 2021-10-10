#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;


#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use codec::{Decode, Encode};
use frame_support::{
	traits::{Currency, ExistenceRequirement, Get, ReservableCurrency, WithdrawReasons},
	storage::child,
	PalletId,
};
use frame_system::ensure_signed;
use sp_core::Hasher;
use sp_runtime::traits::{AccountIdConversion, Saturating, Zero};
use sp_std::prelude::*;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
type FundInfoOf<T> =
	FundInfo<AccountIdOf<T>, BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;

type FundIndex = u32;

const PALLET_ID: PalletId = PalletId(*b"ex/cfund");

#[derive(Encode, Decode, Default, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct FundInfo<AccountId, Balance, BlockNumber> {
	/// The account that will receive the funds if the campaign is successful
	beneficiary: AccountId,
	/// The amount of deposit placed
	deposit: Balance,
	/// The total amount raised
	raised: Balance,
	/// Block number after which funding must have succeeded
	end: BlockNumber,
	/// Upper bound on `raised`
	goal: Balance,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::traits::{
		Currency, ExistenceRequirement, Get, ReservableCurrency, WithdrawReasons,
	};
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::sp_tracing::info;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: ReservableCurrency<Self::AccountId>;

		/// The amount to be held on deposit by the owner of a crowdfund
		type SubmissionDeposit: Get<BalanceOf<Self>>;

		/// The minimum amount that may be contributed into a crowdfund. Should almost certainly be at
		/// least ExistentialDeposit.
		type MinContribution: Get<BalanceOf<Self>>;

		/// The period of time (in blocks) after an unsuccessful crowdfund ending during which
		/// contributors are able to withdraw their funds. After this period, their funds are lost.
		type RetirementPeriod: Get<Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn funds)]
	pub type Funds<T> = StorageMap<_, Blake2_128Concat, FundIndex, FundInfoOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn fund_count)]
	pub type FundCount<T> = StorageValue<_, FundIndex, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId", BalanceOf<T> = "Balance", T::BlockNumber ="BlockNumber")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		Created(FundIndex, T::BlockNumber),
		Contributed(T::AccountId, FundIndex, BalanceOf<T>, T::BlockNumber),
		Withdrew(T::AccountId, FundIndex, BalanceOf<T>, T::BlockNumber),
		Retiring(FundIndex, T::BlockNumber),
		Dissolved(FundIndex, T::BlockNumber, T::AccountId),
		Dispensed(FundIndex, T::BlockNumber, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Crowdfund must end after it starts
		EndTooEarly,
		/// Must contribute at least the minimum amount of funds
		ContributionTooSmall,
		/// The fund index specified does not exist
		InvalidIndex,
		/// The crowdfund's contribution period has ended; no more contributions will be accepted
		ContributionPeriodOver,
		/// You may not withdraw or dispense funds while the fund is still active
		FundStillActive,
		/// You cannot withdraw funds because you have not contributed any
		NoContribution,
		/// You cannot dissolve a fund that has not yet completed its retirement period
		FundNotRetired,
		/// Cannot dispense funds from an unsuccessful fund
		UnsuccessfulFund,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				}
			}
		}
     
		/// Create a new fund
		#[pallet::weight(10_000)]
		pub fn create(
			origin: OriginFor<T>,
			beneficiary: AccountIdOf<T>,
			goal: BalanceOf<T>,
			end: T::BlockNumber,
		) -> DispatchResult {
			let creator = ensure_signed(origin)?;
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(end > now, Error::<T>::EndTooEarly);

			// pub const SubmissionDeposit: u128 = 10; in runtime
			let deposit = T::SubmissionDeposit::get();
			// info!("Deposit {:?}", deposit);
			
			
			// fn withdraw(
			// 	who: &AccountId,
			// 	value: Self::Balance,
			// 	reasons: WithdrawReasons,
			// 	liveness: ExistenceRequirement
			// ) -> Result<Self::NegativeImbalance, DispatchError>
			// Removes some free balance from who account for reason if possible. If liveness is KeepAlive, then no less than ExistentialDeposit must be left remaining.
			
			// This checks any locks, vesting, and liquidity requirements. If the removal is not possible, then it returns Err.
			
			// If the operation is successful, this will return Ok with a NegativeImbalance whose value is value.
			let imb = T::Currency::withdraw(
				&creator,
				deposit,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?;
			let index: FundIndex = <FundCount<T>>::get();

			info!("Fund index {:?}", index);

			<FundCount<T>>::put(index.checked_add(1).ok_or(Error::<T>::StorageOverflow)?);
            
            // fn deposit_creating(
			// 	who: &AccountId,
			// 	value: Self::Balance
			// ) -> Self::PositiveImbalance
			// Adds up to value to the free balance of who. If who doesn’t exist, it is created.
			
			// Infallible.

			// fn resolve_creating(who: &AccountId, value: Self::NegativeImbalance)
            // Similar to deposit_creating, only accepts a NegativeImbalance and returns nothing on success.
			// Negative imbalances are created when there is cut like validator slashed, transaction fees are collected
			T::Currency::resolve_creating(&Self::fund_account_id(index), imb);

			<Funds<T>>::insert(
				index,
				FundInfo { beneficiary, deposit, raised: Zero::zero(), end, goal },
			);

			Self::deposit_event(Event::<T>::Created(index, now));

			Ok(())
		}

		/// Contribute funds to an existing fund
		#[pallet::weight(10_000)]
		pub fn contribute(
			origin: OriginFor<T>, 
			index: FundIndex, 
			value: BalanceOf<T>) -> DispatchResultWithPostInfo {

			let who = ensure_signed(origin)?;

			ensure!(value >= T::MinContribution::get(), Error::<T>::ContributionTooSmall);
			let mut fund = Self::funds(index).ok_or(Error::<T>::InvalidIndex)?;

			// Make sure crowdfund has not ended
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(fund.end > now, Error::<T>::ContributionPeriodOver);

			// Add contribution to the fund
			T::Currency::transfer(
				&who,
				&Self::fund_account_id(index),
				value,
				ExistenceRequirement::AllowDeath
			)?;
			fund.raised += value;
			Funds::<T>::insert(index, &fund);

			let balance = Self::contribution_get(index, &who);
			let balance = balance.saturating_add(value);
			Self::contribution_put(index, &who, &balance);

			Self::deposit_event(Event::Contributed(who, index, balance, now));

			Ok(().into())
		}
		/// Withdraw full balance of a contributor to a fund
		#[pallet::weight(10_000)]
		pub fn withdraw(
			origin: OriginFor<T>,
			#[pallet::compact] index: FundIndex) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let mut fund = Self::funds(index).ok_or(Error::<T>::InvalidIndex)?;
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(fund.end < now, Error::<T>::FundStillActive);

			let balance = Self::contribution_get(index, &who);
			ensure!(balance > Zero::zero(), Error::<T>::NoContribution);

			// Return funds to caller without charging a transfer fee
			let _ = T::Currency::resolve_into_existing(&who, T::Currency::withdraw(
				&Self::fund_account_id(index),
				balance,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath
			)?);

			// Update storage
			Self::contribution_kill(index, &who);
			fund.raised = fund.raised.saturating_sub(balance);
			<Funds<T>>::insert(index, &fund);

			Self::deposit_event(Event::Withdrew(who, index, balance, now));

			Ok(().into())
		}

		/// Dissolve an entire crowdfund after its retirement period has expired.
		/// Anyone can call this function, and they are incentivized to do so because
		/// they inherit the deposit.
		#[pallet::weight(10_000)]
		pub fn dissolve(
			origin: OriginFor<T>, 
			index: FundIndex) -> DispatchResultWithPostInfo {
			let reporter = ensure_signed(origin)?;

			let fund = Self::funds(index).ok_or(Error::<T>::InvalidIndex)?;

			// Check that enough time has passed to remove from storage
			let now = <frame_system::Pallet<T>>::block_number();
			ensure!(now >= fund.end + T::RetirementPeriod::get(), Error::<T>::FundNotRetired);

			let account = Self::fund_account_id(index);

			// Dissolver collects the deposit and any remaining funds
			let _ = T::Currency::resolve_creating(&reporter, T::Currency::withdraw(
				&account,
				fund.deposit + fund.raised,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?);

			// Remove the fund info from storage
			<Funds<T>>::remove(index);
			// Remove all the contributor info from storage in a single write.
			// This is possible thanks to the use of a child tree.
			Self::crowdfund_kill(index);

			Self::deposit_event(Event::Dissolved(index, now, reporter));

			Ok(().into())
		}

		/// Dispense a payment to the beneficiary of a successful crowdfund.
		/// The beneficiary receives the contributed funds and the caller receives
		/// the deposit as a reward to incentivize clearing settled crowdfunds out of storage.
		#[pallet::weight(10_000)]
		pub fn dispense(
			origin: OriginFor<T>, 
			index: FundIndex) -> DispatchResultWithPostInfo {
			let caller = ensure_signed(origin)?;

			let fund = Self::funds(index).ok_or(Error::<T>::InvalidIndex)?;

			// Check that enough time has passed to remove from storage
			let now = <frame_system::Pallet<T>>::block_number();

			ensure!(now >= fund.end, Error::<T>::FundStillActive);

			// Check that the fund was actually successful
			ensure!(fund.raised >= fund.goal, Error::<T>::UnsuccessfulFund);

			let account = Self::fund_account_id(index);

			// Beneficiary collects the contributed funds
			let _ = T::Currency::resolve_creating(&fund.beneficiary, T::Currency::withdraw(
				&account,
				fund.raised,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?);

			// Caller collects the deposit
			let _ = T::Currency::resolve_creating(&caller, T::Currency::withdraw(
				&account,
				fund.deposit,
				WithdrawReasons::TRANSFER,
				ExistenceRequirement::AllowDeath,
			)?);

			// Remove the fund info from storage
			<Funds<T>>::remove(index);
			// Remove all the contributor info from storage in a single write.
			// This is possible thanks to the use of a child tree.
			Self::crowdfund_kill(index);

			Self::deposit_event(Event::Dispensed(index, now, caller));
			Ok(().into())
		}

	}

	impl<T: Config> Pallet<T> {
		pub fn fund_account_id(index: FundIndex) -> T::AccountId {
			PALLET_ID.into_sub_account(index)
		}

		/// Find the ID associated with the fund
			///
			/// Each fund stores information about its contributors and their contributions in a child trie
			/// This helper function calculates the id of the associated child trie.
			pub fn id_from_index(index: FundIndex) -> child::ChildInfo {
				let mut buf = Vec::new();
				buf.extend_from_slice(b"crowdfnd");
				buf.extend_from_slice(&index.to_le_bytes()[..]);

				child::ChildInfo::new_default(T::Hashing::hash(&buf[..]).as_ref())
			}

			/// Record a contribution in the associated child trie.
			pub fn contribution_put(index: FundIndex, who: &T::AccountId, balance: &BalanceOf<T>) {
				let id = Self::id_from_index(index);
				who.using_encoded(|b| child::put(&id, b, &balance));
			}

			/// Lookup a contribution in the associated child trie.
			pub fn contribution_get(index: FundIndex, who: &T::AccountId) -> BalanceOf<T> {
				let id = Self::id_from_index(index);
				who.using_encoded(|b| child::get_or_default::<BalanceOf<T>>(&id, b))
			}

			/// Remove a contribution from an associated child trie.
			pub fn contribution_kill(index: FundIndex, who: &T::AccountId) {
				let id = Self::id_from_index(index);
				who.using_encoded(|b| child::kill(&id, b));
			}

			/// Remove the entire record of contributions in the associated child trie in a single
			/// storage write.
			pub fn crowdfund_kill(index: FundIndex) {
				let id = Self::id_from_index(index);
				// The None here means we aren't setting a limit to how many keys to delete.
				// Limiting can be useful, but is beyond the scope of this recipe. For more info, see
				// https://crates.parity.io/frame_support/storage/child/fn.kill_storage.html
				child::kill_storage(&id, None);
			}
	}
}