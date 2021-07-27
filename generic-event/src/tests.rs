use crate::{mock::*};
use frame_support::{assert_ok};
use sp_io::TestExternalities;
use crate::{RawEvent};

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
fn test() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(GenericEvent::do_something(Origin::signed(1), 32));

		// construct event that should be emitted in the method call directly above
		let expected_event = Event::generic_event(RawEvent::EmitInput(1, 32));

		// iterate through array of `EventRecord`s
		assert_eq!(System::events()[0].event, expected_event);
	})
}

#[test]
fn test_second_event() {
    ExternalityBuilder::build().execute_with(|| {
		assert_ok!(GenericEvent::do_something(Origin::signed(2), 77));

		// construct event that should be emitted in the method call directly above
		let expected_event = Event::generic_event(RawEvent::EmitInput(2, 77));

		// iterate through array of `EventRecord`s
		assert_eq!(System::events()[0].event, expected_event);
	})
}