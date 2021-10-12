
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
pub struct Tournament<AccountId,BalanceOf,RankLevel,BoundedString> {
	owner: AccountId,
    vault: AccountId,
    min_entry: BalanceOf,
    creator_deposit:BalanceOf,
	min_rank:RankLevel,
    max_rank: RankLevel,
	name: BoundedString,
	base_uri: BoundedString,
    banned_cards: BoundedString,
}


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Identifier for the class of tournament.
		type TournamentId: Member  + Parameter + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;

		/// The minimum balance to create a tournament
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
	pub(super) type Tournaments<T: Config> =
		StorageMap<_, Blake2_128Concat, T::TournamentId, Tournament<T::AccountId,BalanceOf<T>, RankLevel,BoundedVec<u8, T::StringLimit>>>;
        

	#[pallet::storage]
	#[pallet::getter(fn next_token_id)]
	pub(super) type NextTournamentId<T: Config> = StorageValue<_, T::TournamentId, ValueQuery>;



	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TournamentCreated(T::TournamentId, T::AccountId),
	
	}

	#[pallet::error]
	pub enum Error<T> {
		NoAvailableTournamentId,
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
		pub fn create_tournament(
			origin: OriginFor<T>,
            vault:T::AccountId,
            min_entry: BalanceOf<T>,
            creator_deposit: BalanceOf<T>,
            min_rank: RankLevel,
            max_rank: RankLevel,
			name: Vec<u8>,
			base_uri: Vec<u8>,
            banned_cards:Vec<u8>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

           
			Self::do_create_tournament(who, vault,min_entry,creator_deposit,min_rank,max_rank,name, base_uri,banned_cards)?;
			
			Ok(().into())
		}
      
			
	
}

impl<T: Config> Pallet<T> {

    pub fn do_create_tournament(
		who: T::AccountId,
		vault:T::AccountId,
        min_entry: BalanceOf<T>,
        creator_deposit: BalanceOf<T>,
        min_rank: RankLevel,
        max_rank: RankLevel,
        name: Vec<u8>,
        base_uri: Vec<u8>,
        banned_cards:Vec<u8>
	) -> Result<T::TournamentId, DispatchError> {


        let deposit = T::CreateTokenDeposit::get();
		T::Currency::reserve(&who, deposit.clone())?;

		let bounded_name: BoundedVec<u8, T::StringLimit> =
			name.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;
		
		let bounded_base_uri: BoundedVec<u8, T::StringLimit> =
			base_uri.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;

        let bounded_ban_cards: BoundedVec<u8, T::StringLimit> =
			banned_cards.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;

		let id = NextTournamentId::<T>::try_mutate(|id| -> Result<T::TournamentId, DispatchError> {
			let current_id = *id;
			*id = id.checked_add(&One::one()).ok_or(Error::<T>::NoAvailableTournamentId)?;
			Ok(current_id)
		})?;

		let tournament = Tournament {
			owner: who.clone(),
            vault: vault,
            min_entry: min_entry,
            creator_deposit:creator_deposit,
            min_rank:min_rank,
            max_rank: max_rank,
            name: bounded_name,
            base_uri: bounded_base_uri,
            banned_cards: bounded_ban_cards
     		
        };

		Tournaments::<T>::insert(id, tournament);

		

		Self::deposit_event(Event::TournamentCreated(id, who.clone()));

		Ok(id)
	

		
	}
	
   


	}

	
}