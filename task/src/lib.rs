#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
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
	//TODO: Better import
	use crate::TaskStatus::Created;
use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::{
		sp_runtime::traits::Hash};
	use scale_info::TypeInfo;
	use sp_std::vec::Vec;

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

	// Use AccountId from frame_system
	type AccountOf<T> = <T as frame_system::Config>::AccountId;

	  // Struct for holding Kitty information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Task<T: Config> {
		pub creator: AccountOf<T>,
		pub requirements: Vec<u8>,
		pub status: TaskStatus,
		pub budget: u32,
		pub owner: AccountOf<T>,
	}

	// Set Gender type in Kitty struct.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
  	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
  	pub enum TaskStatus {
    	Created,
    	InProgress,
		Closed,
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
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn task_count)]
	pub(super) type TaskCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn tasks)]
	// Store Tasks in a  Storage Map where key: hash, value: struct Task
	pub(super) type Tasks<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Task<T>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),

		/// Event for creation of task [AccountID, hash id]
		TaskCreated(T::AccountId, T::Hash),

		/// Task assigned to new account [AccountID, hash id]
		TaskAssigned(T::AccountId, T::Hash),

		/// Task completed by assigned account [AccountID, hash id]
		TaskCompleted(T::AccountId, T::Hash),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Reached maximum number of tasks.
		TaskCountOverflow,
		/// The given task doesn't exists. Try again
		TaskNotExist,
		/// Only the creator of task has the rights to remove task
		OnlyCreatorClosesTask
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}


		/// An dispatchable call that creates tasks.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_task(origin: OriginFor<T>, requirements: Vec<u8>, budget: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let signer = ensure_signed(origin)?;

			// Update storage.
			let task_id = Self::new_task(&signer, requirements, budget)?;
			// TODO: Check if user has balance to create task

			// Emit a Task Created Event.
			Self::deposit_event(Event::TaskCreated(signer,task_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An dispatchable call that starts a task by assigning to new account.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn start_task(origin: OriginFor<T>, task_id: T::Hash) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let signer = ensure_signed(origin)?;

			// Assign task and update storage.
			Self::assign_task(&signer, task_id)?;

			// Emit a Task Created Event.
			Self::deposit_event(Event::TaskAssigned(signer, task_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An dispatchable call that starts a task by assigning to new account.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn complete_task(origin: OriginFor<T>, task_id: T::Hash) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let signer = ensure_signed(origin)?;

			// Complete task and update storage.
			Self::mark_finished(&signer, task_id)?;

			//TODO: Task can only be completed by the owner. Otherwise, Error

			// Emit a Task Created Event.
			Self::deposit_event(Event::TaskCompleted(signer, task_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An dispatchable call that starts a task by assigning to new account.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn remove_task(origin: OriginFor<T>, task_id: T::Hash) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let signer = ensure_signed(origin)?;

			// Complete task and update storage.
			Self::delete_task(&signer, task_id)?;

			//TODO: Task can only be deleted by the creator. Otherwise, Error

			// Emit a Task Created Event.
			Self::deposit_event(Event::TaskCompleted(signer, task_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
	
	//Helper functions
	impl<T:Config> Pallet<T> {

		pub fn new_task(new_creator: &T::AccountId, requirements: Vec<u8>, budget: u32) -> Result<T::Hash, Error<T>> {
			
			let task = Task::<T> {
				creator: new_creator.clone(),
				requirements: requirements,
				status: Created,
				budget: budget,
				owner: new_creator.clone(),
			};

			let task_id = T::Hashing::hash_of(&task);
			
			// Insert task into Hashmap
			<Tasks<T>>::insert(task_id, task);

			// Increase task count
			let new_count = Self::task_count().checked_add(1).ok_or(<Error<T>>::TaskCountOverflow)?;
			<TaskCount<T>>::put(new_count);

			Ok(task_id)
		}

		pub fn assign_task(to: &T::AccountId, task_id:T::Hash) -> Result<(), Error<T>> {
			// Check if task exists
			let mut task = Self::tasks(&task_id).ok_or(<Error<T>>::TaskNotExist)?;

			let task_creator = task.creator.clone();

			task.owner = to.clone();
			task.status = TaskStatus::InProgress;

			<Tasks<T>>::insert(task_id, task);

			Ok(())
		}


		pub fn mark_finished(to: &T::AccountId, task_id:T::Hash) -> Result<(), Error<T>> {
			// Check if task exists
			let mut task = Self::tasks(&task_id).ok_or(<Error<T>>::TaskNotExist)?;

			task.owner = to.clone();
			task.status = TaskStatus::Closed;

			// Insert into update task
			<Tasks<T>>::insert(task_id, task);

			Ok(())
		}

		pub fn delete_task(owner: &T::AccountId, task_id:T::Hash) -> Result<(), Error<T>> {
			// Check if task exists
			let task = Self::tasks(&task_id).ok_or(<Error<T>>::TaskNotExist)?;
			
			//Check if the owner is the one who created task
			ensure!(Self::is_task_creator(&task_id, &owner)?, <Error<T>>::OnlyCreatorClosesTask);


			// remove task once closed
			<Tasks<T>>::remove(task_id);

			// Reduce task count
			let new_count = Self::task_count().saturating_sub(1);
			<TaskCount<T>>::put(new_count);
			
			Ok(())
		}

		pub fn is_task_creator(task_id: &T::Hash, task_closer: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::tasks(task_id) {
				Some(task) => Ok(task.creator == *task_closer),
				None => Err(<Error<T>>::TaskNotExist)
			}
		}

	}

}