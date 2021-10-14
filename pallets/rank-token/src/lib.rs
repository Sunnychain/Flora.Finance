
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure,
	traits::{Currency, Get, ReservableCurrency},
	PalletId, BoundedVec,
};
use primitives::{TokenId, TokenIndex};
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, One, CheckedAdd},
	RuntimeDebug,
};
use sp_std::{convert::TryInto, prelude::*};

pub type RankLevel=u64;




type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub use pallet::*;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen)]
pub struct RankToken<AccountId,RankLevel,BoundedString,u64> {
	owner: AccountId,
	rank:RankLevel,
	name: BoundedString,
	base_uri: BoundedString,
    num_win:u64,
    num_loss:u64
}


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Identifier for the class of rank token.
		type RankTokenId: Member  + Parameter + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;

		/// The minimum balance to create token
		#[pallet::constant]
		type CreateTokenDeposit: Get<BalanceOf<Self>>;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// The maximum length of base uri stored on-chain.
		#[pallet::constant]
		type StringLimit: Get<u32>;

		
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type RankTokens<T: Config> =
		StorageMap<_, Blake2_128Concat, T::RankTokenId, RankToken<T::AccountId, RankLevel,BoundedVec<u8, T::StringLimit>,u64>>;

	#[pallet::storage]
	#[pallet::getter(fn next_token_id)]
	pub(super) type NextRankTokenId<T: Config> = StorageValue<_, T::RankTokenId, ValueQuery>;



	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RankTokenCreated(T::RankTokenId, T::AccountId),
	
	}

	#[pallet::error]
	pub enum Error<T> {
		NoAvailableTokenId,
		Overflow,
		Underflow,
		TokenAlreadyMinted,
		InvalidId,
		NoPermission,
		NotTokenOwner,
		TokenNonExistent,
		ApproveToCurrentOwner,
		NotOwnerOrApproved,
		ApproveToCaller,
		BadMetadata,
		LockedAsset,
		NoAvailableCollectionId,
		CollectionNotFound,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_ranked_token(
			origin: OriginFor<T>,
			name: Vec<u8>,
			base_uri: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_create_ranked_token(&who, name,  base_uri)?;
			
			Ok(().into())
		}
        #[pallet::weight(10_000)]
		pub fn update_ranked_token(
			origin: OriginFor<T>,
			name: Vec<u8>,
			base_uri: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_update_ranked_token(&who, name,  base_uri)?;
			
			Ok(().into())
		}
			
	
}

impl<T: Config> Pallet<T> {

    pub fn do_create_ranked_token(
		who: &T::AccountId,
		name: Vec<u8>,
		base_uri: Vec<u8>,
	) -> Result<T::RankTokenId, DispatchError> {
		let deposit = T::CreateTokenDeposit::get();
		T::Currency::reserve(&who, deposit.clone())?;

		let bounded_name: BoundedVec<u8, T::StringLimit> =
			name.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;
		
		let bounded_base_uri: BoundedVec<u8, T::StringLimit> =
			base_uri.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;

		let id = NextRankTokenId::<T>::try_mutate(|id| -> Result<T::RankTokenId, DispatchError> {
			let current_id = *id;
			*id = id.checked_add(&One::one()).ok_or(Error::<T>::NoAvailableTokenId)?;
			Ok(current_id)
		})?;

		let token = RankToken {
			owner: who.clone(),
			rank:0u64.into(),
            name: bounded_name,
			base_uri: bounded_base_uri,
            num_win:0u64.into(),
            num_loss:0u64.into()

     
		};

		RankTokens::<T>::insert(id, token);

		

		Self::deposit_event(Event::RankTokenCreated(id, who.clone()));

		Ok(id)
	}
	
    pub fn do_update_ranked_token(
		who: &T::AccountId,
		name: Vec<u8>,
		base_uri: Vec<u8>,
	) ->  DispatchError {
		
		Ok(()))
	}


	}

	
}