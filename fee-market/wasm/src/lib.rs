// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           16
// Async Callback (empty):               1
// Total number of exported functions:  18

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    fee_market
    (
        init => init
        upgrade => upgrade
        enableFee => enable_fee
        disableFee => disable_fee
        addFee => add_fee
        removeFee => remove_fee
        getTokenFee => token_fee
        addUsersToWhitelist => add_users_to_whitelist
        removeUsersFromWhitelist => remove_users_from_whitelist
        distributeFees => distribute_fees
        subtractFee => subtract_fee
        getUsersWhitelist => users_whitelist
        addPairs => add_pairs
        removePairs => remove_pairs
        setMinValidSigners => set_min_valid_signers
        addSigners => add_signers
        removeSigners => remove_signers
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
