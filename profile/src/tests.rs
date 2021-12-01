use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};



#[test]
fn create_profile_works() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), vec));
	});
}