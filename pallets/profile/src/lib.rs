
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

pub type RankLevel=u64;
use pallet_utils::{Pallet as Utils, WhoAndWhen, Content};





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
	pub trait Config: frame_system::Config + pallet_utils::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		//type AfterProfileUpdated: Self::AfterProfileUpdated;

		
	}


	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
	pub struct SocialAccount<T: Config> {
		pub followers_count: u32,
		pub following_accounts_count: u16,
		pub following_spaces_count: u16,
		pub reputation: u32,
		pub profile: Option<Profile<T>>,
	}



	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
	pub struct Profile<T: Config> {
		pub created: WhoAndWhen<T>,
		pub updated: Option<WhoAndWhen<T>>,
		pub content: Content
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
	pub struct ProfileHistoryRecord<T: Config> {
		pub edited: WhoAndWhen<T>,
		pub old_data: ProfileUpdate,
	}


	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
	pub struct ProfileUpdate {
		pub content: Option<Content>,
	}

	impl<T: Config> SocialAccount<T> {
		#[allow(clippy::comparison_chain)]
		pub fn change_reputation(&mut self, diff: i16) {
			if diff > 0 {
				self.reputation = self.reputation.saturating_add(diff.abs() as u32);
			} else if diff < 0 {
				self.reputation = self.reputation.saturating_sub(diff.abs() as u32);
			}
		}
	}



	impl Default for ProfileUpdate {
		fn default() -> Self {
			ProfileUpdate {
				content: None
			}
		}
	}

	impl<T: Config> ProfileHistoryRecord<T> {
		fn new(updated_by: T::AccountId, old_data: ProfileUpdate) -> Self {
			ProfileHistoryRecord {
				edited: WhoAndWhen::<T>::new(updated_by),
				old_data
			}
		}
	}

	

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn social_account_by_id)]
	pub(super) type SocialAccountById<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Option<SocialAccount<T>>,ValueQuery>;

		
	#[pallet::storage]
	#[pallet::getter(fn edit_history)]
	pub(super) type EditHistory<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<ProfileHistoryRecord<T>>,ValueQuery>;



	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProfileCreated(T::AccountId),
        ProfileUpdated(T::AccountId),
	
	}

	#[pallet::error]
	pub enum Error<T> {
	/// Social account was not found by id.
	SocialAccountNotFound,
	/// Profile is already created for this account.
	ProfileAlreadyCreated,
	/// Nothing to update in a profile.
	NoUpdatesForProfile,
	/// Account has no profile yet.
	AccountHasNoProfile,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_profile(
			origin:OriginFor<T>, content: Content
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;

			Utils::<T>::is_valid_content(content.clone())?;
	  
			let mut social_account = Self::get_or_new_social_account(owner.clone());
			ensure!(social_account.profile.is_none(), Error::<T>::ProfileAlreadyCreated);
	  
			social_account.profile = Some(
			  Profile {
				created: WhoAndWhen::<T>::new(owner.clone()),
				updated: None,
				content
			  }
			);
			<SocialAccountById<T>>::insert(owner.clone(), Some(social_account));
	  
			Self::deposit_event(Event::ProfileCreated(owner));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn update_profile(
			origin:OriginFor<T>, update: ProfileUpdate
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;

			let has_updates = update.content.is_some();

			ensure!(has_updates, Error::<T>::NoUpdatesForProfile);

			let mut social_account = Self::social_account_by_id(owner.clone()).ok_or(Error::<T>::SocialAccountNotFound)?;
			let mut profile = social_account.profile.ok_or(Error::<T>::AccountHasNoProfile)?;
			let mut is_update_applied = false;
			let mut old_data = ProfileUpdate::default();

			if let Some(content) = update.content {
				if content != profile.content {
				Utils::<T>::is_valid_content(content.clone())?;
				old_data.content = Some(profile.content);
				profile.content = content;
				is_update_applied = true;
				}
			}

			if is_update_applied {
				profile.updated = Some(WhoAndWhen::<T>::new(owner.clone()));
				social_account.profile = Some(profile.clone());

				<SocialAccountById<T>>::insert(owner.clone(), Some(social_account));
				Self::after_profile_updated(owner.clone(), &profile, old_data);

				Self::deposit_event(Event::ProfileUpdated(owner));
			}
			Ok(())
		}
        
			
	
}


impl <T: Config> SocialAccount<T> {
    pub fn inc_followers(&mut self) {
        self.followers_count = self.followers_count.saturating_add(1);
    }

    pub fn dec_followers(&mut self) {
        self.followers_count = self.followers_count.saturating_sub(1);
    }

    pub fn inc_following_accounts(&mut self) {
        self.following_accounts_count = self.following_accounts_count.saturating_add(1);
    }

    pub fn dec_following_accounts(&mut self) {
        self.following_accounts_count = self.following_accounts_count.saturating_sub(1);
    }

    pub fn inc_following_spaces(&mut self) {
        self.following_spaces_count = self.following_spaces_count.saturating_add(1);
    }

    pub fn dec_following_spaces(&mut self) {
        self.following_spaces_count = self.following_spaces_count.saturating_sub(1);
    }
}




impl<T: Config> Pallet<T> {

   
	pub fn get_or_new_social_account(account: T::AccountId) -> SocialAccount<T> {
        Self::social_account_by_id(account).unwrap_or(
            SocialAccount {
                followers_count: 0,
                following_accounts_count: 0,
                following_spaces_count: 0,
                reputation: 1,
                profile: None,
            }
        )
    }

	fn after_profile_updated(sender: T::AccountId, _profile: &Profile<T>, old_data: ProfileUpdate) {
		<EditHistory<T>>::mutate(sender.clone(), |ids|
			ids.push(ProfileHistoryRecord::<T>::new(sender, old_data)));
	}
	

	}

	

}