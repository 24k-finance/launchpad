//

use crate::error::ErrorCode;
use crate::events::MineApplied;
use crate::state::MineApplication;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(
    mine_code: String,
    name: String,
    operator: String,
    relationship: String,
    scale: String,
    location: String,
    approval1: String,
    approval2: String,
    approval3: String,
    finance_scale: u128,
    currency: String,
    start_date: i64,
    end_date: i64,
    rate: u32,
    frozen_month: u8
)]
pub struct ApplyMine<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + MineApplication::LEN,
        seeds = [b"mine_app", mine_code.as_bytes()],
        bump
    )]
    pub application: Account<'info, MineApplication>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ApplyMineParams {
    pub mine_code: String,
    pub name: String,
    pub operator: String,
    pub relationship: String,
    pub scale: String,
    pub location: String,
    pub approval1: String,
    pub approval2: String,
    pub approval3: String,
    pub finance_scale: u128,
    pub currency: String,
    pub start_date: i64,
    pub end_date: i64,
    pub rate: u32,
    pub frozen_month: u8,
}

pub fn handler_apply(ctx: Context<ApplyMine>, params: &ApplyMineParams) -> Result<()> {
    // Validate string lengths
    let validation_checks = vec![
        (params.mine_code.len(), MineApplication::MAX_MINE_CODE_LEN),
        (params.name.len(), MineApplication::MAX_NAME_LEN),
        (params.operator.len(), MineApplication::MAX_OPERATOR_LEN),
        (params.relationship.len(), MineApplication::MAX_RELATIONSHIP_LEN),
        (params.scale.len(), MineApplication::MAX_SCALE_LEN),
        (params.location.len(), MineApplication::MAX_LOCATION_LEN),
        (params.approval1.len(), MineApplication::MAX_APPROVAL_LEN),
        (params.approval2.len(), MineApplication::MAX_APPROVAL_LEN),
        (params.approval3.len(), MineApplication::MAX_APPROVAL_LEN),
        (params.currency.len(), MineApplication::MAX_CURRENCY_LEN),
    ];

    for (length, max) in validation_checks {
        require!(length <= max, ErrorCode::InvalidLength);
    }

    // Validate numerical ranges
    require!(params.rate <= 100_00, ErrorCode::InvalidRate); // 100.00% max
    require!(params.frozen_month <= 36, ErrorCode::InvalidFrozenPeriod);
    require!(params.end_date > params.start_date, ErrorCode::InvalidTimeRange);
    require!(
        params.start_date > Clock::get()?.unix_timestamp,
        ErrorCode::StartTimeInPast
    );

    // Initialize application
    let app = &mut ctx.accounts.application;
    app.owner = ctx.accounts.owner.key();
    app.mine_code = params.mine_code.clone();
    app.name = params.name.clone();
    app.operator = params.operator.clone();
    app.relationship = params.relationship.clone();
    app.scale = params.scale.clone();
    app.location = params.location.clone();
    app.approval1 = params.approval1.clone();
    app.approval2 = params.approval2.clone();
    app.approval3 = params.approval3.clone();
    app.finance_scale = params.finance_scale;
    app.currency = params.currency.clone();
    app.start_date = params.start_date;
    app.end_date = params.end_date;
    app.rate = params.rate;
    app.frozen_month = params.frozen_month;
    app.audit_result = false;
    app.is_signed = false;
    app.sign_date = None;
    app.bump = ctx.bumps.application;

    // Emit event
    emit!(MineApplied {
        mine_code: app.mine_code.clone(),
        owner: app.owner,
        timestamp: Clock::get()?.unix_timestamp
    });

    Ok(())
}
