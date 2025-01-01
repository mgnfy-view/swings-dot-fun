use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;
pub use state::*;
pub use utils::*;

declare_id!("8DNN53jopWc89XcSs5FEm8YncwDmUjMUmnzLg3dUBKA");

#[program]
pub mod swings_dot_fun {
    use super::*;

    pub fn initialize_platform_config(
        mut ctx: Context<InitializePlatformConfig>,
        platform_config_init_params: PlatformConfigInitParams,
    ) -> Result<()> {
        instructions::InitializePlatformConfig::initialize_platform_config(
            &mut ctx,
            platform_config_init_params,
        )?;

        Ok(())
    }

    pub fn set_owner(mut ctx: Context<SetPlatformConfig>, new_owner: Pubkey) -> Result<()> {
        instructions::SetPlatformConfig::set_owner(&mut ctx, new_owner)?;

        Ok(())
    }

    pub fn set_trading_fee_in_bps(
        mut ctx: Context<SetPlatformConfig>,
        new_trading_fee_in_bps: u16,
    ) -> Result<()> {
        instructions::SetPlatformConfig::set_trading_fee_in_bps(&mut ctx, new_trading_fee_in_bps)?;

        Ok(())
    }

    pub fn set_token_total_supply(
        mut ctx: Context<SetPlatformConfig>,
        new_token_total_supply: u64,
    ) -> Result<()> {
        instructions::SetPlatformConfig::set_token_total_supply(&mut ctx, new_token_total_supply)?;

        Ok(())
    }

    pub fn set_virtual_wsol_amount(
        mut ctx: Context<SetPlatformConfig>,
        new_virtual_wsol_amount: u64,
    ) -> Result<()> {
        instructions::SetPlatformConfig::set_virtual_wsol_amount(
            &mut ctx,
            new_virtual_wsol_amount,
        )?;

        Ok(())
    }

    pub fn set_target_wsol_amount(
        mut ctx: Context<SetPlatformConfig>,
        new_target_wsol_amount: u64,
    ) -> Result<()> {
        instructions::SetPlatformConfig::set_virtual_wsol_amount(&mut ctx, new_target_wsol_amount)?;

        Ok(())
    }

    pub fn set_migration_fee(
        mut ctx: Context<SetPlatformConfig>,
        new_migration_fee: u64,
    ) -> Result<()> {
        instructions::SetPlatformConfig::set_migration_fee(&mut ctx, new_migration_fee)?;

        Ok(())
    }

    pub fn withdraw_accumulated_wsol_fees(
        mut ctx: Context<WithdrawAccumulatedWsolFees>,
    ) -> Result<()> {
        instructions::WithdrawAccumulatedWsolFees::withdraw_accumulated_wsol_fees(&mut ctx);

        Ok(())
    }
}
