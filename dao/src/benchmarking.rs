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

//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as PalletDao;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

// Helper function to assert event thrown during verification
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	benchmark_name {
		/* setup initial state */
	}: {
		/* the code to be benchmarked */
	}
	verify {
		/* verifying final state */
	}

	create_vision {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let vision = vec![0u8, s as u8];

	}: create_vision(RawOrigin::Signed(caller.clone()), vision.clone()) 
	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::VisionCreated (caller, vision ).into());
	}

	remove_vision {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let vision = vec![0u8, s as u8];

		// Create vision before removing
		PalletDao::<T>::create_vision(RawOrigin::Signed(caller.clone()).into(), vision.clone());

	}: remove_vision(RawOrigin::Signed(caller.clone()), vision.clone()) 
	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::VisionRemoved (caller, vision ).into());
	}

	sign_vision {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let vision = vec![0u8, s as u8];

		// Create vision before removing
		PalletDao::<T>::create_vision(RawOrigin::Signed(caller.clone()).into(), vision.clone());

	}: sign_vision(RawOrigin::Signed(caller.clone()), vision.clone()) 
	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::VisionSigned (caller, vision ).into());
	}

	unsign_vision {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		let s in 1 .. u8::MAX.into();
		let vision = vec![0u8, s as u8];

		// Create vision before removing
		PalletDao::<T>::create_vision(RawOrigin::Signed(caller.clone()).into(), vision.clone());
		PalletDao::<T>::sign_vision(RawOrigin::Signed(caller.clone()).into(), vision.clone());


	}: unsign_vision(RawOrigin::Signed(caller.clone()), vision.clone()) 
	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::VisionUnsigned (caller, vision ).into());
	}

	//   add_members {
	// 	/* setup initial state */
	// 	let caller: T::AccountId = whitelisted_caller();

	// 	let s in 1 .. u8::MAX.into();
	// 	let name = vec![0u8, s as u8];
		
	// 	let account: T::AccountId = whitelisted_caller();


	//   }: add_members(RawOrigin::Signed(caller.clone()), name.clone(), account.clone())
	// 	/* the code to be benchmarked */
	//   verify {
	// 	/* verifying final state */
	// 	assert_last_event::<T>(Event::<T>::MemberAdded (caller, account ).into());
	//   }
}

impl_benchmark_test_suite!(PalletDao, crate::mock::new_test_ext(), crate::mock::Test,);