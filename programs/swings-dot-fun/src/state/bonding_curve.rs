use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BondingCurve {
    pub creator: Pubkey,
    pub token: Pubkey,
    pub token_total_supply: u64,
    pub virtual_wsol_amount: u64,
    pub target_wsol_amount: u64,
    pub current_token_reserve: u64,
    pub current_wsol_reserve: u64,
    pub launched: bool,
    pub bump: u8,
    pub mint_bump: u8,
    pub bonding_curve_mint_token_account_bump: u8,
}
