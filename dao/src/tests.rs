use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};



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

		// Ensure error is thrown when no vision exists yet
		assert_noop!(Dao::remove_vision(Origin::signed(1), vec2), Error::<Test>::NoSuchVision);
	});
}

#[test]
fn only_vision_owner_can_remove_vision() {
	new_test_ext().execute_with(|| {
		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), vec));

		let mut vec = Vec::new();
		vec.push(7);


		// Ensure the vision can not be deleted by user who didn't create it. Created with user 1, deleted with 2
		assert_noop!(Dao::remove_vision(Origin::signed(2), vec), Error::<Test>::NotVisionOwner);
	});
}

#[test]
fn user_can_sign_onto_vision() {
	new_test_ext().execute_with(|| {

		let mut vec = Vec::new();
		vec.push(7);

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), vec));

		let mut vec = Vec::new();
		vec.push(7);

		// Ensure a user can sign onto vision. 
		assert_ok!(Dao::sign_vision(Origin::signed(1), vec));
	});
}

#[test]
fn can_create_an_organization() {
	new_test_ext().execute_with(|| {

		let mut org_name = Vec::new();
		org_name.push(9);

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), org_name));

		let mut org_name = Vec::new();
		org_name.push(9);

		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::organization(org_name).len(), 1);
	});
}

#[test]
fn can_create_multiple_organization() {
	new_test_ext().execute_with(|| {

		let mut org_name = Vec::new();
		org_name.push(9);

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), org_name));

		let mut org_name = Vec::new();
		org_name.push(9);

		// Ensure second organization can be created by a different user
		assert_ok!(Dao::create_organization(Origin::signed(2), org_name));

		let mut org_name = Vec::new();
		org_name.push(9);

		// Ensure the length of organization is equal to 2
		assert_eq!(Dao::organization(org_name).len(), 2);
	});
}


#[test]
fn can_remove_an_organization() {
	new_test_ext().execute_with(|| {

		let mut org_name = Vec::new();
		org_name.push(9);

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), org_name));

		let mut org_name = Vec::new();
		org_name.push(9);

		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::organization(org_name).len(), 1);

		let mut org_name = Vec::new();
		org_name.push(9);

		// Ensure organization can be removed
		assert_ok!(Dao::dissolve_organization(Origin::signed(1), org_name));

		let mut org_name = Vec::new();
		org_name.push(9);

		assert_eq!(Dao::organization(org_name).len(), 0);

	});
}

#[test]
fn only_creator_can_remove_their_organization() {
	new_test_ext().execute_with(|| {

		let mut org_name = Vec::new();
		org_name.push(9);

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), org_name));


		let mut org_name = Vec::new();
		org_name.push(9);

		// Ensure organization can't be removed by another member. Only creator can remove their own org
		assert_noop!(Dao::dissolve_organization(Origin::signed(2), org_name), Error::<Test>::NotOrganizationCreator);

		let mut org_name = Vec::new();
		org_name.push(9);

		assert_eq!(Dao::organization(org_name).len(), 1);

	});
}
