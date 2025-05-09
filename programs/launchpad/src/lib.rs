// programs/launchpad/src/lib.rs

use {anchor_lang::prelude::*, instructions::*};

pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

declare_id!("CqcCvZmiLwhgKJvhVzttVct2aHRW4n1dJFNiSpoJQHuq");

#[program]
pub mod launchpad {
    use super::*;

    // Pool instructions

    pub fn stake(
        ctx: Context<Stake>, params: StakeParams
    ) -> Result<()> {
        instructions::handler_stake(ctx, &params)
    }

    pub fn release(
        ctx: Context<Release>
    ) -> Result<()> {
        instructions::handler_release(ctx)
    }

    // Manager instructions
    pub fn apply_mine(
        ctx: Context<ApplyMine>, params: ApplyMineParams
    ) -> Result<()> {
        instructions::handler_apply(ctx, &params)
    }

    pub fn approve_mine(
        ctx: Context<ApproveMine>, mine_code: String
    ) -> Result<()> {
        instructions::handler_approve(ctx, mine_code)
    }

    pub fn sign_mine(
        ctx: Context<SignMine>, mine_code: String
    ) -> Result<()> {
        instructions::handler_sign(ctx, mine_code)
    }
}