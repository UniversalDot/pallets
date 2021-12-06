use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Dao::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		// assert_eq!(Dao::something(), Some(42));
	});
}

#[test]
fn can_create_vision() {
	new_test_ext().execute_with(|| {
		
		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), vec));
	});
}

#[test]
fn can_not_create_vision_that_already_exists() {
	new_test_ext().execute_with(|| {
		
		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), vec));

		// Create a new vector with same content. If the same content its hash is the same.
		let mut vec2 = Vec::new();
		vec2.push(7);

		// Ensure the DAO can NOT Create create a vision that already exists
		assert_noop!(Dao::create_vision(Origin::signed(1), vec2), Error::<Test>::VisionAlreadyExists);
	});
}

#[test]
fn can_remove_vision() {
	new_test_ext().execute_with(|| {
		
		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), vec));

		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the DAO can remove a vision document
		assert_ok!(Dao::remove_vision(Origin::signed(1), vec));
	});
}

#[test]
fn when_removing_vision_ensure_it_exists() {
	new_test_ext().execute_with(|| {

		let mut vec2 = Vec::new();
		vec2.push(8);

		// Ensure the DAO can remove a vision document
		assert_noop!(Dao::remove_vision(Origin::signed(1), vec2), Error::<Test>::NoSuchVision);
	});
}