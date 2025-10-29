use frame_support::{assert_noop, assert_ok};

mod mock;

use mock::*;
use pallet_template::{Error, Event};

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);

        // Store a value
        assert_ok!(TemplateModule::store_something(RuntimeOrigin::signed(1), 42));

        // Verify the value was stored
        assert_eq!(TemplateModule::something(), Some(42));

        // Verify event was emitted
        System::assert_last_event(
            Event::SomethingStored {
                value: 42,
                who: 1,
            }
            .into(),
        );
    });
}

#[test]
fn correct_error_for_none_value() {
    new_test_ext().execute_with(|| {
        // Attempt to increment without storing a value first
        assert_noop!(
            TemplateModule::increment(RuntimeOrigin::signed(1)),
            Error::<Test>::NoneValue
        );
    });
}

#[test]
fn increment_works() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);

        // Store initial value
        assert_ok!(TemplateModule::store_something(RuntimeOrigin::signed(1), 10));

        // Increment the value
        assert_ok!(TemplateModule::increment(RuntimeOrigin::signed(1)));

        // Verify the value was incremented
        assert_eq!(TemplateModule::something(), Some(11));

        // Verify event was emitted
        System::assert_last_event(
            Event::SomethingStored {
                value: 11,
                who: 1,
            }
            .into(),
        );
    });
}

#[test]
fn set_balance_requires_root() {
    new_test_ext().execute_with(|| {
        // Non-root account should fail
        assert_noop!(
            TemplateModule::set_balance(RuntimeOrigin::signed(1), 2, 100),
            sp_runtime::DispatchError::BadOrigin
        );

        // Root should succeed
        assert_ok!(TemplateModule::set_balance(RuntimeOrigin::root(), 2, 100));
        assert_eq!(TemplateModule::balance_of(2), 100);
    });
}

#[test]
fn balance_storage_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Set balance using root
        assert_ok!(TemplateModule::set_balance(RuntimeOrigin::root(), 1, 50));

        // Verify balance was stored
        assert_eq!(TemplateModule::balance_of(1), 50);

        // Verify event was emitted
        System::assert_last_event(
            Event::BalanceSet {
                who: 1,
                balance: 50,
            }
            .into(),
        );

        // Verify default value for unset account
        assert_eq!(TemplateModule::balance_of(999), 0);
    });
}
