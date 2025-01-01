use anchor_lang::prelude::*;

#[account()]
#[derive(InitSpace)]
pub struct PlatformConfig {
    pub owner: Pubkey,
    pub trading_fee_in_bps: u16,
    pub accumulated_wsol_fees: u64,
    pub token_total_supply: u64,
    pub virtual_wsol_amount: u64,
    pub target_wsol_amount: u64,
    pub migration_fee: u64,
    pub bump: u8,
}
