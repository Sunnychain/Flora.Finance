
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure,
	traits::{Currency, Get, ReservableCurrency},
	PalletId, BoundedVec,
};

use sp_runtime::{
	traits::{AtLeast32BitUnsigned, One, CheckedAdd},
	RuntimeDebug,
};
use sp_std::{convert::TryInto, prelude::*};


use pallet_utils::{Pallet as Utils, WhoAndWhen, Content};

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug)]
pub enum ScoringAction {
	CreateComment,
    ShareComment,
	CreateRoom,
	WinGame,
    WatchGame,
	WatchPayGame,
    FollowAccount,
	AcquireCommomNFT,
	AcquireUncommomNFT,
	AcquireRareNFT,
	AcquireEpicNFT,
	AcquireLendaryNFT,
	
}

impl Default for ScoringAction {
    fn default() -> Self {
        ScoringAction::FollowAccount
    }
}






#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		
		 // The overarching event type.
		 type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		 // Weights of the social actions
		 type FollowSpaceActionWeight: Get<i16>;
		 type FollowAccountActionWeight: Get<i16>;
	 
		 type SharePostActionWeight: Get<i16>;
		 type UpvotePostActionWeight: Get<i16>;
		 type DownvotePostActionWeight: Get<i16>;
	 
		 type CreateCommentActionWeight: Get<i16>;
		 type ShareCommentActionWeight: Get<i16>;
		 type UpvoteCommentActionWeight: Get<i16>;
		 type DownvoteCommentActionWeight: Get<i16>;
		
	}


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);



	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AccountReputationChanged(AccountId, ScoringAction, u32),
	
	}

	#[pallet::error]
	pub enum Error<T> {
		// Scored account reputation difference by account and action not found.
		ReputationDiffNotFound,
		// Post extension is a comment.
		NotRootPost,
		// Post extension is not a comment.
		NotComment,
				
	
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
			
	
}






impl<T: Config> Pallet<T> {

   


}

}