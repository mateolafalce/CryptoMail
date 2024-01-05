use crate::state::accounts::*;
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn init_crypto_mail_(ctx: Context<InitCryptoMail>) -> Result<()> {
    let signer: Pubkey = ctx.accounts.user.key();
    let program_id: Pubkey = ctx.program_id.key();
    let account: &mut Account<MailAccount> = &mut ctx.accounts.account;
    let (_pda, bump) = Pubkey::find_program_address(&[&signer.to_bytes()], &program_id);
    account.set_bump_original(bump);
    account.set_authority(signer);
    Ok(())
}

#[derive(Accounts)]
pub struct InitCryptoMail<'info> {
    #[account(init, seeds = [&user.key().to_bytes()], bump, payer = user, space = MailAccount::SIZE)]
    pub account: Account<'info, MailAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
