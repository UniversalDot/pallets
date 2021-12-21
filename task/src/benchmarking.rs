//! Benchmarking setup for pallet-task

use super::*;

#[allow(unused)]
use crate::Pallet as PalletTask;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use frame_support::traits::Currency;

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
	let balance = T::Currency::total_balance(&initiator);
	let deadline = u64::MAX;
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


benchmarks! {
	benchmark_name {
		/* setup initial state */
	  }: {
		/* the code to be benchmarked */
	  }
	  verify {
		/* verifying final state */
	  }
}

impl_benchmark_test_suite!(PalletTask, crate::mock::new_test_ext(), crate::mock::Test,);