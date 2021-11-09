use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Profile::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(Profile::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(Profile::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

#[test]
fn stores_value_in_map() {
	new_test_ext().execute_with(|| {
		// Ensure something is stored in map
		assert_ok!(Profile::set_single_entry(Origin::signed(1), 17));
	});
}