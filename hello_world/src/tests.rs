use crate::{mock::*};
use frame_system::RawOrigin;
use sp_io::TestExternalities;
use frame_support::{
	assert_noop, assert_ok, dispatch::DispatchError };

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
fn say_hello_works() {
	ExternalityBuilder::build().execute_with(|| {
		assert_ok!(HelloSubstrate::say_hello(Origin::signed(1)));
	})
}

#[test]
fn say_hello_no_root() {
	ExternalityBuilder::build().execute_with(|| {
		assert_noop!(
			HelloSubstrate::say_hello(RawOrigin::Root.into()),
			DispatchError::BadOrigin
		);
	})
}