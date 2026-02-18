//! Debug utilities for slot hash debugging.
//!
//! Equivalent to sig's src/debug.zig
//!
//! This module provides debug printing functionality for tracking account writes
//! and delta LT hashes during slot processing. To enable debugging for a slot,
//! add it to the SLOTS array.

use solana_account::ReadableAccount;
use solana_clock::Slot;
use solana_pubkey::Pubkey;

/// Slots to print debug info for. Add slots of interest here.
pub const SLOTS: &[Slot] = &[];

/// Accounts to print in full (with data hash instead of full data).
/// For large accounts, add their pubkey here to print a hash of the data
/// instead of the full data bytes.
pub const FULL_ACCOUNTS: &[Pubkey] = &[];

/// Check if the given slot is a debug target slot.
#[inline]
pub fn is_debug_slot(slot: Slot) -> bool {
    SLOTS.contains(&slot)
}

/// Format account information for debug printing.
pub fn fmt_account<A: ReadableAccount>(address: &Pubkey, account: &A) -> String {
    let data = account.data();
    let data_len = data.len();

    if !FULL_ACCOUNTS.contains(address) {
        use solana_sha256_hasher::hashv;
        let data_hash = hashv(&[data]);
        format!(
            "address={} lamports={} owner={} executable={} rent_epoch={} data_len={} data_hash={}",
            address, account.lamports(), account.owner(), account.executable(), account.rent_epoch(), data_len, data_hash,
        )
    } else {
        format!(
            "address={} lamports={} owner={} executable={} rent_epoch={} data_len={} data={:x?}",
            address, account.lamports(), account.owner(), account.executable(), account.rent_epoch(), data_len, data,
        )
    }
}
