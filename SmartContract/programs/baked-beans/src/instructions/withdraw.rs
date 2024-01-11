use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;
use solana_program::{program::invoke_signed, system_instruction};
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      mut,
      seeds = [GLOBAL_STATE_SEED],
      bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    /// CHECK: this should be checked with address in global_state
    pub vault: AccountInfo<'info>,

    #[account(mut, address = global_state.treasury)]
    /// CHECK: this should be checked with address in global_state
    pub treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handle(ctx: Context<Withdraw>) -> Result<()> {
    let accts = ctx.accounts;

    // lamports should be bigger than zero to prevent rent exemption
    let rent = Rent::default();
    let required_lamports = rent
        .minimum_balance(0)
        .max(1)
        .saturating_sub(accts.vault.to_account_info().lamports());

    // send all to treasury
    let bump = ctx.bumps.get("vault").unwrap();
    invoke_signed(
        &system_instruction::transfer(
            &accts.vault.key(), 
            &accts.treasury.key(), 
            accts.vault.lamports()
                .checked_sub(required_lamports)
                .unwrap()
                .checked_sub(10000000)
                .unwrap()
        ),
        &[
            accts.vault.to_account_info().clone(),
            accts.treasury.clone(),
            accts.system_program.to_account_info().clone(),
        ],
        &[&[VAULT_SEED, &[*bump]]],
    )?;

    require!(
        **accts.vault.lamports.borrow() > required_lamports,
        BeanError::InsufficientAmount
    );
    Ok(())
}
