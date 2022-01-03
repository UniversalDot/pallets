//! Benchmarking setup for pallet-task

use super::*;

#[allow(unused)]
use crate::Pallet as PalletTask;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use frame_support::traits::{Currency};
use frame_support::sp_runtime::traits::Hash;
use std::convert::TryInto;
use sp_std::ptr::hash;
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

	let initiator: T::AccountId = whitelisted_caller();
	let volunteer: T::AccountId = whitelisted_caller();
	let owner: T::AccountId = whitelisted_caller();
	let balance = <T as pallet::Config>::Currency::total_balance(&initiator);
	let deadline = u32::MAX;
	let status: TaskStatus = TaskStatus::InProgress;

	// Create object
	let info = Task {
		initiator: initiator,
		volunteer: volunteer,
		current_owner: owner,
		requirements: data,
		status: status,
		budget: balance,
		deadline: deadline,
	};

	return info
}

// Helper function to create a profile
fn create_profile<T: Config>(){

	let caller: T::AccountId = whitelisted_caller();
	let _profile = PalletProfile::<T>::create_profile(RawOrigin::Signed(caller).into(), Vec::new());

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

		// Create profile before creating a task
		create_profile::<T>();
		
		let task = create_task_info::<T>(1);
		let task_hash = T::Hashing::hash_of(&task);

		let s = 255; // max bytes for requirements
		let x = 2000; 
		// let s in 1 .. u8::MAX.into(); // max bytes for requirements
		// let x in 1 .. 2000;


		let requirements = vec![0u8, s as u8];
		let budget = <T as pallet::Config>::Currency::total_balance(&caller);

		//let task_hash = PalletTask::<T>::new_task(&caller, &requirements, &budget, &x)?;

	}: 
	/* the code to be benchmarked */
	create_task(RawOrigin::Signed(caller), requirements, budget, x)
	
	verify {
		/* verifying final state */
		let caller: T::AccountId = whitelisted_caller();
		// TODO: fix task hash error
		// assert_last_event::<T>(Event::<T>::TaskCreated(caller, hash).into());
		assert_eq!(PalletTask::<T>::task_count(), 1);
	}

	start_task {
		/* setup initial state */
		let caller_create: T::AccountId = whitelisted_caller();
		let caller_start: T::AccountId = whitelisted_caller();

		// let task = create_task_info::<T>(1);
		// let hash_task = T::Hashing::hash_of(&task);

		let s in 1 .. u8::MAX.into(); // max bytes for requirements
		let x in 1 .. 2000; 

		let requirements = vec![0u8, s as u8];
		let budget = <T as pallet::Config>::Currency::total_balance(&caller_create);

		// Create profile before creating a task
		create_profile::<T>();		
		PalletTask::<T>::create_task(RawOrigin::Signed(caller_create.clone()).into(), requirements, budget, x.into());
		let hash_task = PalletTask::<T>::tasks_owned(&caller_create)[0];
		
	}: start_task(RawOrigin::Signed(caller_start), hash_task)
		/* the code to be benchmarked */
	
	verify {
		/* verifying final state */
		assert_eq!(PalletTask::<T>::task_count(), 1);
	}
}

impl_benchmark_test_suite!(PalletTask, crate::mock::new_test_ext(), crate::mock::Test,);