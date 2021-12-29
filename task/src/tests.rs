use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

pub const DEADLINE:u32 = 77; 

#[test]
fn create_new_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), Vec::new()));
		
		let mut vec = Vec::new();
		vec.push(2);
		
		// Ensure new task can be created with [signer, requirements vector, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), vec, 7, DEADLINE));
	});
}

#[test]
fn increase_task_count_when_creating_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), Vec::new()));

		let mut vec = Vec::new();
		vec.push(2);
		
		// Ensure new task can be created with [signer, requirements vector, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), vec, 7, DEADLINE));

		// Assert that count is incremented by 1 after task creation
		assert_eq!(Task::task_count(), 1);
	});
}

#[test]
fn increase_task_count_when_creating_two_tasks(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		let mut vec2 = Vec::new();
		vec2.push(7);
		
		// Ensure new task can be created with [signer, requirements vector, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), vec1, 7, DEADLINE));
		assert_ok!(Task::create_task(Origin::signed(1), vec2, 99, DEADLINE));

		// Assert that count is incremented to 2 after task creation
		assert_eq!(Task::task_count(), 2);
	});
}

#[test]
fn cant_own_more_tax_than_max_tasks(){
	new_test_ext().execute_with( || {

		// TODO: use MaxTasksOwned instead of hardcoded values;

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), Vec::new()));

		// Create 77 tasks  ExceedMaxTasksOwned
		for n in 0..77 {
			
			let mut vec1 = Vec::new();
			vec1.push(n);
			
			// Ensure new task can be created with [signer, requirements vector, budget, deadline]
			assert_ok!(Task::create_task(Origin::signed(1), vec1, 7, DEADLINE));
		}	

		// Assert that count is incremented to 2 after task creation
		assert_eq!(Task::task_count(), 77);

		let mut vec2 = Vec::new();
		vec2.push(7);

		// Assert that when creating the 77 Task, Error is thrown
		assert_noop!(Task::create_task(Origin::signed(1), vec2, 7, DEADLINE), Error::<Test>::ExceedMaxTasksOwned);

	});

}

#[test]
fn assign_task_to_current_owner(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(10), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		assert_ok!(Task::create_task(Origin::signed(10), vec1, 7, DEADLINE));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];
		let task = Task::tasks(hash).expect("should found the task");

		assert_eq!(task.current_owner, 10);
	});
}

#[test]
fn verify_inputs_outputs_to_tasks(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(10), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		assert_ok!(Task::create_task(Origin::signed(10), vec1, 7, DEADLINE));

		// Get task through the hash
		let hash = Task::tasks_owned(10)[0];
		let task = Task::tasks(hash).expect("should found the task");

		// Ensure that task properties are assigned correctly
		assert_eq!(task.current_owner, 10);
		assert_eq!(task.budget, 7);
	});
}

#[test]
fn start_tasks_assigns_new_current_owner(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), Vec::new()));
		assert_ok!(Profile::create_profile(Origin::signed(2), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, requirements vector, budget]
		assert_ok!(Task::create_task(Origin::signed(1), vec1, 7, DEADLINE));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));
		
		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);

	});
}

#[test]
fn start_tasks_assigns_task_to_volunteer(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, requirements vector, budget]
		assert_ok!(Task::create_task(Origin::signed(1), vec1, 7, DEADLINE));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));
		
		// Ensure when task is started it is assigned to volunteer (user 2)
		assert_eq!(task.volunteer, 1);
		assert_eq!(Task::tasks_owned(2).len(), 1);
		assert_eq!(Task::tasks_owned(1).len(), 0);
	});
}

#[test]
fn completing_tasks_assigns_new_current_owner(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, requirements vector, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), vec1, 7, DEADLINE));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));
		
		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);
		
		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure that the ownership is reversed again
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);
		assert_eq!(Task::tasks_owned(2).len(), 0);
	});
}

#[test]
fn only_creator_deletes_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), Vec::new()));
		assert_ok!(Profile::create_profile(Origin::signed(2), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, requirements vector, budget]
		assert_ok!(Task::create_task(Origin::signed(1), vec1, 7, DEADLINE));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));
		
		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);
		
		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure that the ownership is reversed again
		assert_eq!(Task::tasks_owned(1).len(), 1);
		assert_eq!(Task::tasks_owned(2).len(), 0);

		// Ensure task is removed by task creator (user 1)
		assert_noop!(Task::remove_task(Origin::signed(2), hash), Error::<Test>::OnlyInitiatorClosesTask);
		assert_ok!(Task::remove_task(Origin::signed(1), hash));
	});
}

