#![cfg_attr(not(feature = "std"), no_std)]

//! # Profile Module
//!
//! Each AccountID is able to create profiles that add specific metadata
//!  to their account. This metadata is used to enrich AccountID with additional
//!  properties such as reputation, interests, etc. 
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::{
		sp_runtime::traits::Hash, 
		traits::{Currency}};
	use scale_info::TypeInfo;
	use sp_std::vec::Vec;


	// Account, Balance are used in Profile Struct
	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;


	// Struct for holding Profile information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Profile<T: Config> {
		pub owner: AccountOf<T>,
		pub interests: Vec<u8>,
		pub balance: Option<BalanceOf<T>>,
		pub reputation: u32,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The Currency handler for the Profile pallet.
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn profiles)]
	/// Stores a Profiles's unique properties in a StorageMap.
	pub(super) type Profiles<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Profile<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Profile was successfully created. 
		ProfileCreated(T::AccountId, T::Hash),

		/// Profile was successfully deleted.
		ProfileDeleted(T::AccountId),

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// One Account can only create a single profile. 
		ProfileAlreadyCreated,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Dispatchable call that enables every new actor to create personal profile in storage.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_profile(origin: OriginFor<T>, interests: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let account = ensure_signed(origin)?;

			let profile_id = Self::generate_profile(&account, interests)?;
			log::info!("A profile is created with ID: {:?}.", profile_id); // TODO Remove loging

			// Emit an event.
			Self::deposit_event(Event::ProfileCreated(account, profile_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}


		/// Dispatchable call that enables every new actor to delete profile from storage.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_profile(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let account = ensure_signed(origin)?;

			Self::delete_profile(&account)?;

			// Emit an event.
			Self::deposit_event(Event::ProfileDeleted(account));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

	}

	// ** Helper internal functions ** //
	impl<T:Config> Pallet<T> {
		// Generates initial Profile.
		pub fn generate_profile(owner: &T::AccountId, interests_vec: Vec<u8>) -> Result<T::Hash, Error<T>> {
			
			// Get current balance of owner
			let balance = T::Currency::free_balance(owner);

			// Populate Profile struct
			let mut profile = Profile::<T> {
				owner: owner.clone(),
				interests: interests_vec,
				balance: Some(balance),
				reputation: 0,
			};

			// Get hash of profile
			let profile_id = T::Hashing::hash_of(&profile);

			// Change reputation
			profile.change_reputation();

			// Insert profile into HashMap
			<Profiles<T>>::insert(owner, profile);

			Ok(profile_id)
		}

		// Public function that deletes a user profile
		pub fn delete_profile(owner: &T::AccountId) -> Result<(), Error<T>> {
			<Profiles<T>>::remove(owner);
			Ok(())
		}
	}

	// Change the reputation on a Profile
	// TODO: Create better reputation function 
	impl<T:Config> Profile<T> {
		pub fn change_reputation(&mut self) {
			self.reputation += 1;
		}

		pub fn change_interests(&mut self, new_interests: Vec<u8>) {
			self.interests = new_interests;
		}
	} 

}