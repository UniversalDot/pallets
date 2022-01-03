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
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

use frame_support::{
	traits::{Currency}};

// Helper function to assert event thrown during verification
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

// This creates an `Profile` object.
fn create_profile_info<T: Config>(_num_fields: u32) -> Profile<T> {
	// let data = Data::Raw(vec![0; 32].try_into().unwrap());
	let mut interests = Vec::new();
	interests.push(u8::MAX);

	let caller: T::AccountId = whitelisted_caller();
	let balance = T::Currency::free_balance(&caller);

	let info = Profile {
		owner: caller,
		interests: interests,
		balance: Some(balance),
		reputation: u32::MAX,
	};

	return info
}


benchmarks! {
	profile_creation {
		/* setup initial state */
		
		let caller: T::AccountId = whitelisted_caller();

		let x in 1 .. 100;  // # of profiles
		let s in 1 .. u8::MAX.into(); // max bytes for interests
		
		// Create profile
		let profile = create_profile_info::<T>(1);
		
		// Create vector of interests
		let interests = vec![0u8, s as u8];

	}: create_profile(RawOrigin::Signed(caller), interests)
		/* the code to be benchmarked above*/
	
	verify {
		/* verifying final state */
		let caller: T::AccountId = whitelisted_caller();
		assert_last_event::<T>(Event::<T>::ProfileCreated { who: caller }.into());
	}

	sort_vector {
		let x in 0 .. 10000;
		let mut m = Vec::<u32>::new();
		for i in (0..x).rev() {
			m.push(i);
		}
	}: {
		// The benchmark execution phase could also be a closure with custom code
		m.sort();
	}

	benchmark_name {
		/* setup initial state */
	}: {
		/* the code to be benchmarked */
	}
	verify {
		/* verifying final state */
	}

	profile_update {
		/* setup initial state */
		let create_account_caller: T::AccountId = whitelisted_caller();
		let update_account_caller: T::AccountId = whitelisted_caller();

		// Create vector of interests
		let s in 1 .. u8::MAX.into(); // max bytes for interests
		let interests = vec![0u8, s as u8];
		let interests_update = vec![0u8, s as u8];

		// before we update profile, profile must be created
		PalletProfile::<T>::create_profile(RawOrigin::Signed(create_account_caller).into(), interests);
		
	}: update_profile(RawOrigin::Signed(update_account_caller), interests_update)
	verify {
		/* verifying final state */
		let caller: T::AccountId = whitelisted_caller();
		assert_last_event::<T>(Event::<T>::ProfileUpdated { who: caller }.into());
	}

	profile_remove {
		/* setup initial state */
		let create_account_caller: T::AccountId = whitelisted_caller();
		let delete_account_caller: T::AccountId = whitelisted_caller();

		// Create vector of interests
		let s in 1 .. u8::MAX.into(); // max bytes for interests
		let interests = vec![0u8, s as u8];

		// before we delete profile, profile must be created
		PalletProfile::<T>::create_profile(RawOrigin::Signed(create_account_caller).into(), interests);
	}: 
	/* the code to be benchmarked */
	remove_profile(RawOrigin::Signed(delete_account_caller))
		
	verify {
		/* verifying final state */
		let caller: T::AccountId = whitelisted_caller();
		assert_last_event::<T>(Event::<T>::ProfileDeleted { who: caller }.into());
	}
}

impl_benchmark_test_suite!(PalletProfile, crate::mock::new_test_ext(), crate::mock::Test,);