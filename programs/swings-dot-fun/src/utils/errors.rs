use anchor_lang::prelude::*;

#[error_code]
pub enum CustomErrors {
    #[msg("Value zero")]
    ValueZero,
    #[msg("Invalid fee value in bps")]
    InvalidFeeValueInBPs,
    #[msg("Invalid platform config params")]
    InvalidPlatformConfigParams,
}
