// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           17
// Async Callback (empty):               1
// Promise callbacks:                    1
// Total number of exported functions:  20

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    multi_transfer_esdt
    (
        init => init
        upgrade => upgrade
        setMinValidSigners => set_min_valid_signers
        addSigners => add_signers
        removeSigners => remove_signers
        getAndClearFirstRefundBatch => get_and_clear_first_refund_batch
        setSovToMxTokenId => set_sov_to_mx_token_id
        batchTransferEsdtToken => batch_transfer_esdt_token
        setMaxTxBatchSize => set_max_tx_batch_size
        setMaxTxBatchBlockDuration => set_max_tx_batch_block_duration
        getCurrentTxBatch => get_current_tx_batch
        getFirstBatchAnyStatus => get_first_batch_any_status
        getBatch => get_batch
        getBatchStatus => get_batch_status
        getFirstBatchId => first_batch_id
        getLastBatchId => last_batch_id
        setMaxBridgedAmount => set_max_bridged_amount
        getMaxBridgedAmount => max_bridged_amount
        transfer_callback => transfer_callback
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
