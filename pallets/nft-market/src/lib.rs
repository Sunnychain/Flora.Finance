#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure,
	traits::{Currency, Get, ReservableCurrency,ExistenceRequirement::KeepAlive},
};


use sp_runtime::{traits::{One,AtLeast32BitUnsigned}, RuntimeDebug};
use sp_std::prelude::*;

pub use pallet::*;

use pallet_nft;

type BalanceOf<T> =
	<<T as pallet_nft::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct Sale<AccountId,NonFungibleTokenId,BalanceOf,BoundedString>{
	//Product Owner
	pub owner:AccountId,
	
	//NFT Index
	pub nft_id:NonFungibleTokenId,
	//Product Price
	pub price : BalanceOf,

	pub tree_name:BoundedString,
	pub tree_description:BoundedString,

}


#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct Auction<AccountId,NonFungibleTokenId,BalanceOf,BlockNumber>{
	//Product Owner
	pub owner:AccountId,
	//NFT Index
	pub nft_id:NonFungibleTokenId,

	//Product Price
	pub current_price : BalanceOf,
	//Number of bids
	pub num_bid : u64,
	//Last Bidder account
	pub last_bidder : AccountId,
	// Auction end block
	pub end_block:BlockNumber,
}



#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct Bid<AccountId,NonFungibleTokenId,BalanceOf>{
	//Bidder
	pub bidder:AccountId,
	
	//NFT Index
	pub nft_id:NonFungibleTokenId,
	
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

		
		type MinimumAuctionAliveTime: Get<Self::BlockNumber>;

		
		
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	
	#[pallet::storage]
	pub(super) type SalesInfo<T: Config> =
		StorageMap<_, 
		Blake2_128Concat, T::NonFungibleTokenId,
		Sale<T::AccountId,T::NonFungibleTokenId,BalanceOf<T>,BoundedVec<u8,T::StringLimit>>>;

	#[pallet::storage]
		pub(super) type AuctionsInfo<T: Config> =
			StorageMap<_, 
			Blake2_128Concat, T::NonFungibleTokenId,
			Auction<T::AccountId,T::NonFungibleTokenId,BalanceOf<T>,T::BlockNumber>>;

	#[pallet::storage]
	#[pallet::getter(fn auction_end_time)]
	/// Index auctions by end time.
	pub type AuctionEndTime<T: Config> =
		StorageDoubleMap<_,
		Blake2_128Concat, 
		T::BlockNumber,
		Blake2_128Concat, 
		T::NonFungibleTokenId,
		()
		>;


		#[pallet::storage]
		pub(super) type Offers<T: Config> =
			StorageMap<_, 
			Blake2_128Concat, T::NonFungibleTokenId,
			Bid<T::AccountId,T::NonFungibleTokenId,BalanceOf<T>>>;
	


	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SalesAdded(T::AccountId,T::NonFungibleTokenId),
		SalesRemoved(T::AccountId,T::NonFungibleTokenId),
		SalesUpdated(T::AccountId,T::NonFungibleTokenId),
		OfferMade(T::AccountId,T::NonFungibleTokenId,BalanceOf<T>),
		OfferAccepted(T::AccountId,T::AccountId,T::NonFungibleTokenId,BalanceOf<T>),
		AuctionCreated(T::AccountId,T::NonFungibleTokenId,T::BlockNumber,BalanceOf<T>),
		BiddedInAuction(T::AccountId,T::NonFungibleTokenId,BalanceOf<T>),

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NumOverflow,
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
		AuctionDurationTooLow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		fn on_finalize(now: T::BlockNumber) {
			Self::conclude_auction(now);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		

		#[pallet::weight(10_000)]
		pub fn add_sale(
			origin: OriginFor<T>,
			non_fungible_id : T::NonFungibleTokenId,
			price: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_add_sale(&who,non_fungible_id, price)?;

			

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn remove_sale(
			origin: OriginFor<T>,
			non_fungible_id : T::NonFungibleTokenId,
			) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_remove_sale(&who, non_fungible_id)?;

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn update_price(
			origin: OriginFor<T>,
			nft_id: T::NonFungibleTokenId,
			price: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_update_price(&who, nft_id, price)?;

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn offer(
			origin: OriginFor<T>,
			nft_id: T::NonFungibleTokenId,
			offer: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_offer(&who,nft_id,offer)?;

			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn bid_auction(
			origin: OriginFor<T>,
			nft_id: T::NonFungibleTokenId,
			offer: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_bid_auction(&who,nft_id, offer)?;

			Ok(().into())
		}

		

		#[pallet::weight(10_000)]
		pub fn create_auction(
			origin: OriginFor<T>,
			non_fungible_id : T::NonFungibleTokenId,
			minimum_price: BalanceOf<T>,
			auction_end_block:T::BlockNumber
		) -> DispatchResult {
			
			let who = ensure_signed(origin)?;

			Self::do_create_auction(&who,non_fungible_id, minimum_price,auction_end_block)?;


			Ok(())
		}

		
	}
}

impl<T: Config> Pallet<T> {
	fn conclude_auction(now: T::BlockNumber)->DispatchResult {
		for (non_fungible_id,_) in <AuctionEndTime<T>>::drain_prefix(&now) {
			
		
			//unlock nft
			pallet_nft::Pallet::<T>::unlock_nft(non_fungible_id);

			
			AuctionsInfo::<T>::try_mutate_exists(non_fungible_id,  |auction|->DispatchResult{
			//Ensure auction exists and remove it from listing
					
				let auction_info = auction.take().ok_or(Error::<T>::ListingNotFound)?;

				//verificar se existe um vencedor 
				if auction_info.last_bidder!=auction_info.owner && auction_info.num_bid>0 {
					//unreserve last bidder currency
					<T as pallet_nft::Config>::Currency::unreserve(&auction_info.last_bidder,auction_info.current_price);
					//Transfer currency to nft owner
					<T as pallet_nft::Config>::Currency::transfer(&auction_info.last_bidder,&auction_info.owner,auction_info.current_price,KeepAlive)?;
					//transfer nft to bidder
					pallet_nft::Pallet::<T>::do_transfer_from(non_fungible_id, &auction_info.owner, &auction_info.last_bidder)?;

				};

				Ok(())

			})?;


			
			
		}
		Ok(())
		
	}

	

	pub fn do_add_sale(
		who: &T::AccountId,
		non_fungible_id : T::NonFungibleTokenId,
		price: BalanceOf<T>,

	) -> DispatchResult {

		
		//ensure origin is owner of token
		ensure!(pallet_nft::Owners::<T>::get(non_fungible_id)==Some(who.clone()),Error::<T>::NoPermission);

		
			
			let token_cp=pallet_nft::Tokens::<T>::get(non_fungible_id).ok_or(Error::<T>::ListingNotFound)?.clone();

			//ensure token is not locked
			ensure!(pallet_nft::IsLocked::<T>::get(non_fungible_id)==0,Error::<T>::AssetIsLocked);
				
			let sale = Sale {
				owner: who.clone(),
				nft_id:non_fungible_id,
				price:price,
				tree_name: token_cp.tree_name.clone(),
				tree_description: token_cp.tree_description.clone()
			};		
			

			//Lock nft
			pallet_nft::Pallet::<T>::lock_nft(non_fungible_id);

			
			SalesInfo::<T>::insert(non_fungible_id, sale);
			Self::deposit_event(Event::SalesAdded(who.clone(),non_fungible_id));
				
			Ok(())


		
	}

	pub fn do_remove_sale(
		who: &T::AccountId,
		non_fungible_id: T::NonFungibleTokenId,
		
		
	) -> DispatchResult {
		
		SalesInfo::<T>::try_mutate_exists(non_fungible_id, |sale| -> DispatchResult {

			//ensure sale exists
			let sales_info = sale.take().ok_or(Error::<T>::ListingNotFound)?;

			//ensure sales owner is origin
			ensure!(sales_info.owner == *who, Error::<T>::NoPermission);

			//ensure token is locked
			ensure!(pallet_nft::IsLocked::<T>::get(sales_info.nft_id)==1,Error::<T>::AssetIsLocked);
			//remove listing
			SalesInfo::<T>::remove(non_fungible_id);

			//unlock nft
			pallet_nft::Pallet::<T>::unlock_nft(non_fungible_id);

			Self::deposit_event(Event::SalesRemoved(who.clone(),non_fungible_id));


			Ok(())
		})

	
	}

	pub fn do_update_price(
		who: &T::AccountId,
		non_fungible_id: T::NonFungibleTokenId,
		new_price: BalanceOf<T>,
	) -> DispatchResult {
		SalesInfo::<T>::try_mutate_exists(non_fungible_id,|sale| -> DispatchResult {

			//ensure sale exists
			let sales_info = sale.as_mut().ok_or(Error::<T>::ListingNotFound)?;

			//ensure sales owner is origin
			ensure!(sales_info.owner == *who, Error::<T>::NoPermission);

			//update price
			sales_info.price=new_price;

			
			Self::deposit_event(Event::SalesRemoved(who.clone(),non_fungible_id));

			Ok(())
		})

	}

	pub fn do_create_auction(
		who: &T::AccountId,
		non_fungible_id: T::NonFungibleTokenId,
		minimum_price: BalanceOf<T>,
		auction_end_block:T::BlockNumber
	) -> DispatchResult {

		//ensure origin is owner of token
		ensure!(pallet_nft::Owners::<T>::get(non_fungible_id)==Some(who.clone()),Error::<T>::NoPermission);

		//ensure token is not locked
		ensure!(pallet_nft::IsLocked::<T>::get(non_fungible_id)==0,Error::<T>::AssetIsLocked);


		//Ensure duration is greater than minimun
		ensure!((auction_end_block - frame_system::Pallet::<T>::block_number())>=T::MinimumAuctionAliveTime::get(),Error::<T>::AuctionDurationTooLow);

		let auction = Auction {
				owner: who.clone(),
				nft_id:non_fungible_id,
				current_price:minimum_price,
				num_bid : 0u64.into(),
				last_bidder : who.clone(), // Converter para nulo
				end_block:auction_end_block
			};

			
			pallet_nft::IsLocked::<T>::try_mutate(non_fungible_id, |lock_flag|->DispatchResult{
				
				*lock_flag=lock_flag.checked_add(1).ok_or(Error::<T>::NumOverflow)?;
				
				
				AuctionsInfo::<T>::insert(non_fungible_id, auction);
				AuctionEndTime::<T>::insert(auction_end_block,non_fungible_id,());
				Self::deposit_event(Event::AuctionCreated(who.clone(),non_fungible_id,auction_end_block,minimum_price));
				Ok(())

			})

					
			
		
	}
	pub fn do_offer(
		 who: &T::AccountId,
		 non_fungible_id: T::NonFungibleTokenId,
		 offer: BalanceOf<T>
	) -> DispatchResult {
		SalesInfo::<T>::try_mutate_exists(non_fungible_id, |sale| -> DispatchResult {

			//ensure sale exists and remove from SalesInfo
			let sales_info = sale.take().ok_or(Error::<T>::ListingNotFound)?;

			//ensure bidder is not owner
			ensure!(who.clone()!=sales_info.owner,Error::<T>::BidderIsOwner);

			//ensure offer is greater or equal to price
			ensure!(offer.clone()>=sales_info.price,Error::<T>::OfferTooLow);

			//transfer currency from bidder to origin
			<T as pallet_nft::Config>::Currency::transfer(who,&sales_info.owner,offer,KeepAlive)?;

				
			//unlock nft
			pallet_nft::Pallet::<T>::unlock_nft(non_fungible_id);
			

			//send nft to bidder
			ensure!(pallet_nft::Pallet::<T>::do_transfer_from(non_fungible_id,&sales_info.owner , who)==Ok(()),Error::<T>::InvalidQuantity);

			Self::deposit_event(Event::OfferAccepted(sales_info.owner,who.clone(),non_fungible_id,offer));
			
			Ok(())
		
		
		})
				
	}


	pub fn do_bid_auction(
		who: &T::AccountId,
		non_fungible_id: T::NonFungibleTokenId,
		offer: BalanceOf<T>
   ) -> DispatchResult {
	   AuctionsInfo::<T>::try_mutate_exists(non_fungible_id, |auction| -> DispatchResult {

		   //ensure auction exists
		   let auction_info = auction.as_mut().ok_or(Error::<T>::ListingNotFound)?;

		   //ensure bidder is not owner
		   ensure!(who.clone()!=auction_info.owner,Error::<T>::BidderIsOwner);

		   //ensure offer is greater to current price
		   ensure!(offer.clone()>auction_info.current_price,Error::<T>::OfferTooLow);

		   
			if auction_info.last_bidder!=auction_info.owner && auction_info.num_bid > 0 {
				//unreserve last bidder currency
				<T as pallet_nft::Config>::Currency::unreserve(&auction_info.last_bidder,auction_info.current_price);
				
				
			};
			//reserve new bidder currency
			<T as pallet_nft::Config>::Currency::reserve(who, offer)?;
			//update bid in storage
			//update price
			auction_info.current_price=offer;
			//update num_bid
			auction_info.num_bid=auction_info.num_bid.checked_add(1).ok_or(Error::<T>::NumOverflow)?;
			//update last bidder
			auction_info.last_bidder = who.clone();

			  
		 Self::deposit_event(Event::BiddedInAuction(who.clone(),non_fungible_id,offer));
		 Ok(())
	   })
			   
   }



}