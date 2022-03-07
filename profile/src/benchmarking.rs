// This file is part of Substrate.

// Copyright UNIVERSALDOT FOUNDATION
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

//! Benchmarking setup for pallet-profile

use super::*;

#[allow(unused)]
use crate::Pallet as PalletProfile;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller, vec};
use frame_system::RawOrigin;

use frame_support::{
	traits::{Currency}};

// Helper function to assert event thrown during verification
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

// This creates an `Profile` object.
fn create_profile_info<T: Config>(_num_fields: u32) -> Profile<T> {
	
	let s: u8 = u8::MAX.into();
	let interests = vec![0u8, s as u8];
	let username = vec![0u8, s as u8];
	
	let caller: T::AccountId = whitelisted_caller();
	let balance = T::Currency::free_balance(&caller);

	let info = Profile {
		owner: caller,
		name: username,
		interests: interests,
		balance: Some(balance),
		reputation: u32::MAX,
	};

	return info
}


benchmarks! {
	// ** Template for testing extrinsic functions ** //
 	benchmark_name {
		/* setup initial state */
	}: {
		/* the code to be benchmarked */
	}
	verify {
		/* verifying final state */
	}

	profile_creation {
		/* setup initial state */
		
		let caller: T::AccountId = whitelisted_caller();

		// Populate data fields
		let x in 1 .. 100;  // # of profiles
		let s in 1 .. u8::MAX.into(); // max bytes for interests
		let profile = create_profile_info::<T>(1);
		let interests = vec![0u8, s as u8];
		let username = vec![0u8, s as u8];

	}: create_profile(RawOrigin::Signed(caller), username,  interests)
	
	verify {
		/* verifying final state */
		let caller: T::AccountId = whitelisted_caller();
		assert_last_event::<T>(Event::<T>::ProfileCreated { who: caller }.into());
	}

	profile_update {
		/* setup initial state */
		let create_account_caller: T::AccountId = whitelisted_caller();
		let update_account_caller: T::AccountId = whitelisted_caller();

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for interests
		let interests = vec![0u8, s as u8];
		let username = vec![0u8, s as u8];

		// before we update profile, profile must be created
		let _ = PalletProfile::<T>::create_profile(RawOrigin::Signed(create_account_caller).into(), username.clone(), interests.clone());
		
	}: update_profile(RawOrigin::Signed(update_account_caller), username, interests)
	
	verify {
		/* verifying final state */
		let caller: T::AccountId = whitelisted_caller();
		assert_last_event::<T>(Event::<T>::ProfileUpdated { who: caller }.into());
	}

	profile_remove {
		/* setup initial state */
		let create_account_caller: T::AccountId = whitelisted_caller();
		let delete_account_caller: T::AccountId = whitelisted_caller();

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for interests
		let interests = vec![0u8, s as u8];
		let username = vec![0u8, s as u8];

		// before we delete profile, profile must be created
		let _ = PalletProfile::<T>::create_profile(RawOrigin::Signed(create_account_caller).into(), username, interests);

	}: remove_profile(RawOrigin::Signed(delete_account_caller))
	
	verify {
		/* verifying final state */
		let caller: T::AccountId = whitelisted_caller();
		assert_last_event::<T>(Event::<T>::ProfileDeleted { who: caller }.into());
	}
}

impl_benchmark_test_suite!(PalletProfile, crate::mock::new_test_ext(), crate::mock::Test,);