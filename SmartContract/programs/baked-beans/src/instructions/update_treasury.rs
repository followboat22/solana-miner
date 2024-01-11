use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateTreasury<'info> {
    #[account(
        mut,
        address = global_state.authority
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    pub system_program: Program<'info, System>,
}

impl<'info> UpdateTreasury<'info> {
    pub fn validate(&self) -> Result<()> {
        require!(
            self.global_state.authority.eq(&self.authority.key()),
            BeanError::NotAllowedAuthority
        );
        Ok(())
    }
}

/// UpdateTreasury Staking Program for the first time
/// to init global state with some data for validation
///
#[access_control(ctx.accounts.validate())]
pub fn handle(ctx: Context<UpdateTreasury>, new_treasury: Pubkey) -> Result<()> {
    let accts = ctx.accounts;
    accts.global_state.treasury = new_treasury;

    Ok(())
}
