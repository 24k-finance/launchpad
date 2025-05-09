use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{
    state::{LaunchPool, StakeRecord},
    error::ErrorCode,
    events::FundsStaked,
};

#[derive(Accounts)]
#[instruction(
    amount: u64,
    stable_coin: String
)]
pub struct Stake<'info> {
    #[account(
        mut,
        seeds = [b"launch_pool", pool.mine_code.as_bytes()],
        bump = pool.bump
    )]
    pub pool: Account<'info, LaunchPool>,

    /// 动态选择金库账户
    #[account(
        mut,
        constraint =
            (stable_coin == "USDC" && vault.key() == pool.usdc_vault) ||
            (stable_coin == "USDT" && vault.key() == pool.usdt_vault),
        constraint = vault.owner == pool.key()
    )]
    pub vault: Account<'info, TokenAccount>,

    /// 用户支付账户
    #[account(
        mut,
        constraint = from.owner == investor.key(),
        constraint =
            (stable_coin == "USDC" && from.mint == pool.usdc_mint) ||
            (stable_coin == "USDT" && from.mint == pool.usdt_mint)
    )]
    pub from: Account<'info, TokenAccount>,

    /// 投资记录PDA
    #[account(
        init,
        payer = investor,
        space = 8 + StakeRecord::LEN,
        seeds = [
            b"stake",
            pool.mine_code.as_bytes(),
            investor.key().as_ref()
        ],
        bump
    )]
    pub record: Account<'info, StakeRecord>,

    #[account(mut)]
    pub investor: Signer<'info>,

    // 程序依赖
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct StakeParams{
    pub amount: u64,
    pub stable_coin: String,
    pub txn_hash: String,
}

pub fn handler_stake(
    ctx: Context<Stake>,
    params: &StakeParams,
) -> Result<()> {
    // Validate funding period
    let current_time = Clock::get()?.unix_timestamp;
    let pool = &ctx.accounts.pool;
    require!(
        current_time >= pool.start_time && current_time <= pool.end_time,
        ErrorCode::NotInFundingPeriod
    );

    // Validate investment amount
    require!(
        pool.raised
            .checked_add(params.amount)
            .map(|sum| sum <= pool.cap)
            .unwrap_or(false),
        ErrorCode::CapExceeded
    );

    // Execute token transfer
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.investor.to_account_info(),
            },
        ),
        params.amount,
    )?;

    // Update pool state
    let pool = &mut ctx.accounts.pool;
    pool.raised = pool.raised.saturating_add(params.amount);

    // Create stake record
    let record = &mut ctx.accounts.record;
    record.investor = ctx.accounts.investor.key();
    record.mine_code = pool.mine_code.clone();
    record.stable_coin = params.stable_coin.clone();
    record.amount = params.amount.into();
    record.txn_hash = params.txn_hash.clone();
    record.timestamp = current_time;
    record.bump = ctx.bumps.record;

    // Emit staking event
    emit!(FundsStaked {
        investor: ctx.accounts.investor.key(),
        amount: params.amount,
        pool: ctx.accounts.pool.key(),
        timestamp: current_time
    });

    Ok(())
}
