#![cfg_attr(not(feature = "std"), no_std)]

//! # DAO Module
//!
//! Organizes People with a common Vision to work on projects.
//! This module works as an extension to the Task module since 
//! it enables the creation of large projects which collect many tasks.
//! 
//! A visionary user is able to propose a Vision for the future. 
//! Within the vision, a specified Road-map is create that is broken 
//! down into tasks. Thus a DAO is a collection of tasks who are undertaken 
//! by people that believe in the vision of the Founder. 
//! 
//! Users support a Vision by signing a vision document. Signing a vision document
//! enrolls users in DAO where they will be able to create/fulfill tasks in 
//! support of the overall vision. For completion of tasks, users are rewarded tokens
//! and increased reputation.
//!  
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
		sp_runtime::traits::Hash};
	use sp_std::vec::Vec;
	use scale_info::TypeInfo;

	// Account used in Dao Struct
	type AccountOf<T> = <T as frame_system::Config>::AccountId;

	// Struct for holding Dao information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Dao<T: Config> {
		pub name: Vec<u8>,
		pub owner: AccountOf<T>,
		pub vision: Vec<u8>,
		pub members: Vec<AccountOf<T>>,  // vector of AccountIDs
		pub tasks: Vec<u8>   //TODO add reference to tasks pallet	
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn vision)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	// Store Vision document in StorageMap as Vector with value: AccountID, BlockNumber
	pub(super) type Vision<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

	

	// #[pallet::storage]
	// #[pallet::getter(fn organizations)]
	// // Create organization storage map identified by HashID and contains DAO Struct
	// pub(super) type Organizations<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Dao<T>>;

	#[pallet::storage]
	#[pallet::getter(fn organization)]
	// Create organization storage map with key: name and value: Vec<AccountID>
	pub(super) type Organization<T: Config> = StorageMap<_, Twox64Concat, Vec<u8>, Vec<T::AccountId>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Vision successfully created [AccountID, Vec]
		VisionCreated(T::AccountId, Vec<u8>),

		/// Vision removed [AccountID, Vec]
		VisionRemoved(T::AccountId, Vec<u8>),

		/// Vision signed [AccountID, Vec]
		VisionSigned(T::AccountId, Vec<u8>),

		/// DAO Organization was created [AccountID, DAO Name]
		OrganizationCreated(T::AccountId, Vec<u8>)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// The vision has already been created.
		VisionAlreadyExists,
		/// The Vision doesn't exist
		NoSuchVision,
		/// You are not the owner of the vision.
		NotVisionOwner,
		/// No rights to remove. Only creator can remove an organization
		NotOrganizationCreator,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// A dispatchable function for creating a vision and publishing it on chain
		/// The vision is signed by submitter and uses current block.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn create_vision(origin: OriginFor<T>, vision_document: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;
			// Verify that the specified vision has not already been created.
			ensure!(!Vision::<T>::contains_key(&vision_document), Error::<T>::VisionAlreadyExists);
			// Get the block number from the FRAME System pallet.
			let current_block = <frame_system::Pallet<T>>::block_number();
			// Store the vision with the sender and block number.
			Vision::<T>::insert(&vision_document, (&sender, current_block));
			// Emit an event that the claim was created.
			Self::deposit_event(Event::VisionCreated(sender, vision_document));
			Ok(())
		}

		#[pallet::weight(10_000)]
        pub fn remove_vision(origin: OriginFor<T>, vision_document: Vec<u8>) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/v3/runtime/origins
            let sender = ensure_signed(origin)?;
            // Verify that the specified vision has been created.
            ensure!(Vision::<T>::contains_key(&vision_document), Error::<T>::NoSuchVision);
            // Get owner of the vision.
            let (owner, _) = Vision::<T>::get(&vision_document);
            // Verify that sender of the current call is the vision creator
            ensure!(sender == owner, Error::<T>::NotVisionOwner);
            // Remove vision from storage.
            Vision::<T>::remove(&vision_document);
            // Emit an event that the vision was erased.
            Self::deposit_event(Event::VisionRemoved(sender, vision_document));
            Ok(())
        }

		// TODO implement logic
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn sign_vision(origin: OriginFor<T>, vision_document: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			let _hash_vision = T::Hashing::hash_of(&vision_document);


			// Update storage.
			// <DaoMembers<T>>::insert(who, hash_vision);

			// Emit an event.
			Self::deposit_event(Event::VisionSigned(who, vision_document));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_organization(origin: OriginFor<T>, org_name: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			//TODO: Ensure only visionary can crate DAOs

			// call public function to create org
			Self::new_org(&who, &org_name)?;

			// Emit an event.
			Self::deposit_event(Event::OrganizationCreated(who, org_name));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn dissolve_organization(origin: OriginFor<T>, org_name: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			//TODO: Ensure only visionary can crate DAOs

			// call public function to create org
			let dao_id = Self::remove_org(&who, &org_name)?;

			// Emit an event.
			// Self::deposit_event(Event::OrganizationCreated(who, dao_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		
	}

	// *** Helper functions *** //
	impl<T:Config> Pallet<T> {
		pub fn new_org(from_initiator: &T::AccountId, org_name: &Vec<u8>) -> Result<(), Error<T>> {
			
			let mut org = <Pallet<T>>::organization(&org_name);
			org.push(from_initiator.clone());

			// TODO: Implement proper logic
			// let mut add_members = Vec::new();
			// add_members.push(from_initiator.clone());

			// let new_dao = Dao::<T> {
			// 	name: org_name,
			// 	owner: from_initiator.clone(),
			// 	vision: Vec::new(),
			// 	members: add_members,
			// 	tasks: Vec::new(),
			// };

			// let dao_id = T::Hashing::hash_of(&new_dao);

			// Insert task into Hashmap
			<Organization<T>>::insert(org_name, org);

			// Increase task count
			// let new_count = Self::task_count().checked_add(1).ok_or(<Error<T>>::TaskCountOverflow)?;
			// <TaskCount<T>>::put(new_count);

			Ok(())
		}

		pub fn remove_org(from_initiator: &T::AccountId, org_name: &Vec<u8>) -> Result<(), Error<T>> {
			// check if initator is a member
			// TODO: Check if its actual creator
			let org = Self::organization(&org_name);
			ensure!(org.contains(&from_initiator), Error::<T>::NotOrganizationCreator);

			// Remove organizational instance
			<Organization<T>>::remove(org_name);

			Ok(())
		}


	}
}