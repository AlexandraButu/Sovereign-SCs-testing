#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod burn_tokens;
pub mod common;
pub mod events;
pub mod mint_tokens;
pub mod token_handler_proxy;

#[multiversx_sc::contract]
pub trait TokenHandler:
    mint_tokens::MintTokens
    + burn_tokens::BurnTokens
    + utils::UtilsModule
    + common::storage::CommonStorage
{
    #[init]
    fn init(&self, header_verifier_address: ManagedAddress, chain_prefix: ManagedBuffer) {
        require!(
            self.blockchain()
                .is_smart_contract(&header_verifier_address),
            "Header Verifier address is not a SC contract address"
        );

        self.header_verifier_address().set(header_verifier_address);
        self.sov_prefix().set(chain_prefix);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
