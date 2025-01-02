use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata as Metaplex,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::{utils::*, BondingCurve, PlatformConfig};

#[derive(Accounts)]
#[instruction(create_token_and_bonding_curve_params: CreateTokenAndBondingCurveParams)]
pub struct CreateTokenAndBondingCurve<'info> {
    #[account(
        seeds = [constants::seeds::PLATFORM_CONFIG],
        bump = platform_config.bump,
    )]
    pub platform_config: Box<Account<'info, PlatformConfig>>,

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        seeds = [
            constants::seeds::MINT,
            create_token_and_bonding_curve_params.name.as_bytes(),
        ],
        bump,
        mint::decimals = constants::general::TOKEN_DECIMALS,
        mint::authority = bonding_curve
    )]
    pub mint: Box<Account<'info, Mint>>,

    /// CHECK: New Metaplex Account being created
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(
        init,
        payer = creator,
        seeds = [
            constants::seeds::BONDING_CURVE,
            mint.key().as_ref(),
        ],
        bump,
        space = constants::general::ANCHOR_DISCRIMINATOR_SIZE + BondingCurve::INIT_SPACE
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,

    #[account(
        init,
        payer = creator,
        seeds = [
            constants::seeds::BONDING_CURVE_MINT_TOKEN_ACCOUNT,
            mint.key().as_ref()
        ],
        bump,
        token::mint = mint,
        token::authority = bonding_curve
    )]
    pub bonding_curve_mint_token_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metaplex>,
    pub rent: Sysvar<'info, Rent>,
}

impl CreateTokenAndBondingCurve<'_> {
    pub fn create_token_and_bonding_curve(
        ctx: &mut Context<CreateTokenAndBondingCurve>,
        create_token_and_bonding_curve_params: CreateTokenAndBondingCurveParams,
    ) -> Result<()> {
        let mint_key = ctx.accounts.mint.key().clone();

        let bonding_curve_seed = &[
            constants::seeds::BONDING_CURVE,
            mint_key.as_ref(),
            &[ctx.bumps.bonding_curve],
        ];
        let bonding_curve_signer = [&bonding_curve_seed[..]];
        let token_data = DataV2 {
            name: create_token_and_bonding_curve_params.name,
            symbol: create_token_and_bonding_curve_params.symbol,
            uri: create_token_and_bonding_curve_params.uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };
        let metadata_creation_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                payer: ctx.accounts.creator.to_account_info(),
                update_authority: ctx.accounts.bonding_curve.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
                mint_authority: ctx.accounts.bonding_curve.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            &bonding_curve_signer,
        );
        create_metadata_accounts_v3(metadata_creation_ctx, token_data, false, true, None)?;

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.bonding_curve.to_account_info(),
                    to: ctx
                        .accounts
                        .bonding_curve_mint_token_account
                        .to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &bonding_curve_signer,
            ),
            ctx.accounts.platform_config.token_total_supply,
        )?;

        let bonding_curve = &mut ctx.accounts.bonding_curve;

        bonding_curve.creator = ctx.accounts.creator.key();
        bonding_curve.token = ctx.accounts.mint.key();
        bonding_curve.token_total_supply = ctx.accounts.platform_config.token_total_supply;
        bonding_curve.virtual_wsol_amount = ctx.accounts.platform_config.virtual_wsol_amount;
        bonding_curve.target_wsol_amount = ctx.accounts.platform_config.target_wsol_amount;
        bonding_curve.current_token_reserve = bonding_curve.token_total_supply;
        bonding_curve.current_wsol_reserve = bonding_curve.virtual_wsol_amount;
        bonding_curve.bump = ctx.bumps.bonding_curve;
        bonding_curve.mint_bump = ctx.bumps.mint;
        bonding_curve.bonding_curve_mint_token_account_bump =
            ctx.bumps.bonding_curve_mint_token_account;

        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateTokenAndBondingCurveParams {
    name: String,
    symbol: String,
    uri: String,
}
