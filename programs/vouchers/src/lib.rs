use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


// For specifying the different functions / transactions the program can do
#[program]
pub mod vouchers {
    use super::*;

    pub fn store(ctx: Context<Store>, code: String) -> Result<()> {
        // CPI over here

        Ok(())
    }

    pub fn redeem(ctx: Context<Redeem>) -> Result<()> {
        // CPI over here
        Ok(())
    }
}

// For specifying the accounts involved in every transaction
// Note: Space is only computed for accounts
#[derive(Accounts)]
pub struct Store<'info> {
    // Initialize the account with CPI
    // Specify the payer of the account creation
    // 8 byte space for discriminator + Space for account
    // 1 byte space for holder.bump
    #[account(init, payer = authority, space = 8 + 1)]
    pub holder: Account<'info, Holder>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub holder: Account<'info, Holder>,
    pub authority: Signer<'info>
}

// Data Structures / Layout of data in memory
#[account]
pub struct Holder {
    bump: u8
}

// Custom Errors
pub enum PaperWalletError {
    InvalidInstructionCommand
}
