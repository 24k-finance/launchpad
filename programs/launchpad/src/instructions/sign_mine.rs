use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token::{ Mint, Token, TokenAccount};

use crate::{
    error::ErrorCode,
    events::MineSigned,
    state::{LaunchPool, MineApplication},
};

#[derive(Accounts)]
#[instruction(mine_code: String, bump_launch_pool: u8)]
pub struct SignMine<'info> {
    #[account(
        mut,
        seeds = [b"mine_app", mine_code.as_bytes()],
        bump = application.bump,
        constraint = application.audit_result @ ErrorCode::ApplicationNotApproved,
        constraint = !application.is_signed @ ErrorCode::ApplicationAlreadySigned,
        constraint = application.owner == owner.key()
    )]
    pub application: Account<'info, MineApplication>,

    /// 项目方签署人（前端用户）
    #[account(mut)]
    pub owner: Signer<'info>,

    /// launch_pool PDA，作为 vault authority
    #[account(
        init,
        payer = owner,
        space = 8 + LaunchPool::LEN,
        seeds = [b"launch_pool", mine_code.as_bytes()],
        bump
    )]
    pub launch_pool: Account<'info, LaunchPool>,

    /// 由 launch_pool（PDA）作为 authority 的资金池账户
    #[account(
        init,
        payer = owner,
        token::mint = payment_mint,
        token::authority = launch_pool,
        seeds = [b"payment_vault", mine_code.as_bytes()],
        bump
    )]
    pub payment_vault: Account<'info, TokenAccount>,

    /// 支付用的 Token（比如 USDC）
    pub payment_mint: Account<'info, Mint>,

    /// 要模拟转账的目标账户（比如平台、测试收款人）
    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,

    /// 基础程序
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler_sign(
    ctx: Context<SignMine>,
    mine_code: String,
    bump_launch_pool: u8,
) -> Result<()> {
    // Step 1: 时间校验
    let now = Clock::get()?.unix_timestamp;
    require!(
        now >= ctx.accounts.application.start_date &&
        now <= ctx.accounts.application.end_date,
        ErrorCode::InvalidTimeRange
    );

    // Step 2: 初始化 launch_pool 内容
    initialize_pool(
        &mut ctx.accounts.launch_pool,
        &ctx.accounts.application,
        &ctx.accounts.owner,
        mine_code.clone(),
        bump_launch_pool,
    )?;

    // Step 3: 标记 application 已签署
    let app = &mut ctx.accounts.application;
    app.is_signed = true;
    app.sign_date = Some(now);

    // // Step 4: 模拟 launch_pool 从 vault 向 receiver 转账


    msg!("{} {}",&mine_code,bump_launch_pool);
    // transfer_from_vault_with_signed(&ctx, &mine_code, bump_launch_pool, 1_000_000)?; // 1 USDC

    // Step 5: 事件
    emit!(MineSigned {
        mine_code,
        pool_address: ctx.accounts.launch_pool.key(),
        timestamp: now,
    });

    Ok(())
}

fn initialize_pool(
    launch_pool: &mut Account<LaunchPool>,
    application: &Account<MineApplication>,
    owner: &Signer,
    mine_code: String,
    bump: u8,
) -> Result<()> {
    launch_pool.price = 100;
    launch_pool.cap = 1_000_000;
    launch_pool.start_time = application.start_date;
    launch_pool.end_time = application.end_date;
    launch_pool.bump = bump;
    launch_pool.mine_code = mine_code;
    launch_pool.rate = application.rate;
    launch_pool.frozen_month = application.frozen_month;
    launch_pool.authority = owner.key();
    Ok(())
}

