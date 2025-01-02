import * as anchor from "@coral-xyz/anchor";

const wsolMint = new anchor.web3.PublicKey("So11111111111111111111111111111111111111112");

const tokenMetadataProgram = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

const decimals = 9;

const seeds = {
    platformConfig: "platform_config",
    platformWsolTokenAccount: "platform_wsol_token_account",
    mint: "mint",
    bondingCurve: "bonding_curve",
    bondingCurveMintTokenAccount: "bonding_curve_mint_token_account",
    metadata: "metadata",
};

const errors = {
    valueZero: "Value zero",
    invalidFeeValueInBps: "Invalid fee value in bps",
    invalidPlatformConfigParams: "Invalid platform config params",
};

export { wsolMint, tokenMetadataProgram, decimals, seeds, errors };
