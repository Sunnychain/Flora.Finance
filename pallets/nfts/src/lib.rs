
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure,
	traits::{Currency, Get, ReservableCurrency,Randomness},
	PalletId, BoundedVec,
};

use frame_support::log;

use sp_runtime::{RuntimeDebug, traits::{AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, One,Zero}};
use sp_std::{convert::TryInto, prelude::*};

use pallet_utils;

use pallet_scores;

pub use pallet::*;

use sp_core::H256;

pub type CollectionId = u64;

use scale_info::TypeInfo;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen,TypeInfo)]
pub struct Token<AccountId,NonFungibleTokenId,NftType,CollectionId,NFTStatus,BoundedString> {
	pub	owner: AccountId,
	pub token_id:NonFungibleTokenId,
	pub land_owner:AccountId,
	pub nft_type: NftType,
	pub collection:CollectionId,
	pub status:NFTStatus,
	pub tree_name:BoundedString,
	pub tree_description:BoundedString,
	pub forest_type_flag:BoundedString,
	pub land_owner_contract:BoundedString,
	pub land_owner_insurance_contract:BoundedString,
	pub gps_land_coordiates:BoundedString,
	pub name: BoundedString,
	pub symbol: BoundedString,
	pub base_uri: BoundedString,
	pub total_trees:u32,
	pub co2_offset_per_year:u32,
	
}


#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen,TypeInfo)]
pub struct GameToken<AccountId> {
	owner: AccountId,
	mana_cost:u8,
	stats:u8,
	effect:u8,
	rarity:u8,
	eval:u32,
}



#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen,TypeInfo)]
pub enum NFTStatus {
	Suspended = 0,
	AprovedNonAudited = 1,
	AprovedAudited = 2,
	Dead = 3,
	Sick=4,
	Recovered=5,
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug,TypeInfo)]
pub enum NftType {
	CarbonCommon=0,
	CarbonUncommom=1,
	CarbonRare=2,
	CarbonEpic=3,
	CarbonLendary=4,
	CarbonZeroCommon=5,
	CarbonZeroUncommom=6,
	CarbonZeroRare=7,
	CarbonZeroEpic=8,
	CarbonZeroLendary=9,
}

