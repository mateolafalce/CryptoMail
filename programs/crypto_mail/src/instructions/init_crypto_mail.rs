use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;

pub fn init_crypto_mail(
    ctx: Context<InitCryptoMail>,
) -> Result<()> {
    let account: &mut Account<MailAccount> = &mut ctx.accounts.account;
    // Generate program-derived address (PDA) and bump seed
    let (_pda, bump) = Pubkey::find_program_address(&[ctx.accounts.user.key().as_ref()], ctx.program_id);
    // Set the bump seed and public key for the CryptoMail account
    account.bump_original = bump;
    account.pubkey = ctx.accounts.user.key();
    Ok(())
}

#[derive(Accounts)]
pub struct InitCryptoMail<'info> {
    // Initialize the MailAccount
    #[account(init, seeds = [user.key().as_ref()], bump, payer = user, space = 8 + MailAccount::SIZE)]
    pub account: Account<'info, MailAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
