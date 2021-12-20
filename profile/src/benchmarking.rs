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

		// Create 10_000 profiles
		let x in 1 .. 100;
		let profile = create_profile_info::<T>(x);
		
		// Create vector of interests
		let mut interests = Vec::new();
		interests.push(u8::MAX);

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

	//   profile_update {
	// 	/* setup initial state */
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	// create_profile(RawOrigin::Signed(caller), interests);

	// 	// Create vector of interests
	// 	let mut interests = Vec::new();
	// 	interests.push(7);

	// 	Template::create_profile(RawOrigin::Signed(caller), interests);
	//   }: update_profile(RawOrigin::Signed(caller), interests)
	//   verify {
	// 	/* verifying final state */
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	assert_last_event::<T>(Event::<T>::ProfileUpdated { who: caller }.into());
	//   }
}

impl_benchmark_test_suite!(PalletProfile, crate::mock::new_test_ext(), crate::mock::Test,);