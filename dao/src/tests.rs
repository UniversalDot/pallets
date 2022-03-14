use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};



#[test]
fn can_create_vision() {
	new_test_ext().execute_with(|| {
		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), VISION.to_vec()));
	});
}

#[test]
fn creating_vision_increases_vision_count() {
	new_test_ext().execute_with(|| {
		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), VISION.to_vec()));

		// Ensure vision count is 1
		assert_eq!(Dao::vision_count(), 1);
	});
}

#[test]
fn can_not_create_vision_that_already_exists() {
	new_test_ext().execute_with(|| {
		
		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), VISION.to_vec()));

		// Ensure the DAO can NOT Create create a vision that already exists
		assert_noop!(Dao::create_vision(Origin::signed(1), VISION.to_vec()), Error::<Test>::VisionAlreadyExists);
	});
}

#[test]
fn can_remove_vision() {
	new_test_ext().execute_with(|| {
		
		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), VISION.to_vec()));

		// Ensure the DAO can remove a vision document
		assert_ok!(Dao::remove_vision(Origin::signed(1), VISION.to_vec()));

		// TODO: Enforce stronger check on Vision test
		assert_eq!(Dao::vision(VISION.to_vec()).0, 0);
	});
}

#[test]
fn removing_vision_decreases_vision_count() {
	new_test_ext().execute_with(|| {
		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), VISION.to_vec()));

		// Ensure vision count is 1
		assert_eq!(Dao::vision_count(), 1);

		// Ensure the DAO can remove a vision document
		assert_ok!(Dao::remove_vision(Origin::signed(1), VISION.to_vec()));

		// Ensure vision count is 0
		assert_eq!(Dao::vision_count(), 0);
	});
}

#[test]
fn when_removing_vision_ensure_it_exists() {
	new_test_ext().execute_with(|| {

		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure error is thrown when no vision exists yet
		assert_noop!(Dao::remove_vision(Origin::signed(1), VISION.to_vec()), Error::<Test>::NoSuchVision);
	});
}

#[test]
fn only_vision_owner_can_remove_vision() {
	new_test_ext().execute_with(|| {
		
		// Create Vision Document
		const VISION: &'static [u8] = &[7];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), VISION.to_vec()));

		// Ensure the vision can not be deleted by user who didn't create it. Created with user 1, deleted with 2
		assert_noop!(Dao::remove_vision(Origin::signed(2), VISION.to_vec()), Error::<Test>::NotVisionOwner);
	});
}

#[test]
fn user_can_sign_onto_vision() {
	new_test_ext().execute_with(|| {

		// Create Static Vision
		const VISION: &'static [u8] = &[1];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), VISION.to_vec()));

		// Ensure a user can sign onto vision. 
		assert_ok!(Dao::sign_vision(Origin::signed(1), VISION.to_vec()));

		// Ensure the length of VisionSigner has increased
		assert_eq!(Dao::applicants_to_organization(VISION.to_vec()).len(), 1);
	});
}

#[test]
fn user_can_unsign_from_vision() {
	new_test_ext().execute_with(|| {

		// Create Static Vision
		const VISION: &'static [u8] = &[1];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), VISION.to_vec()));

		// Ensure a user can sign onto vision. 
		assert_ok!(Dao::sign_vision(Origin::signed(2), VISION.to_vec()));

		// Ensure the length of VisionSigners has increased
		assert_eq!(Dao::applicants_to_organization(VISION.to_vec()).len(), 1);

		// Ensure a user can unsign onto vision. 
		assert_ok!(Dao::unsign_vision(Origin::signed(2), VISION.to_vec()));

		// Ensure the length of VisionSigners has increased
		assert_eq!(Dao::applicants_to_organization(VISION.to_vec()).len(), 0);
	});
}

#[test]
fn user_can_sign_onto_vision_if_vision_exists() {
	new_test_ext().execute_with(|| {

		// Create Vision
		const VISION: &'static [u8] = &[1];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), VISION.to_vec()));

		// Ensure Error is thrown if vision doesn't exist when signing
		assert_noop!(Dao::sign_vision(Origin::signed(1), Vec::new()), Error::<Test>::NoSuchVision );

	});
}

#[test]
fn user_can_unsign_from_vision_if_vision_exists() {
	new_test_ext().execute_with(|| {

		// Create Vision
		const VISION: &'static [u8] = &[1];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), VISION.to_vec()));

		// Ensure Error is thrown if vision doesn't exist when unsigning
		assert_noop!(Dao::unsign_vision(Origin::signed(1), Vec::new()), Error::<Test>::NoSuchVision );

	});
}

