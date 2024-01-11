use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;
pub mod utils;

use instructions::*;

declare_id!("FPP5bGzTThWsafcpBASF382E7EersYgRyyKvJecSer4J");
#[program]
pub mod baked_beans {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, new_authority: Pubkey) -> Result<()> {
        initialize::handle(ctx, new_authority)
    }

    pub fn buy_eggs(ctx: Context<BuyEggs>, amount: u64) -> Result<()> {
        buy_eggs::handle(ctx, amount)
    }

    pub fn sell_eggs(ctx: Context<SellEggs>) -> Result<()> {
        sell_eggs::handle(ctx)
    }

    pub fn hatch_eggs(ctx: Context<HatchEggs>) -> Result<()> {
        hatch_eggs::handle(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw::handle(ctx)
    }

    pub fn update_treasury(ctx: Context<UpdateTreasury>, new_treasury: Pubkey) -> Result<()> {
        update_treasury::handle(ctx, new_treasury)
    }
}
