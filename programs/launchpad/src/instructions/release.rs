use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{
    state::LaunchPool,
    error::ErrorCode,
    events::FundsReleased,
};

#[derive(Accounts)]
pub struct Release<'info> {
    #[account(
        mut,
        seeds = [b"launch_pool", pool.mine_code.as_bytes()],
        bump = pool.bump,
        has_one = authority 
    )]
    pub pool: Account<'info, LaunchPool>,

    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = vault.owner == pool.key(),
        constraint =
            (vault.mint == pool.usdc_mint && receiver.mint == pool.usdc_mint) ||
            (vault.mint == pool.usdt_mint && receiver.mint == pool.usdt_mint)
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler_release(ctx: Context<Release>) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let current_time = Clock::get()?.unix_timestamp;

    // Calculate release timestamp
    let release_time = pool.end_time
        .checked_add(pool.frozen_month as i64 * 30 * 86400)
        .ok_or(ErrorCode::ArithmeticOverflow)?;

    require!(current_time >= release_time, ErrorCode::ReleaseNotYet);
    require!(ctx.accounts.vault.amount > 0, ErrorCode::NoFundsToRelease);

    // Prepare transfer
    let seeds = &[
        b"launch_pool",
        pool.mine_code.as_bytes(),
        &[pool.bump]
    ];

    // Execute transfer
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.receiver.to_account_info(),
                authority: pool.to_account_info(),
            },
            &[seeds]
        ),
        ctx.accounts.vault.amount,
    )?;

    // Emit event
    emit!(FundsReleased {
        pool: pool.key(),
        amount: ctx.accounts.vault.amount,
        receiver: ctx.accounts.receiver.key(),
        timestamp: current_time
    });

    Ok(())
}