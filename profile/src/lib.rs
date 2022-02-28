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


//! # Profile Pallet
//!
//! - [`Config`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The Profile Pallet creates a user profile per AccountID.
//! The Profile is used to enrich the AccountID information with user specific
//! metadata such as personal interests, name, reputation, etc. 
//!
//! ## Interface
//!
//! ### Public Functions
//!
//! - `create_profile` - Function used to create a new user profile.
//! 
//! - `update_profile` - Function used to update an already existing user profile.
//! 
//! - `remove_profile` - Function used to delete an existing user profile.
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
		pub name: Vec<u8>,
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

	#[pallet::storage]
	#[pallet::getter(fn profile_count)]
	/// Storage Value that counts the total number of Profiles
	pub(super) type ProfileCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn profiles)]
	/// Stores a Profile unique properties in a StorageMap.
	pub(super) type Profiles<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Profile<T>>;

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Profile was successfully created. 
		ProfileCreated { who: T::AccountId },

		/// Profile was successfully deleted.
		ProfileDeleted { who: T::AccountId },

		/// Profile was successfully updated.
		ProfileUpdated { who: T::AccountId },

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Reached maximum number of profiles.
		ProfileCountOverflow,
		/// No permission to update this profile.
		NoUpdateAuthority,
		/// Profiles can only be deleted by the creator
		NoDeletionAuthority,
		/// One Account can only create a single profile. 
		ProfileAlreadyCreated,
		/// This Account has not yet created a profile.
		NoProfileCreated,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Dispatchable call that enables every new actor to create personal profile in storage.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_profile(origin: OriginFor<T>, username: Vec<u8>, interests: Vec<u8>) -> DispatchResult {
			
			// Check that the extrinsic was signed and get the signer.
			let account = ensure_signed(origin)?;

			// Call helper function to generate Profile Struct
			let _profile_id = Self::generate_profile(&account, username, interests)?;

			// Emit an event.
			Self::deposit_event(Event::ProfileCreated{ who:account });
			
			Ok(())
		}

		/// Dispatchable call that ensures user can update existing personal profile in storage.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn update_profile(origin: OriginFor<T>, username: Vec<u8>, interests: Vec<u8>) -> DispatchResult {

			// Check that the extrinsic was signed and get the signer.
			let account = ensure_signed(origin)?;
			
			// Since Each account can have one profile, we call into generate profile again
			let _profile_id = Self::change_profile(&account, username, interests)?;

			// Emit an event.
			Self::deposit_event(Event::ProfileUpdated{ who: account });
			
			Ok(())
		}


		/// Dispatchable call that enables every new actor to delete profile from storage.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_profile(origin: OriginFor<T>) -> DispatchResult {
			
			// Check that the extrinsic was signed and get the signer.
			let account = ensure_signed(origin)?;

			// Call helper function to delete profile
			Self::delete_profile(&account)?;

			// Emit an event.
			Self::deposit_event(Event::ProfileDeleted{ who : account});
			
			Ok(())
		}

	}

	// ** Helper internal functions ** //
	impl<T:Config> Pallet<T> {
		// Generates initial Profile.
		pub fn generate_profile(owner: &T::AccountId, username: Vec<u8>, interests_vec: Vec<u8>) -> Result<T::Hash, Error<T>> {
			
			// Check if profile already exists for owner
			ensure!(!Profiles::<T>::contains_key(&owner), Error::<T>::ProfileAlreadyCreated);

			// Get current balance of owner
			let balance = T::Currency::free_balance(owner);

			// Populate Profile struct
			let profile = Profile::<T> {
				owner: owner.clone(),
				name: username,
				interests: interests_vec,
				balance: Some(balance),
				reputation: 0,
			};

			// Get hash of profile
			let profile_id = T::Hashing::hash_of(&profile);

			// Insert profile into HashMap
			<Profiles<T>>::insert(owner, profile);

			// Increase profile count
			let new_count = Self::profile_count().checked_add(1).ok_or(<Error<T>>::ProfileCountOverflow)?;
			<ProfileCount<T>>::put(new_count);

			Ok(profile_id)
		}

		// Changes existing profile
		pub fn change_profile(owner: &T::AccountId, new_username: Vec<u8>, new_interests: Vec<u8>) -> Result<T::Hash, Error<T>> {
			
			// Ensure that only owner can update profile
			let mut profile = Self::profiles(owner).ok_or(<Error<T>>::NoUpdateAuthority)?;
			
			// Change interests of owner
			profile.change_interests(new_interests);

			profile.change_username(new_username);

			// Get hash of profile
			let profile_id = T::Hashing::hash_of(&profile);

			// Insert profile into HashMap
			<Profiles<T>>::insert(owner, profile);

			// Return hash of profileID
			Ok(profile_id)
		}

		// Public function that deletes a user profile
		pub fn delete_profile(owner: &T::AccountId) -> Result<(), Error<T>> {
			
			// Ensure that only creator of profile can delete it
			Self::profiles(owner).ok_or(<Error<T>>::NoDeletionAuthority)?;
			
			// Remove profile from storage
			<Profiles<T>>::remove(owner);

			// Reduce profile count
			let new_count = Self::profile_count().saturating_sub(1);
			<ProfileCount<T>>::put(new_count);
			
			Ok(())
		}

		// Public function that adds reputation to a profile
		pub fn add_reputation(owner: &T::AccountId) -> Result<(), Error<T>> {
			
			// Get current profile
			let mut profile = Self::profiles(owner).ok_or(<Error<T>>::NoUpdateAuthority)?;

			// Increase reputation
			profile.increase_reputation();

			// Insert into storage a new profile
			<Profiles<T>>::insert(owner, profile);

			Ok(())
		}

		// Public function that check if user has a profile
		pub fn has_profile(owner: &T::AccountId) -> Result<bool, Error<T>>  {

			// Check if an account has a profile
			Self::profiles(owner).ok_or(<Error<T>>::NoProfileCreated)?;

			Ok(true)
		}
	}

	// Change the reputation on a Profile (TODO MVP2: Improve reputation functions)
	impl<T:Config> Profile<T> {
		pub fn increase_reputation(&mut self) {
			self.reputation += 1;
		}

		pub fn decrease_reputation(&mut self) {
			self.reputation -= 1;
		}

		pub fn change_interests(&mut self, new_interests: Vec<u8>) {
			self.interests = new_interests;
		}

		pub fn change_username(&mut self, new_username: Vec<u8>) {
			self.name = new_username;
		}
	} 

}