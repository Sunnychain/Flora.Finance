#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure,
	traits::{Currency, Get, ReservableCurrency},
	PalletId, BoundedVec,
};
use sp_runtime::{RuntimeDebug, traits::{AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, One}};
use sp_std::{convert::TryInto, prelude::*};
pub use pallet::*;
// Tests disabled

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen)]
pub struct GameRoom<AccountId,GameState,BoundedString>{
	 player_1:AccountId,
	 player_2:AccountId,
	 game_state : GameState,
	 connection:BoundedString,
	 room_password:BoundedString,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen)]
pub enum GameState {
	WaitingConnections = 0,
	OnGoing = 1,
	FinishedPlayer1Wins = 2,
	FinishedPlayer2wins = 3,
	FinishedDraw=4,
}

type BalanceOf<T> =
<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


#[frame_support::pallet]
	pub mod pallet {
		use super::*;
		use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
		use frame_system::pallet_prelude::*;

		#[pallet::config]
		pub trait Config: frame_system::Config {
			type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

			/// Identifier for the class of token.
			type GameRoomIndex: Member  + Parameter + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;
			type PlayingFee: Get<BalanceOf<Self>>;
			type StringLimit: Get<u32>;

			type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		}

		#[pallet::pallet]
		#[pallet::generate_store(pub(super) trait Store)]
		pub struct Pallet<T>(_);



		#[pallet::storage]
		pub(super) type Games<T: Config> =
			StorageMap<_, Blake2_128Concat, T::GameRoomIndex, GameRoom<T::AccountId,GameState,BoundedVec<u8, T::StringLimit>>>;

		#[pallet::storage]
		#[pallet::getter(fn next_token_id)]
		pub(super) type NextGameRoomId<T: Config> = StorageValue<_, T::GameRoomIndex, ValueQuery>;

		


		#[pallet::event]
		#[pallet::metadata(T::AccountId = "AccountId")]
		#[pallet::generate_deposit(pub(super) fn deposit_event)]
		pub enum Event<T: Config> {
			GameCreated(T::GameRoomIndex,T::AccountId)
			
		
		}

		#[pallet::error]
		pub enum Error<T> {
			NoAvailableGameRoomIndex,
			BadMetadata,
		
		}

		#[pallet::hooks]
		impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

		#[pallet::call]
		impl<T: Config> Pallet<T> {
			#[pallet::weight(10_000)]
			pub fn create_game(
				origin: OriginFor<T>,
				player_1:T::AccountId,
				player_2:T::AccountId,
				connect:Vec<u8>,
			) -> DispatchResult {
				let who = ensure_signed(origin)?;

				Self::do_create_game(&who, player_1, player_2, connect)?;
				
				Ok(().into())
			}
			
		
		
	}





		
		impl<T: Config> Pallet<T> {
			pub fn do_create_game(
				who: &T::AccountId,
				player_1:  T::AccountId,
				player_2: T::AccountId,
				connect: Vec<u8>,
			) -> Result<T::GameRoomIndex, DispatchError> {
					let game_id =
						NextGameRoomId::<T>::try_mutate(|id| -> Result<T::GameRoomIndex, DispatchError> {
							let current_id = *id;
							*id = id
								.checked_add(&One::one())
								.ok_or(Error::<T>::NoAvailableGameRoomIndex)?;
							Ok(current_id)
						})?;

					let bounded_connect:  BoundedVec<u8, T::StringLimit> =
					connect.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;

					let test_net_password : Vec<u8>="testnet_password".into();


					let bounded_password: BoundedVec<u8, T::StringLimit> =
					test_net_password.try_into().map_err(|_| Error::<T>::BadMetadata)?;

					
					let game = GameRoom {
						player_1:player_1,
						player_2:player_2,
						game_state : GameState::WaitingConnections,
						connection: bounded_connect,
						room_password:bounded_password,
					};

				
					Games::<T>::insert(game_id.clone(), game);

					Self::deposit_event(Event::GameCreated(game_id, who.clone()));
					Ok(game_id)
				} 
			

			}

}