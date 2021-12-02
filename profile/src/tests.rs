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

#[test]
fn create_profile_increases_profile_count() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), vec));

		// Ensure count has decreased
		assert_eq!(Profile::profile_count(), 1);
	});
}

#[test]
fn delete_profile_works() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), vec));

		// Ensure the user can delete their profile
		assert_ok!(Profile::remove_profile(Origin::signed(1)));
	});
}

#[test]
fn delete_profile_decreases_profile_count() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), vec));

		// Ensure teh user can delete their profile
		assert_ok!(Profile::remove_profile(Origin::signed(1)));
		
		// Ensure count is reduced when removing profile
		assert_eq!(Profile::profile_count(), 0);
	});
}