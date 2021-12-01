use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};


#[test]
fn create_new_task(){
	new_test_ext().execute_with( || {
		let mut vec = Vec::new();
		vec.push(2);
		
		// Ensure new task can be created with [signer, requirements vector, budget]
		assert_ok!(Task::create_task(Origin::signed(1), vec, 7));
	});
}

#[test]
fn increase_task_count_when_creating_task(){
	new_test_ext().execute_with( || {
		let mut vec = Vec::new();
		vec.push(2);
		
		// Ensure new task can be created with [signer, requirements vector, budget]
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
		
		// Ensure new task can be created with [signer, requirements vector, budget]
		assert_ok!(Task::create_task(Origin::signed(1), vec1, 7));
		assert_ok!(Task::create_task(Origin::signed(1), vec2, 99));

		// Assert that count is incremented to 2 after task creation
		assert_eq!(Task::task_count(), 2);
	});
}

#[test]
fn assign_task_in_progress(){
	new_test_ext().execute_with( || {

		let mut vec1 = Vec::new();
		vec1.push(2);

		assert_ok!(Task::create_task(Origin::signed(10), vec1, 7));

		//TODO: Get taskID
		//let task = Task::tasks(10).len();
		
		//assert_ok!(Task::start_task(Origin::signed(2), task_id));
	});
}


#[test]
fn decrease_task_count_when_removing_task(){
	new_test_ext().execute_with( || {
		
		let mut vec = Vec::new();
		vec.push(2);
		
		// Ensure new task can be created with [signer, requirements vector, budget]
		assert_ok!(Task::create_task(Origin::signed(1), vec, 8));


		// TODO:Remove task
		// Task::remove_task()
		// Assert that count is incremented by 1 after task creation
		assert_eq!(Task::task_count(), 1);

	});
}
