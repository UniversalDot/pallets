use crate::{ *,};
use frame_support::{assert_noop, assert_ok };
use sp_io::TestExternalities;
use crate::{mock::{Event, MapSet, Origin, System, TestRuntime}};

struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let storage = frame_system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap();
		let mut ext = TestExternalities::from(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

#[test]
fn add_member_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MapSet::add_member(Origin::signed(1)));

		let expected_event = Event::map_set(RawEvent::MemberAdded(1));

		assert_eq!(System::events()[0].event, expected_event,);

		assert!(<Members<TestRuntime>>::contains_key(1));
	})
}

#[test]
fn cant_add_duplicate_members() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MapSet::add_member(Origin::signed(1)));

		assert_noop!(
			MapSet::add_member(Origin::signed(1)),
			Error::<TestRuntime>::AlreadyMember
		);
	})
}

#[test]
fn cant_exceed_max_members() {
	ExternalityBuilder::build().execute_with(|| {
		// Add 16 members, reaching the max
		for i in 0..16 {
			assert_ok!(MapSet::add_member(Origin::signed(i)));
		}

		// Try to add the 17th member exceeding the max
		assert_noop!(
			MapSet::add_member(Origin::signed(16)),
			Error::<TestRuntime>::MembershipLimitReached
		);
	})
}

#[test]
fn remove_member_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MapSet::add_member(Origin::signed(1)));
		assert_ok!(MapSet::remove_member(Origin::signed(1)));

		// check correct event emission
		let expected_event = Event::map_set(RawEvent::MemberRemoved(1));

		assert_eq!(System::events()[1].event, expected_event,);

		// check storage changes
		assert!(!<Members<TestRuntime>>::contains_key(1));
	})
}

#[test]
fn remove_member_handles_errors() {
	ExternalityBuilder::build().execute_with(|| {
		// 2 is NOT previously added as a member
		assert_noop!(
			MapSet::remove_member(Origin::signed(2)),
			Error::<TestRuntime>::NotMember
		);
	})
}

#[test]
fn add_value_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MapSet::set_value(Origin::signed(1), 6));

		let expected_event = Event::map_set(RawEvent::ValueAdded);

		assert_eq!(System::events()[0].event, expected_event,);
	})
}

#[test]
fn get_value_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MapSet::get_value(Origin::signed(1)));
	})
}

#[test]
fn remove_value_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MapSet::set_value(Origin::signed(1), 7));
		assert_ok!(MapSet::remove_value(Origin::signed(1)));

		let expected_event1 = Event::map_set(RawEvent::ValueAdded);
		let expected_event2 = Event::map_set(RawEvent::ValueRemoved);

		//System events holds an array of all events thrown. (Indexed starting at 0)
		assert_eq!(System::events()[0].event, expected_event1,);
		assert_eq!(System::events()[1].event, expected_event2,);
	})
}

#[test]
fn add_hash_map_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(MapSet::set_hash_map(Origin::signed(1), 7));
	})
}