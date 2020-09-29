use crate::tests::{AccountState::*, PlasmaTestBuilder};
use models::tx::{Close, TxSignature};

#[test]
fn expected_fail() {
    let mut tb = PlasmaTestBuilder::new();

    let (_, account, _) = tb.add_account(Locked);
    let close = Close {
        account: account.address,
        nonce: account.nonce,
        signature: TxSignature::default(),
    };

    tb.test_tx_fail(close.into(), "Account closing is disabled");
}
