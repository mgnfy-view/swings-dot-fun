use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer as spl_transfer, Mint, Token, TokenAccount, Transfer as SplTransfer},
};

use crate::{utils::*, BondingCurve, PlatformConfig};

#[derive(Accounts)]
#[instruction(_token_name: String, token_amount: u64)]
pub struct SellTokens<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::seeds::PLATFORM_CONFIG],
        bump = platform_config.bump,
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

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = seller
    )]
    pub seller_mint_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = seller,
        associated_token::mint = wsol_mint,
        associated_token::authority = seller
    )]
    pub seller_wsol_token_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl SellTokens<'_> {
    pub fn sell_tokens(
        ctx: &mut Context<SellTokens>,
        _token_name: String,
        token_amount: u64,
    ) -> Result<()> {
        let bonding_curve = &mut ctx.accounts.bonding_curve;

        require!(token_amount > 0, errors::CustomErrors::ValueZero);
        require!(
            !bonding_curve.launched,
            errors::CustomErrors::BondingCurveFilled
        );

        let wsol_amount_out = utils::calculate_amount_out(
            &((token_amount) as u128),
            &(bonding_curve.current_token_reserve as u128),
            &(bonding_curve.current_wsol_reserve as u128),
        );
        let fee_amount = utils::calculate_fee_amount(
            &(wsol_amount_out as u128),
            &(ctx.accounts.platform_config.trading_fee_in_bps as u128),
        );
        require!(wsol_amount_out > 0, errors::CustomErrors::ValueZero);

        bonding_curve.current_wsol_reserve -= wsol_amount_out;
        bonding_curve.current_token_reserve += token_amount;

        ctx.accounts.platform_config.accumulated_wsol_fees += fee_amount;

        let platform_wsol_token_account_seed = &[
            constants::seeds::PLATFORM_WSOL_TOKEN_ACCOUNT,
            &[ctx
                .accounts
                .platform_config
                .platform_wsol_token_account_bump],
        ];
        let platform_wsol_token_account_signer = [&platform_wsol_token_account_seed[..]];
        spl_transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SplTransfer {
                    from: ctx.accounts.seller_mint_token_account.to_account_info(),
                    to: ctx
                        .accounts
                        .bonding_curve_mint_token_account
                        .to_account_info(),
                    authority: ctx.accounts.seller.to_account_info(),
                },
            ),
            token_amount,
        )?;
        spl_transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                SplTransfer {
                    from: ctx.accounts.platform_wsol_token_account.to_account_info(),
                    to: ctx.accounts.seller_wsol_token_account.to_account_info(),
                    authority: ctx.accounts.platform_wsol_token_account.to_account_info(),
                },
                &platform_wsol_token_account_signer,
            ),
            wsol_amount_out - fee_amount,
        )?;

        Ok(())
    }
}
