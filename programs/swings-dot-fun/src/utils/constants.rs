pub mod seeds {
    pub const PLATFORM_CONFIG: &[u8] = b"platform_config";
    pub const PLATFORM_WSOL_TOKEN_ACCOUNT: &[u8] = b"platform_wsol_token_account";
    pub const MINT: &[u8] = b"mint";
    pub const BONDING_CURVE: &[u8] = b"bonding_curve";
    pub const BONDING_CURVE_MINT_TOKEN_ACCOUNT: &[u8] = b"bonding_curve_mint_token_account";
}

pub mod general {

    pub const BPS: u16 = 10_000;
    pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
    pub const WSOL_MINT_ACCOUNT: &str = "So11111111111111111111111111111111111111112";
    pub const TOKEN_DECIMALS: u8 = 9;
}
