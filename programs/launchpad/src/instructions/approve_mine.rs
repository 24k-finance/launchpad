use anchor_lang::prelude::*;
use crate::events::MineApproved;
use crate::state::MineApplication;
use crate::constants::ADMIN_PUBKEY;

#[derive(Accounts)]
#[instruction(mine_code: String)]
pub struct ApproveMine<'info> {
    #[account(
        mut,
        seeds = [b"mine_app", mine_code.as_bytes()],
        bump = application.bump,
        constraint = !application.audit_result
    )]
    pub application: Account<'info, MineApplication>,

    #[account(
        constraint = admin.key.to_string() == ADMIN_PUBKEY 
    )]
    pub admin: Signer<'info>,
}

pub fn handler_approve(ctx: Context<ApproveMine>, _mine_code: String) -> Result<()> {
    let app = &mut ctx.accounts.application;
    app.audit_result = true;

    emit!(MineApproved {
        mine_code: app.mine_code.clone(),
        timestamp: Clock::get()?.unix_timestamp
    });

    Ok(())
}