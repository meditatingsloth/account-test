use anchor_lang::prelude::*;

declare_id!("2SAUzLPDFwkedHtamM19uvC2BC12y6VKsoW2a91bR2KZ");

#[program]
pub mod account_test {
    use super::*;

    pub fn initialize<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
        ctx.accounts.my_account.bump = *ctx.bumps.get("my_account").ok_or(Error::BumpSeedNotInHashMap)?;

        Ok(())
    }

    pub fn update<'info>(ctx: Context<'_, '_, '_, 'info, Update<'info>>) -> Result<()> {
        // This works
        // ctx.accounts.my_account.flag = true;

        // This doesn't work
        let my_account =
            &mut Account::<'info, MyAccount>::try_from(&ctx.accounts.my_account.to_account_info())?;
        my_account.flag = true;
        my_account.exit(&ID)?;

        Ok(())
    }
}
#[error_code]
pub enum Error {
    #[msg("Bump seed not in hash map")]
    BumpSeedNotInHashMap = 0,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 2, 
        seeds = [b"my_account".as_ref()],
        bump,
    )]
    pub my_account: Box<Account<'info, MyAccount>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds = [b"my_account".as_ref()], bump = my_account.bump)]
    pub my_account: Box<Account<'info, MyAccount>>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyAccount {
    pub bump: u8,
    pub flag: bool,
}
