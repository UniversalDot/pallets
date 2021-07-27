use crate::{mock::*};
use crate::{Error, RawEvent};
use frame_support::{assert_err, assert_ok};
use sp_io::TestExternalities;

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
fn set_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(SimpleMap::set_single_entry(Origin::signed(1), 19));

		let expected_event = Event::simple_map(RawEvent::EntrySet(1, 19));

		assert_eq!(System::events()[0].event, expected_event);
	})
}

#[test]
fn get_throws() {
	ExternalityBuilder::build().execute_with(|| {
		assert_err!(
			SimpleMap::get_single_entry(Origin::signed(2), 3),
			Error::<TestRuntime>::NoValueStored
		);
	})
}

#[test]
fn get_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(SimpleMap::set_single_entry(Origin::signed(2), 19));
		assert_ok!(SimpleMap::get_single_entry(Origin::signed(1), 2));

		let expected_event = Event::simple_map(RawEvent::EntryGot(1, 19));

		assert_eq!(System::events()[1].event, expected_event);

		// Ensure storage is still set
		assert_eq!(SimpleMap::simple_map(2), 19);
	})
}

#[test]
fn take_throws() {
	ExternalityBuilder::build().execute_with(|| {
		assert_err!(
			SimpleMap::take_single_entry(Origin::signed(2)),
			Error::<TestRuntime>::NoValueStored
		);
	})
}

#[test]
fn take_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(SimpleMap::set_single_entry(Origin::signed(2), 19));
		assert_ok!(SimpleMap::take_single_entry(Origin::signed(2)));

		let expected_event = Event::simple_map(RawEvent::EntryTaken(2, 19));

		assert_eq!(System::events()[1].event, expected_event);

		// Assert storage has returned to default value (zero)
		assert_eq!(SimpleMap::simple_map(2), 0);
	})
}

#[test]
fn increase_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(SimpleMap::set_single_entry(Origin::signed(2), 19));
		assert_ok!(SimpleMap::increase_single_entry(Origin::signed(2), 2));

		let expected_event = Event::simple_map(RawEvent::EntryIncreased(2, 19, 21));

		assert_eq!(System::events()[1].event, expected_event);

		// Assert storage map entry has been increased
		assert_eq!(SimpleMap::simple_map(2), 21);
	})
}
