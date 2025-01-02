use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{utils::*, BondingCurve, PlatformConfig};

#[derive(Accounts)]
#[instruction(_token_name: String)]
pub struct CreateRaydiumPool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[constants::seeds::PLATFORM_CONFIG],
        bump
    )]
    pub platform_config: Box<Account<'info, PlatformConfig>>,

    #[account(
        address = utils::convert_str_to_pubkey(constants::general::WSOL_MINT_ACCOUNT)
    )]
    pub wsol_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [constants::seeds::PLATFORM_WSOL_TOKEN_ACCOUNT],
        bump = platform_config.platform_wsol_token_account_bump,
        token::mint = wsol_mint,
        token::authority = platform_wsol_token_account,
    )]
    pub platform_wsol_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            constants::seeds::MINT,
            _token_name.as_bytes()
        ],
        bump = bonding_curve.mint_bump,
        mint::authority = bonding_curve
    )]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [
            constants::seeds::BONDING_CURVE,
            mint.key().as_ref(),
        ],
        bump = bonding_curve.bump,
    )]
    pub bonding_curve: Box<Account<'info, BondingCurve>>,

    #[account(
        mut,
        seeds = [
            constants::seeds::BONDING_CURVE_MINT_TOKEN_ACCOUNT,
            mint.key().as_ref()
        ],
        bump = bonding_curve.bonding_curve_mint_token_account_bump,
        token::mint = mint,
        token::authority = bonding_curve
    )]
    pub bonding_curve_mint_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: Raydium V5 program
    pub raydium_v5_program: UncheckedAccount<'info>,

    // Account for position mint
    #[account[mut]]
    pub position_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: Safe
    #[account(mut)]
    pub burn_account: UncheckedAccount<'info>,

    /// CHECK: Which config the pool belongs to.
    pub amm_config: UncheckedAccount<'info>,

    /// CHECK: Pool vault and lp mint authority
    // #[account(seeds = [AUTH_SEED.as_bytes()], bump)]
    pub authority: UncheckedAccount<'info>,

    /// CHECK: Initialize an account to store the pool state
    // #[account(
    //     init,
    //     seeds = [
    //         POOL_SEED.as_bytes(),
    //         amm_config.key().as_ref(),
    //         token_0_mint.key().as_ref(),
    //         token_1_mint.key().as_ref(),
    //     ],
    //     bump,
    //     payer = creator,
    //     space = PoolState::LEN
    // )]
    #[account(mut)]
    pub pool_state: UncheckedAccount<'info>,

    /// CHECK: Pool lp mint
    // #[account(
    //     init,
    //     seeds = [POOL_LP_MINT_SEED.as_bytes(), pool_state.key().as_ref()],
    //     bump,
    //     mint::decimals = 9,
    //     mint::authority = authority,
    //     payer = creator,
    //     mint::token_program = token_program
    // )]
    #[account(mut)]
    pub lp_mint: UncheckedAccount<'info>,

    /// Creator token0 account
    #[account(
            init,
            payer = signer,
            token::mint = wsol_mint,
            token::authority = bonding_curve,
        )]
    pub creator_token_0: Box<Account<'info, TokenAccount>>,

    /// Creator token1 account
    #[account(
            mut,
            token::mint = mint,
            token::authority = bonding_curve,
        )]
    pub creator_token_1: Box<Account<'info, TokenAccount>>,

    /// CHECK: Creator lp token account
    // #[account(
    //     init,
    //     associated_token::mint = lp_mint,
    //     associated_token::authority = creator,
    //     payer = creator,
    //     token::token_program = token_program
    // )]
    #[account(mut)]
    pub creator_lp_token: UncheckedAccount<'info>,

    /// CHECK: Token_0 vault for the pool
    // #[account(
    //     mut,
    //     seeds = [
    //         POOL_VAULT_SEED.as_bytes(),
    //         pool_state.key().as_ref(),
    //         token_0_mint.key().as_ref()
    //     ],
    //     bump,
    // )]
    #[account(mut)]
    pub token_0_vault: UncheckedAccount<'info>,

    /// CHECK: Token_1 vault for the pool
    // #[account(
    //     mut,
    //     seeds = [
    //         POOL_VAULT_SEED.as_bytes(),
    //         pool_state.key().as_ref(),
    //         token_1_mint.key().as_ref()
    //     ],
    //     bump,
    // )]
    #[account(mut)]
    pub token_1_vault: UncheckedAccount<'info>,

    /// CHECK: Create pool fee account
    // #[account(
    //     mut,
    //     address= create_pool_fee_reveiver::id(),
    // )]
    #[account(mut)]
    pub create_pool_fee: UncheckedAccount<'info>,

    /// CHECK: An account to store oracle observations
    #[account(mut)]
    pub observation_state: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl CreateRaydiumPool<'_> {
    pub fn create_raydium_pool(ctx: &mut Context<CreateRaydiumPool>) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;
        let bonding_curve = &ctx.accounts.bonding_curve;

        require!(
            bonding_curve.launched,
            errors::CustomErrors::BondingCurveNotFilledYet
        );

        let wsol_amount = bonding_curve.current_wsol_reserve
            - bonding_curve.virtual_wsol_amount
            - platform_config.migration_fee;
        let token_amount = utils::get_amount_using_spot_price(
            &(wsol_amount as u128),
            &(bonding_curve.current_wsol_reserve as u128),
            &(bonding_curve.current_token_reserve as u128),
        );

        platform_config.accumulated_wsol_fees += platform_config.migration_fee;

        let cpi_program = ctx.accounts.raydium_v5_program.to_account_info();
        let cpi_accounts = raydium_cp_swap::cpi::accounts::Initialize {
            creator: ctx.accounts.signer.to_account_info(),
            amm_config: ctx.accounts.amm_config.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            pool_state: ctx.accounts.pool_state.to_account_info(),
            token_0_mint: ctx.accounts.wsol_mint.to_account_info(),
            token_1_mint: ctx.accounts.mint.to_account_info(),
            lp_mint: ctx.accounts.lp_mint.to_account_info(),
            creator_token_0: ctx.accounts.creator_token_0.to_account_info(),
            creator_token_1: ctx.accounts.creator_token_1.to_account_info(),
            creator_lp_token: ctx.accounts.creator_lp_token.to_account_info(),
            token_0_vault: ctx.accounts.token_0_vault.to_account_info(),
            token_1_vault: ctx.accounts.token_1_vault.to_account_info(),
            create_pool_fee: ctx.accounts.create_pool_fee.to_account_info(),
            observation_state: ctx.accounts.observation_state.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            token_0_program: ctx.accounts.token_program.to_account_info(),
            token_1_program: ctx.accounts.token_program.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let clock = Clock::get();
        let block_time = clock.unwrap().unix_timestamp as u64;
        raydium_cp_swap::cpi::initialize(cpi_ctx, wsol_amount, token_amount, block_time)?;

        Ok(())
    }
}
