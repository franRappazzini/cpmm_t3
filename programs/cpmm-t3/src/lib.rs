pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use states::*;

declare_id!("H2x2uMrSpkaPgkDViEwMiJ2JkmTsuxXqoEtsz2Recf21");

#[program]
pub mod cpmm_t3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, protocol_fee_bps: u16, fee_bps: u16) -> Result<()> {
        ctx.accounts
            .process_ix(protocol_fee_bps, fee_bps, &ctx.bumps)
    }

    pub fn create_pool(ctx: Context<CreatePool>, amount_a: u64, amount_b: u64) -> Result<()> {
        ctx.accounts.process_ix(amount_a, amount_b, &ctx.bumps)
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        pool_id: u64,
        amount_a: u64,
        amount_b: u64,
        min_lp_out: u64,
    ) -> Result<()> {
        ctx.accounts
            .process_ix(pool_id, amount_a, amount_b, min_lp_out)
    }

    pub fn redeem_lp(ctx: Context<RedeemLp>, pool_id: u64, lp_amount: u64) -> Result<()> {
        ctx.accounts.process_ix(pool_id, lp_amount)
    }
}
