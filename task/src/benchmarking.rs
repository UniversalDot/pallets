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

//! Benchmarking setup for pallet-task

use super::*;

#[allow(unused)]
use crate::Pallet as PalletTask;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller, vec, Vec};
use frame_system::RawOrigin;
use frame_support::traits::{Currency};
use pallet_profile::Pallet as PalletProfile;

// Helper function to assert event thrown during verification
fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

// This creates and returns a `Task` object.
fn create_task_info<T: Config>(_num_fields: u32) -> Task<T> {
	
	// Populate with worst case scenario
	let mut data = Vec::new();
	data.push(u8::MAX);

	// Populate data fields
	let initiator: T::AccountId = whitelisted_caller();
	let volunteer: T::AccountId = whitelisted_caller();
	let owner: T::AccountId = whitelisted_caller();
	let balance = <T as pallet::Config>::Currency::total_balance(&initiator);
	let deadline = u32::MAX;
	let status: TaskStatus = TaskStatus::InProgress;

	// Create object
	let info = Task {
		title: data.clone(),
		specification: data.clone(),
		initiator: initiator,
		volunteer: volunteer,
		current_owner: owner,
		status: status,
		budget: balance,
		deadline: deadline,
	};

	return info
}

// Helper function to create a profile
fn create_profile<T: Config>(){

	let username = Vec::new();
	let interests = Vec::new();

	let caller: T::AccountId = whitelisted_caller();
	let _profile = PalletProfile::<T>::create_profile(RawOrigin::Signed(caller).into(), username, interests);

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

	create_task {
		/* setup initial state */
		let caller: T::AccountId = whitelisted_caller();

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for specification
		let x in 1 .. 2000;
		let title = vec![0u8, s as u8];
		let specification = vec![0u8, s as u8];
		let budget = <T as pallet::Config>::Currency::total_balance(&caller);

		// Create profile before creating a task
		create_profile::<T>();
		create_task_info::<T>(1);
		
	}: 
	/* the code to be benchmarked */
	create_task(RawOrigin::Signed(caller.clone()), title, specification, budget, x)
	
	verify {
		/* verifying final state */
		let caller: T::AccountId = whitelisted_caller();
		let hash = PalletTask::<T>::tasks_owned(&caller)[0];

		assert_last_event::<T>(Event::<T>::TaskCreated(caller, hash).into());
	}

	start_task {
		/* setup initial state */
		let caller_create: T::AccountId = whitelisted_caller();
		let caller_start: T::AccountId = whitelisted_caller();

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for specification
		let x in 1 .. 2000; 
		let title = vec![0u8, s as u8];
		let specification = vec![0u8, s as u8];
		let budget = <T as pallet::Config>::Currency::total_balance(&caller_create);

		// Create profile before creating a task
		create_profile::<T>();		
		let _ = PalletTask::<T>::create_task(RawOrigin::Signed(caller_create.clone()).into(), title, specification, budget, x.into());
		let hash_task = PalletTask::<T>::tasks_owned(&caller_create)[0];
		
	}: start_task(RawOrigin::Signed(caller_start.clone()), hash_task)
		/* the code to be benchmarked */
	
	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::TaskAssigned(caller_start, hash_task).into());
	}

	complete_task {
		/* setup initial state */
		let caller_create: T::AccountId = whitelisted_caller();
		let caller_complete: T::AccountId = whitelisted_caller();

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for specification
		let x in 1 .. 2000;
		let title = vec![0u8, s as u8]; 
		let specification = vec![0u8, s as u8];
		let budget = <T as pallet::Config>::Currency::total_balance(&caller_create);

		// Create profile before creating a task
		create_profile::<T>();		
		let _ = PalletTask::<T>::create_task(RawOrigin::Signed(caller_create.clone()).into(), title, specification, budget, x.into());
		let hash_task = PalletTask::<T>::tasks_owned(&caller_create)[0];
		let _ = PalletTask::<T>::start_task(RawOrigin::Signed(caller_complete.clone()).into(), hash_task.clone());

	}: complete_task(RawOrigin::Signed(caller_complete.clone()), hash_task)
		/* the code to be benchmarked */
	
	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::TaskCompleted(caller_complete, hash_task).into());
	}

	remove_task {
		/* setup initial state */
		let caller_create: T::AccountId = whitelisted_caller();
		let caller_complete: T::AccountId = whitelisted_caller();

		// Populate data fields
		let s in 1 .. u8::MAX.into(); // max bytes for specification
		let x in 1 .. 4000;
		let title = vec![0u8, s as u8]; 
		let specification = vec![0u8, s as u8];
		let budget = <T as pallet::Config>::Currency::total_balance(&caller_create);

		// Create profile before creating a task
		create_profile::<T>();		
		let _ = PalletTask::<T>::create_task(RawOrigin::Signed(caller_create.clone()).into(), title, specification, budget, x.into());
		let hash_task = PalletTask::<T>::tasks_owned(&caller_create)[0];
		let _ = PalletTask::<T>::start_task(RawOrigin::Signed(caller_complete.clone()).into(), hash_task.clone());

	}: remove_task(RawOrigin::Signed(caller_complete.clone()), hash_task)
		/* the code to be benchmarked */
	
	verify {
		/* verifying final state */
		assert_last_event::<T>(Event::<T>::TaskRemoved(caller_complete, hash_task).into());
	}
}

impl_benchmark_test_suite!(PalletTask, crate::mock::new_test_ext(), crate::mock::Test,);