#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, 
		traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons}
	};
	use frame_system::pallet_prelude::*;
	use frame_system::ensure_signed;

	// The LockIdentifier constant.
	const EXAMPLE_ID: LockIdentifier = *b"example ";

	// The custom BalanceOf type.
	type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The lockable currency type.
		type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Lockable currency can emit three event types.
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> 
	{
		/// Balance was locked successfully.
		Locked(<T as frame_system::Config>::AccountId, BalanceOf<T>),
		/// Lock was extended successfully.
		ExtendedLock(<T as frame_system::Config>::AccountId, BalanceOf<T>),
		/// Balance was unlocked successfully.
		Unlocked(<T as frame_system::Config>::AccountId),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn lock_capital(
			origin: OriginFor<T>,
			#[pallet::compact] amount: BalanceOf<T>
		) -> DispatchResultWithPostInfo {
			
			let user = ensure_signed(origin)?;
		
			T::Currency::set_lock(
				EXAMPLE_ID,
				&user,
				amount,
				WithdrawReasons::all(),
			);

			Self::deposit_event(Event::Locked(user, amount));
			Ok(().into())
		}
	
		#[pallet::weight(1_000)]
		pub fn extend_lock(
			origin: OriginFor<T>,
			#[pallet::compact] amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;
			
			T::Currency::extend_lock(
				EXAMPLE_ID,
				&user,
				amount,
				WithdrawReasons::all(),
			);

			Self::deposit_event(Event::ExtendedLock(user, amount));
			Ok(().into())
		}

		#[pallet::weight(1_000)]
		pub fn unlock_all(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			T::Currency::remove_lock(EXAMPLE_ID, &user);

			Self::deposit_event(Event::Unlocked(user));
			Ok(().into())
		}
	}
}