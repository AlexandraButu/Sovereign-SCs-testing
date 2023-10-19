// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Async Callback (empty):               1
// Total number of exported functions:   9

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    chain_config
    (
        init => init
        upgrade => upgrade
        deployBridge => deploy_bridge
        getMinValidators => min_validators
        getMaxValidators => max_validators
        getMinStake => min_stake
        getAdditionalStakeRequired => additional_stake_required
        wasPreviouslySlashed => was_previously_slashed
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
