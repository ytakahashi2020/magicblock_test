use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::{delegate, DelegationProgram};
use ephemeral_rollups_sdk::cpi::delegate_account;

declare_id!("81qSgM1GMoRsMzbzedNYpHKst93FKcUqXkTALUP4BFxa");

pub const TEST_PDA_SEED: &[u8] = b"test-pda";

#[delegate]
#[program]
pub mod anchor_counter {
    use super::*;

    /// Initialize the counter.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }

    /// Increment the counter.
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        if counter.count > 1000 {
            counter.count = 0;
        }
        Ok(())
    }
    /// Delegate the account to the delegation program
    pub fn delegate(ctx: Context<DelegateInput>) -> Result<()> {
        let pda_seeds: &[&[u8]] = &[TEST_PDA_SEED];

        delegate_account(
            &ctx.accounts.payer,
            &ctx.accounts.pda,
            &ctx.accounts.owner_program,
            &ctx.accounts.buffer,
            &ctx.accounts.delegation_record,
            &ctx.accounts.delegation_metadata,
            &ctx.accounts.delegation_program,
            &ctx.accounts.system_program,
            pda_seeds,
            0,
            3_000,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 30 + 8, seeds = [TEST_PDA_SEED], bump)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Account for the increment instruction.
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [TEST_PDA_SEED], bump)]
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct DelegateInput<'info> {
    pub payer: Signer<'info>,
    /// CHECK The pda to delegate
    #[account(mut)]
    pub pda: AccountInfo<'info>,
    /// CHECK: The program that owns the pda
    #[account(address = crate::id())]
    pub owner_program: AccountInfo<'info>,
    /// CHECK The temporary buffer account used during delegation
    #[account(
        mut, seeds = [ephemeral_rollups_sdk::consts::BUFFER, pda.key().as_ref()],
        bump, seeds::program = crate::id()
    )]
    pub buffer: AccountInfo<'info>,
    /// CHECK: The delegation record account
    #[account(
        mut, seeds = [ephemeral_rollups_sdk::consts::DELEGATION_RECORD, pda.key().as_ref()],
        bump, seeds::program = delegation_program.key()
    )]
    pub delegation_record: AccountInfo<'info>,
    /// CHECK: The delegation metadata account
    #[account(
        mut, seeds = [ephemeral_rollups_sdk::consts::DELEGATION_METADATA, pda.key().as_ref()],
        bump, seeds::program = delegation_program.key()
    )]
    pub delegation_metadata: AccountInfo<'info>,
    pub delegation_program: Program<'info, DelegationProgram>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub count: u64,
}