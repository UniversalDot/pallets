//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

use frame_support::{
	traits::{Currency}};



// This creates an `Profile` object.
// All data is pre-populated with some arbitrary bytes.
fn create_profile_info<T: Config>(num_fields: u32) -> Profile<T> {
	// let data = Data::Raw(vec![0; 32].try_into().unwrap());
	let mut interests = Vec::new();
	interests.push(77);

	let caller: T::AccountId = whitelisted_caller();

	let balance = T::Currency::free_balance(&caller);

	let info = Profile {
		owner: caller,
		interests: interests,
		balance: Some(balance),
		reputation: 1111111,
	};

	return info
}


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