#[test]
fn user_can_sign_onto_vision_only_if_not_signed_previously() {
	new_test_ext().execute_with(|| {

		// Create Vision Document
		const VISION: &'static [u8] = &[1];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), VISION.to_vec()));

		// Ensure Vision can be signed
		assert_ok!(Dao::sign_vision(Origin::signed(2), VISION.to_vec()));

		// Ensure Error is thrown if vision is already signed
		assert_noop!(Dao::sign_vision(Origin::signed(2), VISION.to_vec()), Error::<Test>::AlreadySigned );

	});
}

#[test]
fn user_can_unsign_from_vision_only_if_signed_previously() {
	new_test_ext().execute_with(|| {

		// Create Vision Document
		const VISION: &'static [u8] = &[1];

		// Ensure the DAO can create a vision document
		assert_ok!(Dao::create_vision(Origin::signed(1), VISION.to_vec()));

		// Ensure Error is thrown if vision has not been signed previously 
		assert_noop!(Dao::unsign_vision(Origin::signed(2), VISION.to_vec()), Error::<Test>::NotSigned );

	});
}

#[test]
fn can_create_an_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[10];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(7), ORG_NAME.to_vec()));

		let org = Dao::organization(ORG_NAME);

		// Ensure the length of organization is equal to 1
		assert_eq!(Dao::organization(ORG_NAME.to_vec()).len(), 1);
		assert_eq!(org, &[7]);
	});
}

#[test]
fn creating_organization_increases_organization_count() {
	new_test_ext().execute_with(|| {
		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Ensure organization count is 1
		assert_eq!(Dao::organization_count(), 1);
	});
}

#[test]
fn can_create_multiple_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization names
		const ORG_NAME1: &'static [u8] = &[7];
		const ORG_NAME2: &'static [u8] = &[8];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME1.to_vec()));

		// Ensure second organization can be created by a different user
		assert_ok!(Dao::create_organization(Origin::signed(2), ORG_NAME2.to_vec()));

		// Ensure each organization was created successfully
		assert_eq!(Dao::organization(ORG_NAME1.to_vec()).len(), 1);
		assert_eq!(Dao::organization(ORG_NAME2.to_vec()).len(), 1);
		
		// Ensure organization count is 2
		assert_eq!(Dao::organization_count(), 2);
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
fn removing_organization_decreases_organization_count() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Ensure the length of organization is equal to 1, and count is 1
		assert_eq!(Dao::organization(ORG_NAME.to_vec()).len(), 1);
		assert_eq!(Dao::organization_count(), 1);

		// Ensure organization can be removed
		assert_ok!(Dao::dissolve_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Ensure the organization has been removed by checking the length
		assert_eq!(Dao::organization(ORG_NAME.to_vec()).len(), 0);

		// Ensure organization count is 0
		assert_eq!(Dao::organization_count(), 0);
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

		// Ensure users can be added to a DAO
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
		
		// Ensure adding existing member throws an error
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

		// Ensure error is thrown when removing members from non-existing organization
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

		// Validate Ensure length of users in org is 2
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

#[test]
fn user_can_view_organization_it_belongs_to_member_of() {
	new_test_ext().execute_with(|| {

		// Create Static Organization names
		const ORG_NAME1: &'static [u8] = &[7];
		const ORG_NAME2: &'static [u8] = &[8];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME1.to_vec()));
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME2.to_vec()));

		// Ensure users can be added to a DAO
		assert_ok!(Dao::add_members(Origin::signed(1), ORG_NAME1.to_vec(), 4));
		assert_ok!(Dao::add_members(Origin::signed(1), ORG_NAME2.to_vec(), 4));

		// Ensure user 4 belongs to two organizations
		assert_eq!(Dao::member_of(4).len(), 2);

	});
}

#[test]
fn user_can_be_removed_from_organization_it_belongs_to_member_of() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME1: &'static [u8] = &[7];
		const ORG_NAME2: &'static [u8] = &[8];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME1.to_vec()));
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME2.to_vec()));

		// Ensure user 4 is member of 0 organizations
		assert_eq!(Dao::member_of(4).len(), 0);

		// Ensure user 4 can be added to a DAO
		assert_ok!(Dao::add_members(Origin::signed(1), ORG_NAME1.to_vec(), 4));
		assert_ok!(Dao::add_members(Origin::signed(1), ORG_NAME2.to_vec(), 4));

		// Ensure the user 4 is member of 2 organizations
		assert_eq!(Dao::member_of(4).len(), 2);

		// User can be removed from organization
		assert_ok!(Dao::remove_members(Origin::signed(1), ORG_NAME1.to_vec(), 4));

		// Ensure user 4 belongs to 1 organizations
		assert_eq!(Dao::member_of(4).len(), 1);	

	});
}

