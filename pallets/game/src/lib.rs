
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
pub struct Game<AccountId,BlockNumber,BoundedString> {
	player_1: AccountId,
    player_2: AccountId,
    start_block: BlockNumber,
    end_block:BlockNumber,
	game_room_uri: BoundedString,
    winner: u64, // 1 - Player 1 // 2 - Player 2
    game_log:BoundedString
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
		type GameRoomIndex: Member  + Parameter + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;

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
	pub(super) type Games<T: Config> =
		StorageMap<_, Blake2_128Concat, T::GameRoomIndex, Game<T::AccountId, T::BlockNumber,BoundedVec<u8, T::StringLimit>>>;

	#[pallet::storage]
	#[pallet::getter(fn next_token_id)]
	pub(super) type GameRoomId<T: Config> = StorageValue<_, T::GameRoomIndex, ValueQuery>;



	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		GameSessionCreated(T::GameRoomIndex, T::AccountId),
	
	}

	#[pallet::error]
	pub enum Error<T> {
		NoAvailableGameRoomIndex,
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
		pub fn create_game(
			origin: OriginFor<T>,
			player_1: T::AccountId,
            player_2: T::AccountId,
            start_block: T::BlockNumber,

		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_create_game(&who,player_1, player_2,  start_block)?;
			
			Ok(().into())

    		}
      
			
	
}

impl<T: Config> Pallet<T> {

    pub fn do_create_game(
		who: &T::AccountId,
		player_1: T::AccountId,
        player_2: T::AccountId,
        start_block: T::BlockNumber
	) -> Result<T::GameRoomIndex, DispatchError> {
			
		
		let id = GameRoomId::<T>::try_mutate(|id| -> Result<T::GameRoomIndex, DispatchError> {
			let current_id = *id;
			*id = id.checked_add(&One::one()).ok_or(Error::<T>::NoAvailableGameRoomIndex)?;
			Ok(current_id)
		})?;

        
		
		let bounded_empty: BoundedVec<u8, T::StringLimit> =
        Vec::<u8>::new().try_into().map_err(|_| Error::<T>::BadMetadata)?;

		let token = Game {
            player_1: player_1,
            player_2: player_2,
            start_block:  start_block,
            end_block:0u32.into(),
            game_room_uri: bounded_empty.clone(),
            winner: 0, // 1 - Player 1 // 2 - Player 2
            game_log:bounded_empty
    
		};

        
		Games::<T>::insert(id, token);

		

		Self::deposit_event(Event::GameSessionCreated(id, who.clone()));

		Ok(id)
	}
	
  


	}

	
}