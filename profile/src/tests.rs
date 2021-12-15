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
fn only_one_profile_per_account_allowed() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), vec));

		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);

		assert_noop!(Profile::create_profile(Origin::signed(1), vec), Error::<Test>::ProfileAlreadyCreated );
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

#[test]
fn user_can_only_delete_own_profile() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), vec));

		// Ensure another user can NOT delete others profile
		assert_noop!(Profile::remove_profile(Origin::signed(2)), Error::<Test>::NoDeletionAuthority);
		
		// Ensure count is NOT reduced when removing profile
		assert_eq!(Profile::profile_count(), 1);
	});
}

#[test]
fn user_can_update_profile() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), vec));

		// Create new vector of interests
		let mut vec2 = Vec::new();
		vec2.push(99);

		// Ensure user can update profile with new interests
		assert_ok!(Profile::update_profile(Origin::signed(1), vec2));
		
		// Ensure count is NOT reduced when removing profile
		assert_eq!(Profile::profile_count(), 1);
		// TODO: Make sure this test is cover appropriately
		// assert_eq!(Profile::profiles(Origin::signed(1)).len(), 1);
	});
}

#[test]
fn user_can_only_update_own_profile() {
	new_test_ext().execute_with(|| {
		// Create vector of interests
		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the user can create profile
		assert_ok!(Profile::create_profile(Origin::signed(1), vec));

		// Create new vector of interests
		let mut vec2 = Vec::new();
		vec2.push(99);

		// Ensure another user can NOT update others profile.
		assert_noop!(Profile::update_profile(Origin::signed(2), vec2), Error::<Test>::NoUpdateAuthority);
	});
}