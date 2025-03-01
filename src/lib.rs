use near_sdk::near;
use near_sdk_contract_tools::{ft::*, hook::Hook};

#[derive(FungibleToken)]
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
}

pub struct NonTransferrableHook;

impl<C> Hook<C, Nep141Transfer<'_>> for NonTransferrableHook {
    fn hook<R>(_contract: &mut C, _args: &Nep141Transfer<'_>, _f: impl FnOnce(&mut C) -> R) -> R {
        panic!("Token is non-transferable");
    }
}
