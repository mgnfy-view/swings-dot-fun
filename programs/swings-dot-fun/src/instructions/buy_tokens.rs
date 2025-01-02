use anchor_lang::{
    prelude::*,
    system_program::{transfer as sol_transfer, Transfer as SolTransfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        sync_native, transfer as spl_transfer, Mint, SyncNative, Token, TokenAccount,
        Transfer as SplTransfer,
    },
};

use crate::{utils::*, BondingCurve, PlatformConfig};

#[derive(Accounts)]
#[instruction(_token_name: String, wsol_amount: u64)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

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
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer
    )]
    pub buyer_mint_token_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl BuyTokens<'_> {
    pub fn buy_tokens(
        ctx: &mut Context<BuyTokens>,
        _token_name: String,
        mut wsol_amount: u64,
    ) -> Result<()> {
        let bonding_curve = &mut ctx.accounts.bonding_curve;

        require!(wsol_amount > 0, errors::CustomErrors::ValueZero);
        require!(
            !bonding_curve.launched,
            errors::CustomErrors::BondingCurveFilled
        );

        if bonding_curve.current_wsol_reserve + wsol_amount >= bonding_curve.target_wsol_amount {
            wsol_amount = bonding_curve.target_wsol_amount - bonding_curve.current_wsol_reserve;
            bonding_curve.launched = true;
        }

        let fee_amount = utils::calculate_fee_amount(
            &(wsol_amount as u128),
            &(ctx.accounts.platform_config.trading_fee_in_bps as u128),
        );
        let token_amount_out = utils::calculate_amount_out(
            &(wsol_amount as u128),
            &(bonding_curve.current_wsol_reserve as u128),
            &(bonding_curve.current_token_reserve as u128),
        );
        require!(token_amount_out > 0, errors::CustomErrors::ValueZero);

        bonding_curve.current_wsol_reserve += wsol_amount;
        bonding_curve.current_token_reserve -= token_amount_out;

        ctx.accounts.platform_config.accumulated_wsol_fees += fee_amount;

        sol_transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                SolTransfer {
                    from: ctx.accounts.buyer.to_account_info(),
                    to: ctx.accounts.platform_wsol_token_account.to_account_info(),
                },
            ),
            wsol_amount + fee_amount,
        )?;
        sync_native(CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SyncNative {
                account: ctx.accounts.platform_wsol_token_account.to_account_info(),
            },
        ))?;

        let mint_key = ctx.accounts.mint.key().clone();
        let bonding_curve_seed = &[
            constants::seeds::BONDING_CURVE,
            mint_key.as_ref(),
            &[bonding_curve.bump],
        ];
        let bonding_curve_signer = [&bonding_curve_seed[..]];
        spl_transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                SplTransfer {
                    from: ctx
                        .accounts
                        .bonding_curve_mint_token_account
                        .to_account_info(),
                    to: ctx.accounts.buyer_mint_token_account.to_account_info(),
                    authority: bonding_curve.to_account_info(),
                },
                &bonding_curve_signer,
            ),
            token_amount_out,
        )?;

        Ok(())
    }
}
