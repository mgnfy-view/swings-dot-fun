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
}
