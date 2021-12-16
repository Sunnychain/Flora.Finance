
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure,
	PalletId, BoundedVec,
};

use sp_runtime::{
	
	RuntimeDebug,
};
use sp_std::{prelude::*};




#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug)]
pub enum ScoringAction {
	CreateToken=1,
	CreateRoom=2,
	WinGame=3,
    WatchGame=4,
	WatchPayGame=5,
    FollowAccount=6,
	AcquireCommomNFT=7,
	AcquireUncommomNFT=8,
	AcquireRareNFT=9,
	AcquireEpicNFT=10,
	AcquireLendaryNFT=11,
	
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
	pub trait Config: frame_system::Config + pallet_profile::Config{
		
		 // The overarching event type.
		 type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		 // Weights of the social actions
		/* 
		 type FollowSpaceActionWeight: Get<i16>;
		 type FollowAccountActionWeight: Get<i16>;
	 
		 type SharePostActionWeight: Get<i16>;
		 type UpvotePostActionWeight: Get<i16>;
		 type DownvotePostActionWeight: Get<i16>;
	 
		 type CreateCommentActionWeight: Get<i16>;
		 type ShareCommentActionWeight: Get<i16>;
		 type UpvoteCommentActionWeight: Get<i16>;
		 type DownvoteCommentActionWeight: Get<i16>;
		 */
		
	}


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);



	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AccountReputationChanged(T::AccountId, ScoringAction, u32),
	
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

	pub fn action_performed(
		acc: T::AccountId,
        action: ScoringAction,
    ) -> Result<u32, DispatchError>  {

		let mut social_account = pallet_profile::Pallet::<T>::get_or_new_social_account(acc.clone());
		
		

		//add reputation and mmr
		if action == ScoringAction::WinGame{
			social_account.mmr+=5;

		};
		
		social_account.reputation+=action as u32;
		pallet_profile::SocialAccountById::<T>::insert(acc.clone(), social_account.clone());


		Ok(action as u32)
      
    }

}

}