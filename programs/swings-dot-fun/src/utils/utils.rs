use anchor_lang::prelude::*;
use std::str::FromStr;

pub fn convert_str_to_pubkey(str: &str) -> Pubkey {
    Pubkey::from_str(str).unwrap()
}