/// Collection info
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug,TypeInfo)]
pub struct Collection<AccountId> {
	/// Class owner
	pub owner: AccountId,
	/// The account of nft
	pub nft_account: AccountId,
	/// Metadata from ipfs
	pub metadata: Vec<u8>,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	use sp_runtime::traits::AccountIdConversion;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_scores::pallet::Config{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type PalletId: Get<PalletId>;

		/// Identifier for the class of token.
		type NonFungibleTokenId: Member  + Parameter + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;

		type Randomness: Randomness<H256,u32>;

		/// The maximum length of base uri stored on-chain.
		#[pallet::constant]
		type StringLimit: Get<u32>;

		/// The minimum balance to create token
		#[pallet::constant]
		type CreateTokenDeposit: Get<BalanceOf<Self>>;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		/// The minimum balance to create collection
		#[pallet::constant]
		type CreateCollectionDeposit: Get<BalanceOf<Self>>;

		type CarbonZeroId : Get<u64>;

		type CarbonZeroRareId : Get<u64>;

		type CarbonZeroEpicId : Get<u64>;

		type CarbonZeroLegendaryId : Get<u64>;

		
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type Tokens<T: Config> =
		StorageMap<_, Blake2_128Concat, T::NonFungibleTokenId, Token<T::AccountId, T::NonFungibleTokenId,NftType,CollectionId,NFTStatus,BoundedVec<u8, T::StringLimit>>>;

	#[pallet::storage]
	#[pallet::getter(fn next_token_id)]
	pub(super) type NextTokenId<T: Config> = StorageValue<_, T::NonFungibleTokenId, ValueQuery>;

	#[pallet::storage]
	pub type Collections<T: Config> =
		StorageMap<_, Blake2_128Concat, CollectionId, Collection<T::AccountId>>;

	#[pallet::storage]
	#[pallet::getter(fn next_collection_id)]
	pub(super) type NextCollectionId<T: Config> = StorageValue<_, CollectionId, ValueQuery>;

	#[pallet::storage]
	pub (super) type GameTokens<T: Config> =
		StorageMap<_, Blake2_128Concat, T::NonFungibleTokenId,
		GameToken<T::AccountId>>;


	#[pallet::storage]
	#[pallet::getter(fn owner_of)]
	pub type Owners<T: Config> =
		StorageMap<_, Blake2_128Concat,  T::NonFungibleTokenId, T::AccountId>;


	/// The NftMaster Account similar to treasury vault
    #[pallet::storage]
    #[pallet::getter(fn nft_master)]
    pub type NftMaster<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	#[pallet::storage]
	pub type IsLocked<T: Config> =
		StorageMap<_, Blake2_128Concat, T::NonFungibleTokenId,u32,ValueQuery>;

	// The Nonce storage item.
    #[pallet::storage]
    #[pallet::getter(fn get_nonce)]
    pub type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_owned_tokens)]
	pub type OwnedTokens<T: Config> =
		StorageDoubleMap<_, 
		Blake2_128Concat, T::AccountId,
		Blake2_128Concat, T::NonFungibleTokenId,
		Token<T::AccountId,T::NonFungibleTokenId,NftType,CollectionId,NFTStatus,BoundedVec<u8, T::StringLimit>>>;


	

    #[pallet::genesis_config]
    pub struct GenesisConfig<T:Config>{
		pub nft_master: T::AccountId,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			Self {
				nft_master: Default::default(),
			}
		}
	}

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
               // create a NTF class
			let treasury_acc = self.nft_master.clone();

			NftMaster::<T>::put(treasury_acc.clone());
		
			let col_id = Pallet::<T>::do_create_collection(
				&treasury_acc.clone(), 
	 			&treasury_acc.clone(), 
				"flora.finance/collections".into()
			).unwrap();
							
			
        }
        
    }


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TokenCreated(T::NonFungibleTokenId, T::AccountId),
		Transfer(T::NonFungibleTokenId, T::AccountId, T::AccountId),
		Approval(T::NonFungibleTokenId, T::AccountId, T::AccountId),
		ApprovalForAll(T::NonFungibleTokenId, T::AccountId, T::AccountId, bool),
		CollectionCreated(CollectionId, T::AccountId),
		CollectionDestroyed(CollectionId, T::AccountId),
		GameTokenCreated(T::AccountId,T::NonFungibleTokenId),
		GameTokenBurnt(T::AccountId,T::NonFungibleTokenId),
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
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		

	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {


		#[pallet::weight(10_000)]
		pub fn create_collection(
			origin: OriginFor<T>,
			nft_account: T::AccountId,
			metadata: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_create_collection(&who,  &nft_account, metadata)?;
			
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn destroy_collection(
			origin: OriginFor<T>,
			collection_id: CollectionId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_destroy_collection(&who, collection_id)?;

			Ok(().into())
		}


		#[pallet::weight(10_000)]
		pub fn create_token(
			origin: OriginFor<T>,
			land_owner:T::AccountId,
			nft_type:NftType,
			collection:CollectionId,
			tree_name: Vec<u8>,
			tree_description: Vec<u8>,
			forest_type_flag:Vec<u8>,
			land_owner_contract:Vec<u8>,
			land_owner_insurance_contract:Vec<u8>,
			gps_coordinates:Vec<u8>,
			name: Vec<u8>,
			symbol: Vec<u8>,
			base_uri: Vec<u8>,
			total_trees:u32,
			co2_offset:u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_create_token(&who, &land_owner,nft_type,collection,tree_name,tree_description,forest_type_flag,
				land_owner_contract,land_owner_insurance_contract,gps_coordinates,name, symbol,
			    base_uri,total_trees,co2_offset)?;

			Ok(())
		}

		

		#[pallet::weight(10_000)]
		pub fn transfer_from(
			origin: OriginFor<T>,
			id: T::NonFungibleTokenId,
			from: T::AccountId,
			to: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;


			ensure!(
				Self::owner_of(id)==Some(who),
				Error::<T>::NotOwnerOrApproved
			);

			Self::do_transfer_from(id, &from, &to)?;

			Ok(())
		}

		/*
		#[pallet::weight(10_000)]
		pub fn mint_batch(
			origin: OriginFor<T>,
			id: T::NonFungibleTokenId,
			to: T::AccountId,
			token_id: TokenId,
			num_of_token:u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Self::has_permission(id, &who), Error::<T>::NoPermission);

			Self::do_mint_batch(id, &to,num_of_token)?;

			Ok(())
		}*/

		#[pallet::weight(10_000)]
		pub fn burn(
			origin: OriginFor<T>,
			id: T::NonFungibleTokenId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_burn(id, &who)?;

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn burn_game_token(
			origin: OriginFor<T>,
			id: T::NonFungibleTokenId,
			
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_burn_game_token(id, &who)?;

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn mint_game_token(
			origin: OriginFor<T>,
			id: T::NonFungibleTokenId,
			
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Tokens::<T>::try_mutate_exists(id,|token|->DispatchResult{

				let mut aux=token.as_mut().ok_or(Error::<T>::TokenNonExistent)?;

				Self::do_mint_game_token(who, id,aux.nft_type)?;

				Ok(())

			});

					

			

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Returns the `AccountId` of the treasury account.
	pub fn treasury_account() -> T::AccountId {
			AccountIdConversion::into_account(&<T as pallet::Config>::PalletId::get())
	}

	pub fn do_create_collection(
		who: &T::AccountId,
		nft_account: &T::AccountId,
		metadata: Vec<u8>,
	) -> Result<CollectionId, DispatchError> {
		let collection_id =
			NextCollectionId::<T>::try_mutate(|id| -> Result<CollectionId, DispatchError> {
				let current_id = *id;
				*id = id
					.checked_add(One::one())
					.ok_or(Error::<T>::NoAvailableCollectionId)?;
				Ok(current_id)
			})?;

		let deposit = T::CreateCollectionDeposit::get();
		<T as pallet::Config>::Currency::reserve(who, deposit.clone())?;

		let collection = Collection {
			owner: who.clone(),
			nft_account: nft_account.clone(),
			metadata,
		};

	
		Collections::<T>::insert(collection_id, collection);

		Self::deposit_event(Event::CollectionCreated(collection_id, who.clone()));
		Ok(collection_id)
	} 

	pub fn do_destroy_collection(
		who: &T::AccountId,
		collection_id: CollectionId,
	) -> DispatchResult {
		Collections::<T>::try_mutate_exists(collection_id, |collection| -> DispatchResult {
			let c = collection.take().ok_or(Error::<T>::CollectionNotFound)?;
			ensure!(c.owner == *who, Error::<T>::NoPermission);

			let deposit = T::CreateCollectionDeposit::get();
			<T as pallet::Config>::Currency::unreserve(who, deposit);

			Self::deposit_event(Event::CollectionDestroyed(collection_id, who.clone()));

			Ok(())
		})
	}

	pub fn do_create_token(
		who: &T::AccountId,
		land_owner:&T::AccountId,
		nft_type:NftType,
		collection:CollectionId,
		tree_name: Vec<u8>,
		tree_description: Vec<u8>,
		forest_type_flag:Vec<u8>,
		land_owner_contract:Vec<u8>,
		land_owner_insurance_contract:Vec<u8>,
		gps_coordinates:Vec<u8>,
		name: Vec<u8>,
		symbol: Vec<u8>,
		base_uri: Vec<u8>,
		total_trees:u32,
		co2_offset:u32,
	) -> Result<T::NonFungibleTokenId, DispatchError> {
		let deposit = T::CreateTokenDeposit::get();
		<T as pallet::Config>::Currency::reserve(&who, deposit.clone())?;

		let bounded_name: BoundedVec<u8, T::StringLimit> =
			name.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;
		let bounded_symbol: BoundedVec<u8, T::StringLimit> =
			symbol.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;
		let bounded_base_uri: BoundedVec<u8, T::StringLimit> =
			base_uri.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;

		let bounded_tree_name: BoundedVec<u8, T::StringLimit> =
			tree_name.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;

		let bounded_tree_description: BoundedVec<u8, T::StringLimit> =
			tree_description.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;

		let bounded_forest_type_flag: BoundedVec<u8, T::StringLimit> =
			forest_type_flag.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;

		let bounded_land_owner_contract: BoundedVec<u8, T::StringLimit> =
			land_owner_contract.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;
		
		let bounded_land_owner_insurance_contract: BoundedVec<u8, T::StringLimit> =
			land_owner_insurance_contract.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;

		let bounded_gps: BoundedVec<u8, T::StringLimit> =
			gps_coordinates.clone().try_into().map_err(|_| Error::<T>::BadMetadata)?;

		ensure!(Collections::<T>::contains_key(collection),Error::<T>::InvalidId);

		let id = NextTokenId::<T>::try_mutate(|id| -> Result<T::NonFungibleTokenId, DispatchError> {
			let current_id = *id;
			*id = id.checked_add(&One::one()).ok_or(Error::<T>::NoAvailableTokenId)?;
			Ok(current_id)
		})?;

		let token = Token {
			owner: who.clone(),
			land_owner: land_owner.clone(),
			nft_type:nft_type,
			collection:collection.clone(),
			status:NFTStatus::AprovedNonAudited,
			tree_name:bounded_tree_name,
			tree_description:bounded_tree_description,
			forest_type_flag:bounded_forest_type_flag,
			land_owner_contract:bounded_land_owner_contract,
			land_owner_insurance_contract:bounded_land_owner_insurance_contract,
			gps_land_coordiates:bounded_gps,
			name: bounded_name,
			symbol: bounded_symbol,
			base_uri: bounded_base_uri,
			total_trees:total_trees,
			co2_offset_per_year:co2_offset,
			token_id:id.clone()
		};

		
		
		
		Tokens::<T>::insert(id.clone(), token.clone());
		OwnedTokens::<T>::insert(who.clone(),id.clone(),token.clone());
		Owners::<T>::insert(id.clone(),who.clone());
		IsLocked::<T>::insert(id.clone(),0);

		
		pallet_scores::pallet::Pallet::<T>::action_performed(who.clone(),pallet_scores::ScoringAction::CreateToken);

		
		
		Self::deposit_event(Event::TokenCreated(id, who.clone()));

		Ok(id)
	}

	pub fn do_transfer_from(
		id: T::NonFungibleTokenId,
		from: &T::AccountId,
		to: &T::AccountId,
		
	) -> DispatchResult {

		Tokens::<T>::try_mutate_exists(id, |token|->DispatchResult{
			let tok = token.as_mut().ok_or(Error::<T>::InvalidId)?;

			let owner = Self::owner_of(id).ok_or(Error::<T>::TokenNonExistent).unwrap();
			ensure!(
			owner != T::AccountId::default(),
			Error::<T>::TokenNonExistent
		);

		ensure!(tok.owner==owner,Error::<T>::NoPermission);
		ensure!(IsLocked::<T>::get(id)==0,Error::<T>::LockedAsset);


		ensure!(owner == *from, Error::<T>::NotTokenOwner);

		

		
		
		tok.owner=to.clone();
		Owners::<T>::insert(id.clone(),  to.clone());

		Owners::<T>::try_mutate(id,|own|-> DispatchResult{
			*own = Some(to.clone());
			Ok(())
		});

		OwnedTokens::<T>::remove(owner.clone(),id.clone());
		
		OwnedTokens::<T>::insert(to.clone(),id.clone(),tok.clone());

		Self::deposit_event(Event::Transfer(
			id.clone(),
			from.clone(),
			to.clone(),
		));

		Ok(())


		})

		
	}

/*
	pub fn do_mint_batch(
		id: T::NonFungibleTokenId,
		to: &T::AccountId,
		num_of_token:u32,
	) -> DispatchResult {

		let mut next_token_id=0;

		for n in 0 .. num_of_token{
			while Self::exists(id, next_token_id){
				next_token_id+=1;

			}
			
			Self::do_mint(id, &to, next_token_id)?;
		}

		Ok(())
	}
*/
	pub fn do_mint_game_token(
		who:T::AccountId,
		id: T::NonFungibleTokenId,
		nft_type: NftType
		
	) -> DispatchResult {
		let owner = Self::owner_of(id).ok_or(Error::<T>::TokenNonExistent).unwrap();
		
		ensure!(who.clone()==owner,Error::<T>::NoPermission);

		ensure!(IsLocked::<T>::get(id)==0,Error::<T>::LockedAsset);

		//Lock nft
		IsLocked::<T>::try_mutate(id, |lock_flag|->DispatchResult{
				
			*lock_flag=lock_flag.checked_add(1).ok_or(Error::<T>::Overflow)?;
			Ok(())

		})?;

	
		let game_token = Self::generate_game_token(owner.clone(),id.clone(),nft_type);

	
		GameTokens::<T>::insert(id,game_token);
		

		Self::deposit_event(Event::GameTokenCreated(
			who,
			id,
			
		));

		Ok(())
	}

	pub fn do_burn_game_token(
		id: T::NonFungibleTokenId,
		account: &T::AccountId,
		
	) -> DispatchResult {
		let owner = Self::owner_of(id).ok_or(Error::<T>::TokenNonExistent).unwrap();
		ensure!(*account == owner, Error::<T>::NotTokenOwner);

		ensure!(GameTokens::<T>::contains_key(id),Error::<T>::TokenNonExistent);

		GameTokens::<T>::remove(id);

		ensure!(Self::unlock_nft(id)==Ok(()),Error::<T>::Overflow);
		

		Self::deposit_event(Event::GameTokenBurnt(
			owner,
			id,
			
		));

		Ok(())
	}


	

	pub fn do_burn(
		id: T::NonFungibleTokenId,
		account: &T::AccountId,
		
	) -> DispatchResult {
		Tokens::<T>::try_mutate_exists(id, |token|->DispatchResult{
			let tok = token.take().ok_or(Error::<T>::InvalidId)?;

			let owner = Self::owner_of(id).ok_or(Error::<T>::TokenNonExistent).unwrap();
			ensure!(
			owner != T::AccountId::default(),
			Error::<T>::TokenNonExistent
		);

		ensure!(tok.owner==owner,Error::<T>::NoPermission);
		ensure!(IsLocked::<T>::get(id)==0,Error::<T>::LockedAsset);


				

		Owners::<T>::remove(id);
		OwnedTokens::<T>::remove(owner.clone(),id);
		

		Self::deposit_event(Event::GameTokenBurnt(
			owner,
			id,
			
		));

		Ok(())


		})
	}

	

	pub fn lock_nft(
		id: T::NonFungibleTokenId,
		
	) -> DispatchResult {
		//Lock nft
		IsLocked::<T>::try_mutate(id, |lock_flag|->DispatchResult{
			
			ensure!(*lock_flag == 0,Error::<T>::LockedAsset);

			*lock_flag=lock_flag.checked_add(1).ok_or(Error::<T>::Overflow)?;
			Ok(())

		})
			
	}

	pub fn unlock_nft(
		id: T::NonFungibleTokenId,
		
	) -> DispatchResult {
		//unLock nft
		IsLocked::<T>::try_mutate(id, |lock_flag|->DispatchResult{
			
			ensure!(*lock_flag == 1,Error::<T>::LockedAsset);

			*lock_flag=lock_flag.checked_sub(1).ok_or(Error::<T>::Overflow)?;
			Ok(())

		})
			
	}

	fn generate_game_token(
		owner:T::AccountId,
		id: T::NonFungibleTokenId,
		nft_type: NftType,
		)->GameToken<T::AccountId>{

			let mut rarity=0u8;

			let mut upper_bound=800u32;
			
			
			if nft_type == NftType::CarbonZeroUncommom || nft_type == NftType::CarbonUncommom{
				rarity=1;
				upper_bound = 1_300;

			}

			if nft_type == NftType::CarbonRare || nft_type == NftType::CarbonZeroRare{
				rarity=2;
				upper_bound = 1_800;

			}
			if nft_type == NftType::CarbonEpic || nft_type == NftType::CarbonZeroEpic{
				rarity=3;
				upper_bound = 2_300;

			}

			if nft_type == NftType::CarbonLendary || nft_type == NftType::CarbonZeroLendary{
				rarity=4;
				upper_bound = 2_800;

			}

			let mut aux_cost= 0u8;
			let mut aux_value=0u8;
			let mut aux_effect=0u8;

			let lower_bound= upper_bound-500;

			

			
			while !(Self::reward_func(aux_cost,aux_value,aux_effect)<upper_bound && Self::reward_func(aux_cost,aux_value,aux_effect)>lower_bound) {

				let rand= Self::generate_random();

				aux_cost = Self::scale_in_between(1,7,rand[10]);
				aux_value = Self::scale_in_between(1,7,rand[20]);
				aux_effect = Self::scale_in_between(1,33,rand[30]);

				log::info!("test cost {:?}", rand[10]);
				log::info!("test value {:?}", aux_value);
				log::info!("test effect {:?}", aux_effect);
				log::info!("reward{:?}", Self::reward_func(aux_cost,aux_value,aux_effect));

				

			};
			


			let game_token = GameToken{
				owner: owner.clone(),
				mana_cost:aux_cost,
				stats:aux_value,
				effect:aux_effect,
				rarity:rarity,
				eval:Self::reward_func(aux_cost,aux_value,aux_effect),
			};
	

			game_token

		}

		fn reward_func(cost:u8,value:u8,effect:u8)->u32{

			(value as u32*1_000/(cost as u32+1)) + (effect as u32*1_000/(effect as u32+1))
			
		}

		fn generate_random()->H256{

			let nonce = Self::get_and_increment_nonce();
            let randomValue = T::Randomness::random(&nonce);
			log::info!("random  {:?}",randomValue.0);
			randomValue.0
		}
	

		fn get_and_increment_nonce() -> Vec<u8> {
			let nonce = <Nonce<T>>::get();
			<Nonce<T>>::put(nonce.wrapping_add(1));
			nonce.encode()
		}

		fn scale_in_between(a:u8,b:u8,val:u8)-> u8 {
			/*
			(((b as u32-a as u32)*((val as u32*1000)/255)+a as u32*1000)/1000)*/

			(((b as u32-a as u32)*((val as u32*1000)/255)+a as u32*1000)/1000).try_into().unwrap()
		}

}