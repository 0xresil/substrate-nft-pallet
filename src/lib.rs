#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

pub mod nft;
use sp_runtime::traits::{StaticLookup};


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum offchain data length.
		#[pallet::constant]
		type NFTOffchainDataLimit: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Counter for NFT ids.
	#[pallet::storage]
	#[pallet::getter(fn next_nft_id)]
	pub type NextNFTId<T: Config> = StorageValue<_, crate::nft::NFTId, ValueQuery>;

	/// Data related to NFTs.
	#[pallet::storage]
	#[pallet::getter(fn nfts)]
	pub type Nfts<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		crate::nft::NFTId,
		crate::nft::NFTData<T::AccountId, T::NFTOffchainDataLimit>,
		OptionQuery,
	>;

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// No NFT was found with that NFT id.
		NFTNotFound,
		/// This function can only be called by the owner of the NFT.
		NotTheNFTOwner,
		/// Operation is not allowed because the NFT is owned by the caller.
		CannotTransferNFTsToYourself,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Mint a new NFT with the provided details. An ID will be auto
		/// generated and logged as an event, The caller of this function
		/// will become the owner of the new NFT.
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn mint(
			origin: OriginFor<T>,
			offchain_data: crate::nft::U8BoundedVec<T::NFTOffchainDataLimit>,
		) -> DispatchResultWithPostInfo {
			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn burn(origin: OriginFor<T>, nft_id: crate::nft::NFTId) -> DispatchResultWithPostInfo {
			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn transfer(
			origin: OriginFor<T>,
			nft_id: crate::nft::NFTId,
			recipient: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResultWithPostInfo {
			Ok(().into())
		}

    #[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn approve(
			origin: OriginFor<T>,
			nft_id: crate::nft::NFTId,
			delegate: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResultWithPostInfo {
			Ok(().into())
		}

    #[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn create_collection(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
			Ok(().into())
		}

    #[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn freeze(
			origin: OriginFor<T>,
			nft_id: crate::nft::NFTId
		) -> DispatchResultWithPostInfo {
			Ok(().into())
		}

    #[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn thaw(
			origin: OriginFor<T>,
			nft_id: crate::nft::NFTId
		) -> DispatchResultWithPostInfo {
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn get_next_nft_id() -> crate::nft::NFTId {
			let nft_id = NextNFTId::<T>::get();
			let next_id = nft_id
				.checked_add(1)
				.expect("If u32 is not enough we should crash for safety; qed.");
			NextNFTId::<T>::put(next_id);	
			nft_id
		}
	}
}
