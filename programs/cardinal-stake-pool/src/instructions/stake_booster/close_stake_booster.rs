use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseStakeBoosterCtx<'info> {
    #[account(mut, close = authority)]
    stake_booster: Box<Account<'info, StakeBooster>>,
    #[account(mut, constraint = stake_booster.stake_pool == stake_pool.key() @ ErrorCode::InvalidStakePool)]
    stake_pool: Box<Account<'info, StakePool>>,
    #[account(mut, constraint = authority.key() == stake_pool.authority @ ErrorCode::InvalidAuthority)]
    authority: Signer<'info>,
}

pub fn handler(_ctx: Context<CloseStakeBoosterCtx>) -> Result<()> {
    return Err(error!(ErrorCode::InstructionNotSupported));

    Ok(())
}
