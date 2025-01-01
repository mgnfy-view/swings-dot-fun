use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{utils::*, PlatformConfig};

#[derive(Accounts)]
pub struct InitializePlatformConfig<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [constants::seeds::PLATFORM_CONFIG],
        bump,
        space = constants::general::ANCHOR_DISCRIMINATOR_SIZE + PlatformConfig::INIT_SPACE
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    #[account(
        address = utils::convert_str_to_pubkey(constants::general::WSOL_MINT_ACCOUNT)
    )]
    pub wsol_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = owner,
        seeds = [constants::seeds::PLATFORM_WSOL_TOKEN_ACCOUNT],
        bump,
        token::mint = wsol_mint,
        token::authority = platform_wsol_token_account,
    )]
    pub platform_wsol_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl InitializePlatformConfig<'_> {
    pub fn initialize_platform_config(
        ctx: &mut Context<InitializePlatformConfig>,
        platform_config_init_params: PlatformConfigInitParams,
    ) -> Result<()> {
        require!(
            platform_config_init_params.token_total_supply > 0
                && platform_config_init_params.virtual_wsol_amount > 0
                && platform_config_init_params.target_wsol_amount > 0,
            errors::CustomErrors::ValueZero
        );
        require!(
            platform_config_init_params.trading_fee_in_bps <= constants::general::BPS,
            errors::CustomErrors::InvalidFeeValueInBPs
        );
        require!(
            platform_config_init_params.target_wsol_amount
                - platform_config_init_params.virtual_wsol_amount
                > platform_config_init_params.migration_fee,
            errors::CustomErrors::InvalidPlatformConfigParams
        );

        let platform_config = &mut ctx.accounts.platform_config;

        platform_config.owner = platform_config_init_params.owner;
        platform_config.trading_fee_in_bps = platform_config_init_params.trading_fee_in_bps;
        platform_config.token_total_supply = platform_config_init_params.token_total_supply;
        platform_config.virtual_wsol_amount = platform_config_init_params.virtual_wsol_amount;
        platform_config.target_wsol_amount = platform_config_init_params.target_wsol_amount;
        platform_config.migration_fee = platform_config_init_params.migration_fee;
        platform_config.bump = ctx.bumps.platform_config;
        platform_config.platform_wsol_token_account_bump = ctx.bumps.platform_wsol_token_account;

        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PlatformConfigInitParams {
    pub owner: Pubkey,
    pub trading_fee_in_bps: u16,
    pub token_total_supply: u64,
    pub virtual_wsol_amount: u64,
    pub target_wsol_amount: u64,
    pub migration_fee: u64,
}
