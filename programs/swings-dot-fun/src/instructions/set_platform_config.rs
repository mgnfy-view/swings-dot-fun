use anchor_lang::prelude::*;

use crate::{platform_config, utils::*, PlatformConfig};

#[derive(Accounts)]
pub struct SetPlatformConfig<'info> {
    #[account()]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::seeds::PLATFORM_CONFIG],
        bump = platform_config.bump,
        has_one = owner
    )]
    pub platform_config: Account<'info, PlatformConfig>,
}

impl SetPlatformConfig<'_> {
    pub fn set_owner(ctx: &mut Context<SetPlatformConfig>, new_owner: Pubkey) -> Result<()> {
        ctx.accounts.platform_config.owner = new_owner;

        Ok(())
    }

    pub fn set_trading_fee_in_bps(
        ctx: &mut Context<SetPlatformConfig>,
        new_trading_fee_in_bps: u16,
    ) -> Result<()> {
        require!(
            new_trading_fee_in_bps <= constants::general::BPS,
            errors::CustomErrors::InvalidFeeValueInBPs
        );

        ctx.accounts.platform_config.trading_fee_in_bps = new_trading_fee_in_bps;

        Ok(())
    }

    pub fn set_token_total_supply(
        ctx: &mut Context<SetPlatformConfig>,
        new_token_total_supply: u64,
    ) -> Result<()> {
        require!(new_token_total_supply > 0, errors::CustomErrors::ValueZero);

        ctx.accounts.platform_config.token_total_supply = new_token_total_supply;

        Ok(())
    }

    pub fn set_virtual_wsol_amount(
        ctx: &mut Context<SetPlatformConfig>,
        new_virtual_wsol_amount: u64,
    ) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;

        require!(new_virtual_wsol_amount > 0, errors::CustomErrors::ValueZero);
        require!(
            platform_config.target_wsol_amount - new_virtual_wsol_amount
                > platform_config.migration_fee,
            errors::CustomErrors::InvalidPlatformConfigParams
        );

        platform_config.virtual_wsol_amount = new_virtual_wsol_amount;

        Ok(())
    }

    pub fn set_target_wsol_amount(
        ctx: &mut Context<SetPlatformConfig>,
        new_target_wsol_amount: u64,
    ) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;

        require!(new_target_wsol_amount > 0, errors::CustomErrors::ValueZero);
        require!(
            new_target_wsol_amount - platform_config.virtual_wsol_amount
                > platform_config.migration_fee,
            errors::CustomErrors::InvalidPlatformConfigParams
        );

        platform_config.target_wsol_amount = new_target_wsol_amount;

        Ok(())
    }

    pub fn set_migration_fee(
        ctx: &mut Context<SetPlatformConfig>,
        new_migration_fee: u64,
    ) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;

        require!(new_migration_fee > 0, errors::CustomErrors::ValueZero);
        require!(
            platform_config.target_wsol_amount - platform_config.virtual_wsol_amount
                > new_migration_fee,
            errors::CustomErrors::InvalidPlatformConfigParams
        );

        platform_config.migration_fee = new_migration_fee;

        Ok(())
    }
}
