//! Benchmarking setup for pallet-task

use super::*;

#[allow(unused)]
use crate::Pallet as PalletTask;
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
}

impl_benchmark_test_suite!(PalletTask, crate::mock::new_test_ext(), crate::mock::Test,);