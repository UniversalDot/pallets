//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;


benchmarks! {
	benchmark_name {
		/* setup initial state */
		let s in 1 .. 100;
		let mut profile = Vec::new();
		profile.push(7);
		let caller: T::AccountId = whitelisted_caller();

	}: create_profile(RawOrigin::Signed(caller), profile)
		/* the code to be benchmarked above*/
	
	verify {
		/* verifying final state */
		assert_eq!(ProfileCount::<T>::get(), s);
	}
}

impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test,);