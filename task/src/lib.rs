// This file is part of Substrate.

// Copyright (C) 2022 UNIVERSALDOT FOUNDATION.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


//! # Task Pallet
//!
//! - [`Config`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The Task Pallet creates a way for users to interact with one another.
//! 
//! There are two types of Users who can interact with tasks. We call them
//! Initiators and Volunteers. 
//! 
//! Initiators are people who have the permission to Create and Remove Tasks.
//! Volunteers are people who have the permission to Start and Complete Tasks.
//! 
//! Anybody can become an Initiator or Volunteer. In other words, 
//! one doesn't need permission to become an Initiator or Volunteer. 
//! 
//! When Tasks are created, there is some associated metadata that shall be defined.
//! This includes the following:
//! - Task Specification (Defining the Task specification)
//! - Task Budget (The cost of completion for the Task)
//! - Task Deadline (The specified time until which the task should be completed)
//! 
//! Furthermore, budget funds are locked in escrow when task is created. 
//! Funds are removed from escrow when task is removed. 
//!
//! ## Interface
//!
//! ### Public Functions
//!
//! - `create_task` - Function used to create a new task. 
//! 
//! - `start_task` - Function used to start already existing task.
//! 
//! - `complete_task` - Function used to complete a task.
//! 
//! - `remove_task` - Function used to remove task.
//!
//! ## Related Modules
//!


