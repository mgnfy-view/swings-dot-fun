use anchor_lang::prelude::*;
use std::str::FromStr;

use crate::constants::general::BPS;

pub fn convert_str_to_pubkey(str: &str) -> Pubkey {
    Pubkey::from_str(str).unwrap()
}

pub fn calculate_amount_out(amount_in: &u64, reserve_in: &u64, reserve_out: &u64) -> u64 {
    ((amount_in * reserve_out) as u128 / (amount_in + reserve_in) as u128) as u64
}

pub fn calculate_fee_amount(amount: &u64, trading_fee_in_bps: &u64) -> u64 {
    ((amount * trading_fee_in_bps) as u128 / BPS as u128) as u64
}