#[test]
fn only_started_task_can_be_completed(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), Vec::new()));
		assert_ok!(Profile::create_profile(Origin::signed(2), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, requirements vector, budget, deadline]
		assert_ok!(Task::create_task(Origin::signed(1), vec1, 7, DEADLINE));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		assert_noop!(Task::complete_task(Origin::signed(2), hash), Error::<Test>::NoPermissionToComplete);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));
		
		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));
	});
}

#[test]
fn when_task_is_removed_ownership_is_cleared(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), Vec::new()));
		assert_ok!(Profile::create_profile(Origin::signed(2), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, requirements vector, budget]
		assert_ok!(Task::create_task(Origin::signed(1), vec1, 7, DEADLINE));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);
		assert_eq!(Task::tasks_owned(1).len(), 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));
		
		// Ensure when task is started user1 has 0 tasks, and user2 has 1
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 1);
		
		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure that the ownership is reversed again
		assert_eq!(Task::tasks_owned(1).len(), 1);
		assert_eq!(Task::tasks_owned(2).len(), 0);

		// Ensure task is removed by task creator (user 1)
		assert_ok!(Task::remove_task(Origin::signed(1), hash));

		// Ensure ownership of task is cleared
		assert_eq!(Task::tasks_owned(1).len(), 0);
		assert_eq!(Task::tasks_owned(2).len(), 0);
	});
}


#[test]
fn decrease_task_count_when_removing_task(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), Vec::new()));
		
		let mut vec = Vec::new();
		vec.push(2);
		
		// Ensure new task can be created with [signer, requirements vector, budget]
		assert_ok!(Task::create_task(Origin::signed(1), vec, 8, DEADLINE));

		// Get hash of task owned
		let hash = Task::tasks_owned(1)[0];
		let _task = Task::tasks(hash).expect("should found the task");

		// Removing task decreases count
		assert_ok!(Task::remove_task(Origin::signed(1), hash));
		assert_eq!(Task::task_count(), 0);
	});
}


// <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<  Integration tests  >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 
 

#[test]
fn increase_profile_reputation_when_task_completed(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), Vec::new()));
		assert_ok!(Profile::create_profile(Origin::signed(2), Vec::new()));

		let mut vec1 = Vec::new();
		vec1.push(2);

		// Ensure new task can be created with [signer, requirements vector, budget]
		assert_ok!(Task::create_task(Origin::signed(1), vec1, 7, DEADLINE));

		// Ensure new task is assigned to new current_owner (user 1)
		let hash = Task::tasks_owned(1)[0];
		let task = Task::tasks(hash).expect("should found the task");
		assert_eq!(task.current_owner, 1);

		// Ensure task is started by new current_owner (user 2)
		assert_ok!(Task::start_task(Origin::signed(2), hash));

		// Ensure task is completed by current current_owner (user 2)
		assert_ok!(Task::complete_task(Origin::signed(2), hash));

		// Ensure task is removed by task creator (user 1)
		assert_ok!(Task::remove_task(Origin::signed(1), hash));

		let profile1 = Profile::profiles(1).expect("should find the profile");
		let profile2 = Profile::profiles(2).expect("should find the profile");

		// Ensure that the reputation has been added to both profiles
		assert_eq!(profile1.reputation, 1);
		assert_eq!(profile2.reputation, 1);

	});
}

#[test]
fn only_add_reputation_when_task_has_been_completed(){
	new_test_ext().execute_with( || {

		// Profile is necessary for task creation
		assert_ok!(Profile::create_profile(Origin::signed(1), Vec::new()));

		let mut vec = Vec::new();
		vec.push(2);
		
		// Ensure new task can be created with [signer, requirements vector, budget]
		assert_ok!(Task::create_task(Origin::signed(1), vec, 8, DEADLINE));

		// Get hash of task owned
		let hash = Task::tasks_owned(1)[0];
		let _task = Task::tasks(hash).expect("should found the task");

		// Removing task decreases count
		assert_ok!(Task::remove_task(Origin::signed(1), hash));
		assert_eq!(Task::task_count(), 0);

		// Reputation should remain 0 since the task was removed without being completed
		let profile = Profile::profiles(1).expect("should find the profile");
		assert_eq!(profile.reputation, 0);
	});
}
