use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(UpdateHook::add_value(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(UpdateHook::single_value(), 42);
	});
}

