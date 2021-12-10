#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure,
	traits::{Currency, Get, ReservableCurrency,Time},
	PalletId, BoundedVec,
};

#[cfg(feature = "std")]
use serde::Deserialize;



use sp_runtime::{
	traits::{AtLeast32BitUnsigned, One, CheckedAdd},
	RuntimeDebug,
};
use sp_std::{collections::btree_set::BTreeSet,convert::TryInto, prelude::*};
use scale_info::TypeInfo;

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug,TypeInfo)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[cfg_attr(feature = "std", serde(tag = "contentType", content = "contentId"))]
pub enum Content {
    /// No content.
    None,
    /// A raw vector of bytes.
    Raw(Vec<u8>),
    /// IPFS CID v0 of content.
    #[allow(clippy::upper_case_acronyms)]
    IPFS(Vec<u8>),
    /// Hypercore protocol (former DAT) id of content.
    Hyper(Vec<u8>),
}

impl From<Content> for Vec<u8> {
    fn from(content: Content) -> Vec<u8> {
        match content {
            Content::None => Vec::<u8>::new(),
            Content::Raw(vec_u8) => vec_u8,
            Content::IPFS(vec_u8) => vec_u8,
            Content::Hyper(vec_u8) => vec_u8,
        }
    }
}

impl Default for Content {
    fn default() -> Self {
        Self::None
    }
}

impl Content {
    pub fn is_none(&self) -> bool {
        self == &Self::None
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn is_ipfs(&self) -> bool {
        matches!(self, Self::IPFS(_))
    }
}


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type PalletId: Get<PalletId>;

        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

	}

	pub type SpaceId = u64;

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug,TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct WhoAndWhen<T:Config> {
		pub account: T::AccountId,
		pub block: T::BlockNumber,
		pub time: T::Moment,
	}

	impl<T:Config> WhoAndWhen<T> {
		pub fn new(account: T::AccountId) -> Self {
			WhoAndWhen {
				account,
				block: frame_system::Pallet::<T>::block_number(),
				time: <pallet_timestamp::Pallet<T>>::now(),
			}
		}
	}

	#[derive(Encode, Decode, Ord, PartialOrd, Clone, Eq, PartialEq, RuntimeDebug)]
	pub enum User<AccountId> {
		Account(AccountId),
		Space(SpaceId),
	}

	impl<AccountId> User<AccountId> {
		pub fn maybe_account(self) -> Option<AccountId> {
			if let User::Account(account_id) = self {
				Some(account_id)
			} else {
				None
			}
		}
	}

	/// Treasury Account
    #[pallet::storage]
    #[pallet::getter(fn treasury_account)]
    pub type TreasuryAccount<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	
	#[pallet::genesis_config]
    pub struct GenesisConfig<T:Config>{
		pub treasury_account: T::AccountId,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			Self {
				treasury_account: Default::default(),
			}
		}
	}

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
               // create a NTF class
			let treasury_acc = self.treasury_account.clone();

			TreasuryAccount::<T>::put(treasury_acc.clone());
			
			
        }
        
    }


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TokenCreated(T::AccountId),
	
	}




type BalanceOf<T> =
<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

	#[pallet::error]
	pub enum Error<T> {
		 /// Account is blocked in a given space.
		 AccountIsBlocked,
		 /// Content is blocked in a given space.
		 ContentIsBlocked,
		 /// Post is blocked in a given space.
		 PostIsBlocked,
		 /// IPFS CID is invalid.
		 InvalidIpfsCid,
		 /// `Raw` content type is not yet supported.
		 RawContentTypeNotSupported,
		 /// `Hyper` content type is not yet supported.
		 HypercoreContentTypeNotSupported,
		 /// Space handle is too short.
		 HandleIsTooShort,
		 /// Space handle is too long.
		 HandleIsTooLong,
		 /// Space handle contains invalid characters.
		 HandleContainsInvalidChars,
		 /// Content type is `None`.
		 ContentIsEmpty,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	

impl<T: Config> Pallet<T> {

	   
	pub fn is_valid_content(content: Content) -> DispatchResult {
        match content {
            Content::None => Ok(()),
            Content::Raw(_) => Err(Error::<T>::RawContentTypeNotSupported.into()),
            Content::IPFS(ipfs_cid) => {
                let len = ipfs_cid.len();
                // IPFS CID v0 is 46 bytes.
                // IPFS CID v1 is 59 bytes.df-integration-tests/src/lib.rs:272:5
                ensure!(len == 46 || len == 59, Error::<T>::InvalidIpfsCid);
                Ok(())
            },
            Content::Hyper(_) => Err(Error::<T>::HypercoreContentTypeNotSupported.into())
        }
    }

	pub fn remove_from_vec<F: PartialEq>(vector: &mut Vec<F>, element: F) {
		if let Some(index) = vector.iter().position(|x| *x == element) {
			vector.swap_remove(index);
		}
	}

	pub fn bool_to_option(value: bool) -> Option<bool> {
		if value { Some(value) } else { None }
	}

	pub fn convert_users_vec_to_btree_set(
        users_vec: Vec<User<T::AccountId>>
    ) -> Result<BTreeSet<User<T::AccountId>>, DispatchError> {
        let mut users_set: BTreeSet<User<T::AccountId>> = BTreeSet::new();

        for user in users_vec.iter() {
            users_set.insert(user.clone());
        }

        Ok(users_set)
    }
}

	
}