#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use crate::TaskStatus::Created; //TODO: Better import
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use frame_support::{
		sp_runtime::traits:: Hash,
		traits::{Currency, tokens::ExistenceRequirement}, 
		transactional};
	use scale_info::TypeInfo;
	use sp_std::vec::Vec;

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

	// Use AccountId from frame_system
	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> =<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// Struct for holding Task information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Task<T: Config> {
		pub title: Vec<u8>,
		pub specification: Vec<u8>,
		pub initiator: AccountOf<T>,
		pub volunteer: AccountOf<T>,
		pub current_owner: AccountOf<T>,
		pub status: TaskStatus,
		pub budget: BalanceOf<T>,
		pub deadline: u32,
	}

	// Set TaskStatus enum.
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
	pub trait Config: frame_system::Config + pallet_profile::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Currency type that is linked with AccountID
		type Currency: Currency<Self::AccountId>;

		/// The maximum amount of tasks a single account can own.
		#[pallet::constant]
		type MaxTasksOwned: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn task_count)]
	/// TaskCount: Get total number of Tasks in the system
	pub(super) type TaskCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn tasks)]
	/// Tasks: Store Tasks in a  Storage Map where [key: hash, value: Task]
	pub(super) type Tasks<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Task<T>>;

	#[pallet::storage]
	#[pallet::getter(fn tasks_owned)]
	/// Keeps track of which Accounts own which Tasks.
	pub(super) type TasksOwned<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<T::Hash, T::MaxTasksOwned>, ValueQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event for creation of task [AccountID, hash id]
		TaskCreated(T::AccountId, T::Hash),

		/// Task assigned to new account [AccountID, hash id]
		TaskAssigned(T::AccountId, T::Hash),

		/// Task completed by assigned account [AccountID, hash id]
		TaskCompleted(T::AccountId, T::Hash),

		/// Task removed [AccountID, hash id]
		TaskRemoved(T::AccountId, T::Hash),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Reached maximum number of tasks.
		TaskCountOverflow,
		/// The given task doesn't exists. Try again
		TaskNotExist,
		/// Only the initiator of task has the rights to remove task
		OnlyInitiatorClosesTask,
		/// Not enough balance to pay
		NotEnoughBalance,
		/// Exceed maximum tasks owned
		ExceedMaxTasksOwned,
		/// You are not allowed to complete this task
		NoPermissionToComplete,
		/// This account has no Profile yet. 
		NoProfile,
        ProfileAddReputationFailed, // TODO: remove this when pallet_profile returns DispatchError
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Function call that creates tasks.  [ origin, specification, budget, deadline]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_task(origin: OriginFor<T>, title: Vec<u8>, specification: Vec<u8>, budget: BalanceOf<T>, deadline: u32) -> DispatchResultWithPostInfo {
			
			// Check that the extrinsic was signed and get the signer.
			let signer = ensure_signed(origin)?;

			// Update storage.
			let task_id = Self::new_task(&signer, &title, &specification, &budget, &deadline)?;
			
			// TODO: Check if user has balance to create task
			// T::Currency::reserve(&signer, budget).map_err(|_| "locker can't afford to lock the amount requested")?;

			// Emit a Task Created Event.
			Self::deposit_event(Event::TaskCreated(signer, task_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		/// Function call that starts a task by assigning new task owner. [origin, task_id]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn start_task(origin: OriginFor<T>, task_id: T::Hash) -> DispatchResult {
			
			// Check that the extrinsic was signed and get the signer.
			let signer = ensure_signed(origin)?;

			// Assign task and update storage.
			Self::assign_task(&signer, &task_id)?;

			// Emit a Task Assigned Event.
			Self::deposit_event(Event::TaskAssigned(signer, task_id));
			
			Ok(())
		}

		/// Function that completes a task [origin, task_id]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2,1))]
		pub fn complete_task(origin: OriginFor<T>, task_id: T::Hash) -> DispatchResult {
			
			// Check that the extrinsic was signed and get the signer.
			let signer = ensure_signed(origin)?;

			// Complete task and update storage.
			Self::mark_finished(&signer, &task_id)?;

			// Emit a Task Completed Event.
			Self::deposit_event(Event::TaskCompleted(signer, task_id));
			
			Ok(())
		}

		/// Function to remove task. [origin, task_id]
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn remove_task(origin: OriginFor<T>, task_id: T::Hash) -> DispatchResult {
			
			// Check that the extrinsic was signed and get the signer.
			let signer = ensure_signed(origin)?;

			// Complete task and update storage.
			Self::delete_task(&signer, &task_id)?;

			// Emit a Task Removed Event.
			Self::deposit_event(Event::TaskRemoved(signer, task_id));
			
			Ok(())
		}
	}
	
	// *** Helper functions *** //
	impl<T:Config> Pallet<T> {

		pub fn new_task(from_initiator: &T::AccountId, title: &[u8], specification: &[u8], budget: &BalanceOf<T>, deadline: &u32) -> Result<T::Hash, DispatchError> {

			// Ensure user has a profile before creating a task
			ensure!(pallet_profile::Pallet::<T>::has_profile(from_initiator).unwrap(), <Error<T>>::NoProfile);
			
			// Init Task Object
			let task = Task::<T> {
				title: title.to_vec(),
				specification: specification.to_vec(),
				initiator: from_initiator.clone(),
				volunteer: from_initiator.clone(),
				status: Created,
				budget: *budget,
				current_owner: from_initiator.clone(),
				deadline: *deadline,
			};

			// Create hash of task
			let task_id = T::Hashing::hash_of(&task);

			// Performs this operation first because as it may fail
			<TasksOwned<T>>::try_mutate(&from_initiator, |tasks_vec| {
				tasks_vec.try_push(task_id)
			}).map_err(|_| <Error<T>>::ExceedMaxTasksOwned)?;
			
			// Insert task into Hashmap
			<Tasks<T>>::insert(task_id, task);

			// Increase task count
			let new_count = Self::task_count().checked_add(1).ok_or(<Error<T>>::TaskCountOverflow)?;
			<TaskCount<T>>::put(new_count);

			Ok(task_id)
		}

		pub fn assign_task(to: &T::AccountId, task_id: &T::Hash) -> Result<(), DispatchError> {
			// Check if task exists
			let mut task = Self::tasks(&task_id).ok_or(<Error<T>>::TaskNotExist)?;

			// Remove task ownership from previous owner
			let prev_owner = task.current_owner.clone(); 
			<TasksOwned<T>>::try_mutate(&prev_owner, |owned| {
				if let Some(index) = owned.iter().position(|&id| id == *task_id) {
					owned.swap_remove(index);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::TaskNotExist)?;

			// Change task properties and insert
			task.current_owner = to.clone();
			task.volunteer = to.clone();
			task.status = TaskStatus::InProgress;
			<Tasks<T>>::insert(task_id, task);

			// Assign task to volunteer
			<TasksOwned<T>>::try_mutate(to, |vec| {
				vec.try_push(*task_id)
			}).map_err(|_| <Error<T>>::ExceedMaxTasksOwned)?;

			Ok(())
		}


		pub fn mark_finished(to: &T::AccountId, task_id: &T::Hash) -> Result<(), DispatchError> {
			
			// Check if task exists
			let mut task = Self::tasks(&task_id).ok_or(<Error<T>>::TaskNotExist)?;

			// Check if task is in progress before closing
			ensure!(TaskStatus::InProgress == task.status, <Error<T>>::NoPermissionToComplete);

			// TODO: Check if the volunteer is the one who finished task


			// Remove task ownership from current signer 
			<TasksOwned<T>>::try_mutate(&to, |owned| {
				if let Some(index) = owned.iter().position(|&id| id == *task_id) {
					owned.swap_remove(index);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::TaskNotExist)?;

			// Set current owner to initiator
			task.current_owner = task.initiator.clone();
			task.status = TaskStatus::Closed;
			let task_initiator = task.initiator.clone();

			// Insert into update task
			<Tasks<T>>::insert(task_id, task);

			// Assign task to new owner (original initiator)
			<TasksOwned<T>>::try_mutate(task_initiator, |vec| {
				vec.try_push(*task_id)
			}).map_err(|_| <Error<T>>::ExceedMaxTasksOwned)?;

			Ok(())
		}

		pub fn delete_task(task_initiator: &T::AccountId, task_id: &T::Hash) -> Result<(), DispatchError> {
			// Check if task exists
			let task = Self::tasks(&task_id).ok_or(<Error<T>>::TaskNotExist)?;
			
			//Check if the owner is the one who created task
			ensure!(Self::is_task_initiator(task_id, task_initiator)?, <Error<T>>::OnlyInitiatorClosesTask);

			// Remove from ownership
			<TasksOwned<T>>::try_mutate(&task_initiator, |owned| {
				if let Some(index) = owned.iter().position(|&id| id == *task_id) {
					owned.swap_remove(index);
					return Ok(());
				}
				Err(())
			}).map_err(|_| <Error<T>>::TaskNotExist)?;

			// Transfer balance to volunteer
			let volunteer = task.volunteer.clone();
			let budget = task.budget;
			Self::transfer_balance(task_initiator, &volunteer, budget)?;

			// Reward reputation points to profiles who created/completed a task
			Self::handle_reputation(task_id)?;

			// remove task once closed
			<Tasks<T>>::remove(task_id);

			// Reduce task count
			let new_count = Self::task_count().saturating_sub(1);
			<TaskCount<T>>::put(new_count);
			
			Ok(())
		}

		// Function to check if the current signer is the task_initiator
		pub fn is_task_initiator(task_id: &T::Hash, task_closer: &T::AccountId) -> Result<bool, DispatchError> {
			match Self::tasks(task_id) {
				Some(task) => Ok(task.initiator == *task_closer),
				None => Err(<Error<T>>::TaskNotExist.into())
			}
		}

		// Function to transfer balance from one account to another
		#[transactional]
		pub fn transfer_balance(task_initiator: &T::AccountId, task_volunteer: &T::AccountId, budget: BalanceOf<T>) -> Result<(),  DispatchError> {
			<T as self::Config>::Currency::transfer(task_initiator, task_volunteer, budget, ExistenceRequirement::KeepAlive)
		}

		// Handles reputation update for profiles
		pub fn handle_reputation(task_id: &T::Hash) -> Result<(), DispatchError> {

			// Check if task exists
			let task = Self::tasks(&task_id).ok_or(<Error<T>>::TaskNotExist)?;

			// Ensure that reputation is added only when task is in status Closed
			if task.status == TaskStatus::Closed {
				pallet_profile::Pallet::<T>::add_reputation(&task.initiator).map_err(|_| Error::<T>::ProfileAddReputationFailed)?;
				pallet_profile::Pallet::<T>::add_reputation(&task.volunteer).map_err(|_| Error::<T>::ProfileAddReputationFailed)?;
			}

			Ok(())
		}
	}
}
