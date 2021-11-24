use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Task::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(Task::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(Task::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

#[test]
fn create_new_task(){
	new_test_ext().execute_with( || {
		let mut vec = Vec::new();
		vec.push(2);
		
		// Ensure new task can be creted with [signer, requirements vector, buget]
		assert_ok!(Task::create_task(Origin::signed(1), vec, 7));
	});
}

#[test]
fn increase_task_count_when_creating_task(){
	new_test_ext().execute_with( || {
		let mut vec = Vec::new();
		vec.push(2);
		
		// Ensure new task can be creted with [signer, requirements vector, buget]
		assert_ok!(Task::create_task(Origin::signed(1), vec, 7));

		// Assert that count is incremented by 1 after task creation
		assert_eq!(Task::task_count(), 1);
	});
}

#[test]
fn increase_task_count_when_creating_two_tasks(){
	new_test_ext().execute_with( || {

		let mut vec1 = Vec::new();
		vec1.push(2);

		let mut vec2 = Vec::new();
		vec2.push(7);
		
		// Ensure new task can be creted with [signer, requirements vector, buget]
		assert_ok!(Task::create_task(Origin::signed(1), vec1, 7));
		assert_ok!(Task::create_task(Origin::signed(1), vec2, 99));

		// Assert that count is incremented to 2 after task creation
		assert_eq!(Task::task_count(), 2);
	});
}