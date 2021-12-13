use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};



#[test]
fn can_create_vision() {
	new_test_ext().execute_with(|| {
		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), ORG_NAME.to_vec()));
	});
}

#[test]
fn can_not_create_vision_that_already_exists() {
	new_test_ext().execute_with(|| {
		
		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), ORG_NAME.to_vec()));

		// Ensure the DAO can NOT Create create a vision that already exists
		assert_noop!(Dao::create_vision(Origin::signed(1), ORG_NAME.to_vec()), Error::<Test>::VisionAlreadyExists);
	});
}

#[test]
fn can_remove_vision() {
	new_test_ext().execute_with(|| {
		
		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), ORG_NAME.to_vec()));

		// Ensure the DAO can remove a vision document
		assert_ok!(Dao::remove_vision(Origin::signed(1), ORG_NAME.to_vec()));
	});
}

#[test]
fn when_removing_vision_ensure_it_exists() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure error is thrown when no vision exists yet
		assert_noop!(Dao::remove_vision(Origin::signed(1), ORG_NAME.to_vec()), Error::<Test>::NoSuchVision);
	});
}

#[test]
fn only_vision_owner_can_remove_vision() {
	new_test_ext().execute_with(|| {
		
		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), ORG_NAME.to_vec()));

		// Ensure the vision can not be deleted by user who didn't create it. Created with user 1, deleted with 2
		assert_noop!(Dao::remove_vision(Origin::signed(2), ORG_NAME.to_vec()), Error::<Test>::NotVisionOwner);
	});
}

#[test]
fn user_can_sign_onto_vision() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), ORG_NAME.to_vec()));

		// Ensure a user can sign onto vision. 
		assert_ok!(Dao::sign_vision(Origin::signed(1), ORG_NAME.to_vec()));
	});
}

#[test]
fn can_create_an_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::organization(ORG_NAME.to_vec()).len(), 1);
	});
}

#[test]
fn can_create_multiple_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME1: &'static [u8] = &[7];
		const ORG_NAME2: &'static [u8] = &[8];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME1.to_vec()));

		// Ensure second organization can be created by a different user
		assert_ok!(Dao::create_organization(Origin::signed(2), ORG_NAME2.to_vec()));

		// Ensure the length of organization is equal to 2
		assert_eq!(Dao::organization(ORG_NAME1.to_vec()).len(), 1);
		assert_eq!(Dao::organization(ORG_NAME2.to_vec()).len(), 1);
	});
}


#[test]
fn can_remove_an_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::organization(ORG_NAME.to_vec()).len(), 1);

		// Ensure organization can be removed
		assert_ok!(Dao::dissolve_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Ensure the organization has been removed by checking the length
		assert_eq!(Dao::organization(ORG_NAME.to_vec()).len(), 0);
	});
}

#[test]
fn only_creator_can_remove_their_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Ensure organization can't be removed by another member. Only creator can remove their own org
		assert_noop!(Dao::dissolve_organization(Origin::signed(2), ORG_NAME.to_vec()), Error::<Test>::NotOrganizationCreator);

		// Ensure the organization has not been deleted
		assert_eq!(Dao::organization(ORG_NAME.to_vec()).len(), 1);

	});
}

#[test]
fn can_add_user_to_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		//Ensure users can be added to a DAO
		assert_ok!(Dao::add_members(Origin::signed(1), ORG_NAME.to_vec(), 4));

		// Ensure the organization has 2 members (creator abd user4)
		assert_eq!(Dao::organization(ORG_NAME.to_vec()).len(), 2);

	});
}

#[test]
fn only_creator_can_add_user_to_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Throw error if another than Creator is trying to add members
		assert_noop!(Dao::add_members(Origin::signed(2), ORG_NAME.to_vec(), 4), Error::<Test>::NotOrganizationCreator);
	});
}


#[test]
fn can_only_add_members_if_not_already_in_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Throw error if another than Creator is trying to add members
		assert_ok!(Dao::add_members(Origin::signed(1), ORG_NAME.to_vec(), 2));
		
		// TODO: Fix test and implementation
		assert_noop!(Dao::add_members(Origin::signed(1), ORG_NAME.to_vec(), 2), Error::<Test>::AlreadyMember );
	});
}

#[test]
fn organization_exists_check_before_adding_user_to_org() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1),ORG_NAME.to_vec()));

		// Throw error if org_name is not found
		assert_noop!(Dao::add_members(Origin::signed(1), Vec::new(), 4), Error::<Test>::InvalidOrganization);
	});
}

#[test]
fn only_creator_can_remove_users_from_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1),ORG_NAME.to_vec()));

		// Ensure users can be added to a DAO
		assert_ok!(Dao::add_members(Origin::signed(1), ORG_NAME.to_vec(), 4));

		// When user 2 who didn't create organization tries to remove user, throw error
		assert_noop!(Dao::remove_members(Origin::signed(2), ORG_NAME.to_vec(), 4), Error::<Test>::NotOrganizationCreator );

	});
}

#[test]
fn organization_exists_check_before_removing_user_from_org() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1),ORG_NAME.to_vec()));

		// Throw error if org_name is not found
		assert_ok!(Dao::add_members(Origin::signed(1), ORG_NAME.to_vec(), 4));

		assert_noop!(Dao::remove_members(Origin::signed(1), Vec::new(), 4), Error::<Test>::InvalidOrganization );
	});
}

#[test]
fn can_remove_users_from_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1),ORG_NAME.to_vec()));

		// Ensure users can be added to a DAO
		assert_ok!(Dao::add_members(Origin::signed(1), ORG_NAME.to_vec(), 4));
		assert_ok!(Dao::add_members(Origin::signed(1), ORG_NAME.to_vec(), 5));

		// User can be removed from organization
		assert_ok!(Dao::remove_members(Origin::signed(1), ORG_NAME.to_vec(), 4));

		//  Validate Ensure length of users in org is 2
		assert_eq!(Dao::organization(ORG_NAME.to_vec()).len(), 2);

	});
}

#[test]
fn can_only_remove_users_that_belong_to_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1),ORG_NAME.to_vec()));

		// Ensure users can be added to a DAO
		assert_ok!(Dao::add_members(Origin::signed(1), ORG_NAME.to_vec(), 4));

		// Ensure length of users in org is 2
		assert_eq!(Dao::organization(ORG_NAME.to_vec()).len(), 2);

		// Ensure error is thrown if user is not in organization
		assert_noop!(Dao::remove_members(Origin::signed(1), ORG_NAME.to_vec(), 5), Error::<Test>::NotMember);

	});
}