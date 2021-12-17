use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

pub const MAX_VALUE:u32 = 251;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(UpdateHook::add_value(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(UpdateHook::single_value(), 42);
	});
}

#[test]
fn it_throws_error_when_max_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(UpdateHook::add_value(Origin::signed(1), MAX_VALUE) , Error::<Test>::Overflow);
	});
}
