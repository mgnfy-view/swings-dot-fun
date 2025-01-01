use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{utils::*, PlatformConfig};

#[derive(Accounts)]
pub struct WithdrawAccumulatedWsolFees<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::seeds::PLATFORM_CONFIG],
        bump = platform_config.bump,
        has_one = owner
    )]
    pub platform_config: Account<'info, PlatformConfig>,

    #[account(
        address = utils::convert_str_to_pubkey(constants::general::WSOL_MINT_ACCOUNT)
    )]
    pub wsol_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [constants::seeds::PLATFORM_WSOL_TOKEN_ACCOUNT],
        bump = platform_config.platform_wsol_token_account_bump,
        token::mint = wsol_mint,
        token::authority = platform_wsol_token_account,
    )]
    pub platform_wsol_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = wsol_mint,
        associated_token::authority = owner,
    )]
    pub owner_wsol_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl WithdrawAccumulatedWsolFees<'_> {
    pub fn withdraw_accumulated_wsol_fees(
        ctx: &mut Context<WithdrawAccumulatedWsolFees>,
    ) -> Result<()> {
        let platform_config = &mut ctx.accounts.platform_config;
        let accumulated_fees = platform_config.accumulated_wsol_fees;

        platform_config.accumulated_wsol_fees = 0;

        let wsol_token_transfer_seed = &[
            constants::seeds::PLATFORM_WSOL_TOKEN_ACCOUNT,
            &[platform_config.platform_wsol_token_account_bump],
        ];
        let wsol_token_transfer_signer = &[&wsol_token_transfer_seed[..]];

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.platform_wsol_token_account.to_account_info(),
                    to: ctx.accounts.owner_wsol_token_account.to_account_info(),
                    authority: ctx.accounts.platform_wsol_token_account.to_account_info(),
                },
                wsol_token_transfer_signer,
            ),
            accumulated_fees,
        )?;

        Ok(())
    }
}
