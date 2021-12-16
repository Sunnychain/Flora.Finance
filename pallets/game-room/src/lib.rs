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

use pallet_scores;
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
		pub trait Config: frame_system::Config + pallet_scores::pallet::Config {
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
			GameNotFound,
			InvalidPassword,
			BrokenGameState,
			InvalidUpdateGameState,

		
		}

		#[pallet::hooks]
		impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

		#[pallet::call]
		impl<T: Config> Pallet<T> {
			#[pallet::weight(10_000)]
			pub fn create_game(
				origin: OriginFor<T>,
				password:Vec<u8>,
			) -> DispatchResult {
				let who = ensure_signed(origin)?;

				Self::do_create_game(&who,password)?;
				
				Ok(().into())
			}

			#[pallet::weight(10_000)]
			pub fn update_game_status(
				origin: OriginFor<T>,
				game_id:T::GameRoomIndex,
				game_state: GameState,
				password: Vec<u8>,
			) -> DispatchResult {
				let who = ensure_signed(origin)?;

				Self::do_update_game(&who,game_id,game_state,password)?;
				
				Ok(().into())
			}

			#[pallet::weight(10_000)]
			pub fn join_game(
				origin: OriginFor<T>,
				game_id:T::GameRoomIndex,
				password: Vec<u8>,
			) -> DispatchResult {
				let who = ensure_signed(origin)?;

				Self::do_join_game(&who,game_id,password)?;
				
				Ok(().into())
			}
			
		
		
	}





		
		impl<T: Config> Pallet<T> {
			pub fn do_create_game(
				who: &T::AccountId,
				password: Vec<u8>,
			) -> Result<T::GameRoomIndex, DispatchError> {
					let game_id =
						NextGameRoomId::<T>::try_mutate(|id| -> Result<T::GameRoomIndex, DispatchError> {
							let current_id = *id;
							*id = id
								.checked_add(&One::one())
								.ok_or(Error::<T>::NoAvailableGameRoomIndex)?;
							Ok(current_id)
						})?;


					let connect : Vec<u8>="flora.finance/games/game-room/".into();
					let bounded_connect:  BoundedVec<u8, T::StringLimit> =
					connect.try_into().map_err(|_| Error::<T>::BadMetadata)?;

					let bounded_password: BoundedVec<u8, T::StringLimit> =
					password.try_into().map_err(|_| Error::<T>::BadMetadata)?;

					
					let game = GameRoom {
						player_1:who.clone(),
						player_2:who.clone(),
						game_state : GameState::WaitingConnections,
						connection: bounded_connect,
						room_password:bounded_password,
					};

					pallet_scores::pallet::Pallet::<T>::action_performed(who.clone(),pallet_scores::ScoringAction::CreateRoom)?;
					Games::<T>::insert(game_id.clone(), game);

					Self::deposit_event(Event::GameCreated(game_id, who.clone()));
					Ok(game_id)
				} 

			pub fn do_update_game(
				who: &T::AccountId,
				game_id:T::GameRoomIndex,
				game_state:GameState,
				password: Vec<u8>,
			) ->  Result<T::GameRoomIndex, DispatchError>  {
				Games::<T>::try_mutate_exists(game_id,|game|->Result<T::GameRoomIndex, DispatchError> {
					let game_mut=game.as_mut().ok_or(Error::<T>::GameNotFound)?;
					ensure!(game_mut.room_password==password,Error::<T>::InvalidPassword);
					ensure!((game_mut.game_state.clone() as u32) < (game_state.clone() as u32),Error::<T>::InvalidUpdateGameState);
					//ensure root acesss????????
					match game_state{
						GameState::FinishedPlayer1Wins=>pallet_scores::pallet::Pallet::<T>::action_performed(game_mut.player_1.clone(),pallet_scores::ScoringAction::WinGame).unwrap(),
						GameState::FinishedPlayer2wins=>pallet_scores::pallet::Pallet::<T>::action_performed(game_mut.player_2.clone(),pallet_scores::ScoringAction::WinGame).unwrap(),
						GameState::WaitingConnections =>  0,
						GameState::OnGoing=> 0 ,
						GameState::FinishedDraw => 0,
					};
					
					game_mut.game_state=game_state;			

					Ok(game_id)
				})

				
				} 
			
			
			
	
			pub fn do_join_game(
				who: &T::AccountId,
				game_id:T::GameRoomIndex,
				password:Vec<u8>,
			) -> Result<T::GameRoomIndex, DispatchError>  {

				Games::<T>::try_mutate_exists(game_id,|game|->Result<T::GameRoomIndex, DispatchError> {
					let game_mut=game.as_mut().ok_or(Error::<T>::GameNotFound)?;
					ensure!(game_mut.player_1==game_mut.player_2,Error::<T>::BrokenGameState);
					ensure!(game_mut.room_password==password,Error::<T>::InvalidPassword);
					ensure!(game_mut.game_state==GameState::WaitingConnections,Error::<T>::BrokenGameState);
					
					//ensure root acesss????????		
					game_mut.game_state=GameState::OnGoing;
					game_mut.player_2=who.clone();			

					Ok(game_id)
				})
			} 
			
				
			}
			

		}

		

