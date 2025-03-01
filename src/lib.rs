use std::borrow::Cow;

use near_sdk::{assert_one_yocto, env, near, AccountId, PanicOnDefault};
use near_sdk_contract_tools::{ft::*, hook::Hook};

#[derive(FungibleToken, PanicOnDefault)]
#[fungible_token(transfer_hook = "NonTransferrableHook")]
#[near(contract_state)]
pub struct Contract {}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let mut contract = Self {};
        Nep148Controller::set_metadata(&mut contract, &ContractMetadata {
            spec: "ft-1.0.0".to_string(),
            name: "Prettear".to_string(),
            symbol: "pTEAR".to_string(),
            decimals: 24,
            icon: Some("https://ipfs.near.social/ipfs/bafkreiceejyh2zn4lygzno6u7v2isaipvanwt5anl5zhw7uzl3o2dphcb4".to_string()),
            reference: None,
            reference_hash: None,
        });
        contract
    }

    #[private]
    pub fn mint(&mut self, amount: u128, to: AccountId) {
        assert_one_yocto();
        Nep141Controller::mint(
            self,
            &Nep141Mint {
                amount,
                receiver_id: Cow::Borrowed(&to),
                memo: None,
            },
        )
        .unwrap();
    }
}

pub struct NonTransferrableHook;

impl<C> Hook<C, Nep141Transfer<'_>> for NonTransferrableHook {
    fn hook<R>(_contract: &mut C, args: &Nep141Transfer<'_>, _f: impl FnOnce(&mut C) -> R) -> R {
        if *args.sender_id != "intear.pool.near"
            && *args.sender_id != "slimedragon.near"
            && *args.sender_id != "intear.near"
        {
            env::panic_str("Token is non-transferable")
        }
        _f(_contract)
    }
}
