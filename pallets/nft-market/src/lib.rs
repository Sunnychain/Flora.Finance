#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure,
	traits::{Currency, Get, ReservableCurrency},
};
use primitives::{Balance, TokenId};
use pallet_nft;

use sp_runtime::{traits::{One,AtLeast32BitUnsigned}, RuntimeDebug};
use sp_std::prelude::*;

pub use pallet::*;
// Tests disabled
type BalanceOf<T> =
	<<T as pallet_nft::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type CollectionId = u64;

pub type SalesId = u64;

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum NftType {
	NonFungibleToken,
	MultiToken,
}

/// Collection info
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct Collection<AccountId> {
	/// Class owner
	pub owner: AccountId,
	// The type of nft
	pub nft_type: NftType,
	/// The account of nft
	pub nft_account: AccountId,
	/// Metadata from ipfs
	pub metadata: Vec<u8>,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct Sale<AccountId,CollectionId,NonFungibleTokenId,TokenId,BalanceOf>{
	//Product Owner
	pub owner:AccountId,
	//Collection Index
	pub collection:CollectionId,
	//NFT Index
	pub nft_id:NonFungibleTokenId,
	//Token Index
	pub token_id: TokenId,
	//Product Price
	pub price : BalanceOf,

}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct Bid<AccountId,CollectionId,NonFungibleTokenId,TokenId,BalanceOf>{
	//Bidder
	pub bidder:AccountId,
	//Collection Index
	pub collection:CollectionId,
	//NFT Index
	pub nft_id:NonFungibleTokenId,
	//Token Index
	pub token_id: TokenId,
	//Offer Amount
	pub offer : BalanceOf,

}


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_nft::Config{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The minimum balance to create collection
		#[pallet::constant]
		type CreateCollectionDeposit: Get<BalanceOf<Self>>;

		
		
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type Collections<T: Config> =
		StorageMap<_, Blake2_128Concat, CollectionId, Collection<T::AccountId>>;

	#[pallet::storage]
	#[pallet::getter(fn next_collection_id)]
	pub(super) type NextCollectionId<T: Config> = StorageValue<_, CollectionId, ValueQuery>;

	#[pallet::storage]
	pub(super) type SalesInfo<T: Config> =
		StorageDoubleMap<_, 
		Blake2_128Concat, T::NonFungibleTokenId,
		Blake2_128Concat, TokenId,
		Sale<T::AccountId,CollectionId,T::NonFungibleTokenId,TokenId,BalanceOf<T>>>;

		#[pallet::storage]
		pub(super) type Offers<T: Config> =
			StorageDoubleMap<_, 
			Blake2_128Concat, T::NonFungibleTokenId,
			Blake2_128Concat, TokenId,
			Bid<T::AccountId,CollectionId,T::NonFungibleTokenId,TokenId,BalanceOf<T>>>;
	


	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CollectionCreated(CollectionId, T::AccountId),
		CollectionDestroyed(CollectionId, T::AccountId),
		SalesAdded(T::AccountId,T::NonFungibleTokenId,TokenId),
		SalesRemoved(T::AccountId,T::NonFungibleTokenId,TokenId),
		SalesUpdated(T::AccountId,T::NonFungibleTokenId,TokenId),
		OfferMade(T::AccountId,T::NonFungibleTokenId,TokenId,BalanceOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NumOverflow,
		NoAvailableCollectionId,
		CollectionNotFound,
		ListingNotFound,
		NoAvailableAssetId,
		AssetNotFound,
		InvalidQuantity,
		NoPermission,
		CannotDestroyCollection,
		NoAvaiableSalesId,
		AssetIsLocked,
		BidderIsOwner,
		OfferTooLow,
		HighestBidderAlready,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_collection(
			origin: OriginFor<T>,
			nft_type: NftType,

			nft_account: T::AccountId,
			metadata: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_create_collection(&who, nft_type, &nft_account, metadata)?;
			
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn add_sale(
			origin: OriginFor<T>,
			non_fungible_id : T::NonFungibleTokenId,
			collection_id: CollectionId,
			token_id: TokenId,
			price: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_add_sale(&who,non_fungible_id,collection_id, token_id, price)?;

			

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn remove_sale(
			origin: OriginFor<T>,
			non_fungible_id : T::NonFungibleTokenId,
			collection_id: CollectionId,
			token_id: TokenId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_remove_sale(&who, non_fungible_id, token_id)?;

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn update_price(
			origin: OriginFor<T>,
			nft_id: T::NonFungibleTokenId,
			token_id: TokenId,
			price: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_update_price(&who, nft_id, token_id, price)?;

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn offer(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			nft_id: T::NonFungibleTokenId,
			token_id: TokenId,
			offer: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_offer(&who, collection_id,nft_id, token_id,offer)?;

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn accept_offer(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			token_id: TokenId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_accept_offer(&who, collection_id, token_id)?;

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
	}
}

impl<T: Config> Pallet<T> {
	pub fn do_create_collection(
		who: &T::AccountId,
		nft_type: NftType,
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
		T::Currency::reserve(who, deposit.clone())?;

		let collection = Collection {
			owner: who.clone(),
			nft_type,
			nft_account: nft_account.clone(),
			metadata,
		};

	
		Collections::<T>::insert(collection_id, collection);

		Self::deposit_event(Event::CollectionCreated(collection_id, who.clone()));
		Ok(collection_id)
	}

	pub fn do_add_sale(
		who: &T::AccountId,
		non_fungible_id : T::NonFungibleTokenId,
		collection_id: CollectionId,
		token_id: TokenId,
		price: BalanceOf<T>,

	) -> DispatchResult {

		//ensure collection exists
		ensure!(Collections::<T>::contains_key(collection_id),Error::<T>::CollectionNotFound);

		//ensure origin is owner of token
		ensure!(pallet_nft::Owners::<T>::get(non_fungible_id,token_id)==who.clone(),Error::<T>::NoPermission);

		//ensure token is not locked
		ensure!(pallet_nft::IsLocked::<T>::get(non_fungible_id,token_id)==0,Error::<T>::AssetIsLocked);



				
			
			let sale = Sale {
				owner: who.clone(),
				collection:collection_id,
				nft_id:non_fungible_id,
				token_id:token_id,
				price:price
			};

			//Lock nft
			pallet_nft::IsLocked::<T>::try_mutate(non_fungible_id,token_id, |lock_flag|->DispatchResult{
				
				*lock_flag=lock_flag.checked_add(1).ok_or(Error::<T>::NumOverflow)?;
				Ok(())

			})?;

			
			SalesInfo::<T>::insert(non_fungible_id,token_id, sale);
			Self::deposit_event(Event::SalesAdded(who.clone(),non_fungible_id,token_id));
			
		Ok(())
	}

	pub fn do_remove_sale(
		who: &T::AccountId,
		non_fungible_id: T::NonFungibleTokenId,
		token_id: TokenId
		
	) -> DispatchResult {
		
		SalesInfo::<T>::try_mutate_exists(non_fungible_id,token_id, |sale| -> DispatchResult {

			//ensure sale exists
			let sales_info = sale.take().ok_or(Error::<T>::ListingNotFound)?;

			//ensure sales owner is origin
			ensure!(sales_info.owner == *who, Error::<T>::NoPermission);

			//ensure token is locked
			ensure!(pallet_nft::IsLocked::<T>::get(sales_info.nft_id,sales_info.token_id)==1,Error::<T>::AssetIsLocked);
			//remove listing
			SalesInfo::<T>::remove(non_fungible_id, token_id);

			//unlock nft
			pallet_nft::IsLocked::<T>::try_mutate(non_fungible_id,token_id, |lock_flag|->DispatchResult{
				
				*lock_flag=lock_flag.checked_sub(1).ok_or(Error::<T>::NumOverflow)?;
				Ok(())

			})?;

			Self::deposit_event(Event::SalesRemoved(who.clone(),non_fungible_id,token_id));


			Ok(())
		})

	
	}

	pub fn do_update_price(
		who: &T::AccountId,
		non_fungible_id: T::NonFungibleTokenId,
		token_id: TokenId,
		new_price: BalanceOf<T>,
	) -> DispatchResult {
		SalesInfo::<T>::try_mutate_exists(non_fungible_id,token_id, |sale| -> DispatchResult {

			//ensure sale exists
			let sales_info = sale.as_mut().ok_or(Error::<T>::ListingNotFound)?;

			//ensure sales owner is origin
			ensure!(sales_info.owner == *who, Error::<T>::NoPermission);

			//update price
			sales_info.price=new_price;

			
			Self::deposit_event(Event::SalesRemoved(who.clone(),non_fungible_id,token_id));

			Ok(())
		});

		Ok(())
	}

	pub fn do_offer(
		 who: &T::AccountId,
		 collection:CollectionId,
		 non_fungible_id: T::NonFungibleTokenId,
		 token_id: TokenId,
		 offer: BalanceOf<T>
	) -> DispatchResult {
		SalesInfo::<T>::try_mutate_exists(non_fungible_id,token_id, |sale| -> DispatchResult {

			//ensure sale exists
			let sales_info = sale.as_mut().ok_or(Error::<T>::ListingNotFound)?;

			//ensure bidder is not owner
			ensure!(who.clone()!=sales_info.owner,Error::<T>::BidderIsOwner);

			//ensure offer is greater or equal to price
			ensure!(offer.clone()>=sales_info.price,Error::<T>::OfferTooLow);

			Offers::<T>::try_mutate(non_fungible_id,token_id,|bid|-> DispatchResult{

				let cur_bid = Bid{
					bidder:who.clone(),
					collection:collection.clone(),
					nft_id:non_fungible_id.clone(),
					token_id: token_id.clone(),
					offer : offer.clone(),

				};	
						
						
				
				if !bid.is_none(){
					let mut last_bid = bid.as_mut().ok_or(Error::<T>::ListingNotFound)?;
					//ensure bidder is not the last best current bidder
					ensure!(who.clone()!=last_bid.bidder,Error::<T>::HighestBidderAlready);

					//ensure bid is greater than best current bid
					ensure!(offer.clone()>last_bid.offer,Error::<T>::OfferTooLow);

					//reserve currency
					T::Currency::reserve(who, offer)?;

					//unreserve last bidder currency
					T::Currency::unreserve(&last_bid.bidder,last_bid.offer);

					//update bid in storage
					*bid=Some(cur_bid);
				
	
				}else{
	
					//reserve currency
					T::Currency::reserve(who, offer)?;

					//update bid in storage
					*bid=Some(cur_bid);

				}
	
				Self::deposit_event(Event::OfferMade(who.clone(),non_fungible_id,token_id,offer));
				Ok(())
			})

	
			
		
	})
		
	
	}

	pub fn do_accept_offer(
		_who: &T::AccountId,
		_collection_id: CollectionId,
		_token_id: TokenId,
	) -> DispatchResult {
		Ok(())
	}

	pub fn do_destroy_collection(
		who: &T::AccountId,
		collection_id: CollectionId,
	) -> DispatchResult {
		Collections::<T>::try_mutate_exists(collection_id, |collection| -> DispatchResult {
			let c = collection.take().ok_or(Error::<T>::CollectionNotFound)?;
			ensure!(c.owner == *who, Error::<T>::NoPermission);

			let deposit = T::CreateCollectionDeposit::get();
			T::Currency::unreserve(who, deposit);

			Self::deposit_event(Event::CollectionDestroyed(collection_id, who.clone()));

			Ok(())
		})
	}
}