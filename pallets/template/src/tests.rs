use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, assert_err, 	dispatch::{DispatchInfo, GetDispatchInfo},
};
use sp_runtime::transaction_validity::TransactionValidityError;
use sp_runtime::transaction_validity::InvalidTransaction;
use crate as pallet_template;
use crate::PhantomData;
use crate::mock;
use crate::TransactionLimitChecker;
use sp_runtime::traits::SignedExtension;
#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());

	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}

#[test]
fn signed_ext_watch_dummy_works() {
	new_test_ext().execute_with(|| {
		let call = pallet_template::Call::do_something { something: 10 }.into();
		let info = DispatchInfo::default();

		let checker = TransactionLimitChecker::<mock::Test>(PhantomData);
        let account_id = 1;  // Assuming `1` is a valid account ID.

        // Simulate calling the transaction five times successfully
        for _ in 0..6 {
            assert_eq!(
                checker.validate(&account_id, &call, &info, 150).unwrap().priority,
                u64::MAX,
                "Transaction should be validated with max priority on valid calls"
            );
        }

        // Check that the sixth call fails due to transaction limit exhaustion
        assert_eq!(
            checker.validate(&account_id, &call, &info, 150),
            Err(InvalidTransaction::ExhaustsResources.into()),
            "Sixth transaction should fail as it exceeds the transaction limit"
        );
		// assert_eq!(
		// 	TransactionLimitChecker::<mock::Test>(PhantomData)
		// 		.validate(&1, &call, &info, 150)
		// 		.unwrap()
		// 		.priority,
		// 	u64::MAX,
		// );
		// assert_eq!(
		// 	TransactionLimitChecker::<mock::Test>(PhantomData).validate(&1, &call, &info, 250),
		// 	InvalidTransaction::ExhaustsResources.into(),
		// );
	})
}