#[test]
fn user_can_not_be_removed_from_organization_that_does_not_exist() {
	new_test_ext().execute_with(|| {

		// Create Static Organization names
		const ORG_NAME1: &'static [u8] = &[7];
		const ORG_NAME2: &'static [u8] = &[8];
		const ORG_NAME3: &'static [u8] = &[1];

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME1.to_vec()));
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME2.to_vec()));

		// Ensure user 4 is member of 0 organizations
		assert_eq!(Dao::member_of(4).len(), 0);

		// Ensure user 4 can be added to 2 organizations
		assert_ok!(Dao::add_members(Origin::signed(1), ORG_NAME1.to_vec(), 4));
		assert_ok!(Dao::add_members(Origin::signed(1), ORG_NAME2.to_vec(), 4));

		// Ensure the user 4 is member of 2 organizations
		assert_eq!(Dao::member_of(4).len(), 2);

		// Throws error when attempting to remove user from non-existing organization
		assert_noop!(Dao::remove_members(Origin::signed(1), ORG_NAME3.to_vec(), 4), Error::<Test>::InvalidOrganization );

		// Ensure user 4 belongs to 1 organizations
		assert_eq!(Dao::member_of(4).len(), 2);	

	});
}

// < -------- Integration Tests -------------> 

#[test]
fn can_add_tasks_to_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];
		let hash = sp_core::H256::zero();

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Ensure tasks can be added to a DAO
		assert_ok!(Dao::add_tasks(Origin::signed(1), ORG_NAME.to_vec(), hash));

		// Ensure the organization has 1 task
		assert_eq!(Dao::organization_tasks(ORG_NAME.to_vec()).len(), 1);

	});
}

#[test]
fn can_add_task_to_organization_only_once() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];
		let hash = sp_core::H256::zero();

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Add task twice
		assert_ok!(Dao::add_tasks(Origin::signed(1), ORG_NAME.to_vec(), hash));

		// Ensure Error is thrown
		assert_noop!(Dao::add_tasks(Origin::signed(1), ORG_NAME.to_vec(), hash), Error::<Test>::TaskAlreadyExists);

		// Check only 1 task was added
		assert_eq!(Dao::organization_tasks(ORG_NAME.to_vec()).len(), 1);

	});
}

#[test]
fn only_creator_can_add_task_to_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];
		let hash = sp_core::H256::zero();

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Throw error if another than Creator is trying to add members
		assert_noop!(Dao::add_tasks(Origin::signed(2), ORG_NAME.to_vec(), hash), Error::<Test>::NotOrganizationCreator);
	});
}

#[test]
fn can_not_add_tasks_to_organization_that_does_not_exist() {
	new_test_ext().execute_with(|| {

		// Create zero hash
		let hash = sp_core::H256::zero();

		// Throw error if organization is not found
		assert_noop!(Dao::add_tasks(Origin::signed(2), Vec::new(), hash), Error::<Test>::InvalidOrganization);
	});
}

#[test]
fn can_remove_task_from_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];
		let hash = sp_core::H256::zero();

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Add task to organization
		assert_ok!(Dao::add_tasks(Origin::signed(1), ORG_NAME.to_vec(), hash));

		// Check only 1 task was added
		assert_eq!(Dao::organization_tasks(ORG_NAME.to_vec()).len(), 1);

		// Remove task from organization
		assert_ok!(Dao::remove_tasks(Origin::signed(1), ORG_NAME.to_vec(), hash));

		// Ensure the organization tasks are 0
		assert_eq!(Dao::organization_tasks(ORG_NAME.to_vec()).len(), 0);

	});
}

#[test]
fn can_remove_task_from_organization_only_once() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];
		let hash = sp_core::H256::zero();

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Add task to organization
		assert_ok!(Dao::add_tasks(Origin::signed(1), ORG_NAME.to_vec(), hash));

		// Remove task from organization
		assert_ok!(Dao::remove_tasks(Origin::signed(1), ORG_NAME.to_vec(), hash));

		// Ensure once the task has been removed, error is thrown
		assert_noop!(Dao::remove_tasks(Origin::signed(1), ORG_NAME.to_vec(), hash), Error::<Test>::TaskNotExist);

	});
}

#[test]
fn only_creator_can_remove_task_to_organization() {
	new_test_ext().execute_with(|| {

		// Create Static Organization name
		const ORG_NAME: &'static [u8] = &[7];
		let hash = sp_core::H256::zero();

		// Ensure organization can be created
		assert_ok!(Dao::create_organization(Origin::signed(1), ORG_NAME.to_vec()));

		// Throw error if another than Creator is trying to remove members
		assert_noop!(Dao::remove_tasks(Origin::signed(2), ORG_NAME.to_vec(), hash), Error::<Test>::NotOrganizationCreator);
	});
}

#[test]
fn can_not_remove_tasks_from_organization_that_does_not_exist() {
	new_test_ext().execute_with(|| {

		// Create zero hash
		let hash = sp_core::H256::zero();

		// Throw error if organization is not found
		assert_noop!(Dao::remove_tasks(Origin::signed(2), Vec::new(), hash), Error::<Test>::InvalidOrganization);
	});
}