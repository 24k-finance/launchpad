use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    state::{MineApplication, LaunchPool},
    error::ErrorCode,
    events::MineSigned,
};

#[derive(Accounts)]
#[instruction(mine_code: String)]
pub struct SignMine<'info> {
    #[account(
        mut,
        seeds = [b"mine_app", mine_code.as_bytes()],
        bump = application.bump,
        constraint = application.audit_result ,
        constraint = !application.is_signed,
        constraint = application.owner == owner.key()
    )]
    pub application: Account<'info, MineApplication>,

    #[account(mut)]
    pub owner: Signer<'info>,

    /// CPI 创建 LaunchPool 所需的 PDA
    #[account(
        init,
        payer = owner,
        space = 8 + LaunchPool::LEN,
        seeds = [b"launch_pool", mine_code.as_bytes()],
        bump
    )]
    pub launch_pool: Account<'info, LaunchPool>,

    /// 支付代币的金库账户（示例：USDC）
    #[account(
        init,
        payer = owner,
        token::mint = payment_mint,
        token::authority = launch_pool,
    )]
    pub payment_vault: Account<'info, TokenAccount>,

    /// 支付代币的 Mint（示例：USDC Mint）
    pub payment_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler_sign(ctx: Context<SignMine>, mine_code: String) -> Result<()> {
    // 1. 验证时间有效性
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        current_time >= ctx.accounts.application.start_date &&
        current_time <= ctx.accounts.application.end_date,
        ErrorCode::InvalidTimeRange
    );

    // 2. 初始化 launch_pool 账户字段
    initialize_pool(&mut ctx.accounts.launch_pool, &ctx.accounts.application, &ctx.accounts.owner)?;

    // 3. 更新签署状态
    let app = &mut ctx.accounts.application;
    app.is_signed = true;
    app.sign_date = Some(current_time);

    emit!(MineSigned {
        mine_code,
        pool_address: ctx.accounts.launch_pool.key(),
        timestamp: current_time
    });

    Ok(())
}

fn initialize_pool(
    launch_pool: &mut Account<LaunchPool>,
    application: &Account<MineApplication>,
    owner: &Signer,
) -> Result<()> {
    launch_pool.price = 100; 
    launch_pool.cap = 1_000_000; 
    launch_pool.start_time = application.start_date;
    launch_pool.end_time = application.end_date;
    launch_pool.bump = application.bump;
    launch_pool.mine_code = application.mine_code.clone();
    launch_pool.rate = application.rate;
    launch_pool.frozen_month = application.frozen_month;
    launch_pool.authority = owner.key();

    Ok(())
}