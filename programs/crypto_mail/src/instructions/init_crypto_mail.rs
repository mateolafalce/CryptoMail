use crate::state::accounts::*;
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn init_crypto_mail_(ctx: Context<InitCryptoMail>) -> Result<()> {
    let signer: Pubkey = ctx.accounts.user.key();
    let program_id: Pubkey = ctx.program_id.key();
    let (pda, bump) = Pubkey::find_program_address(&[&signer.to_bytes()], &program_id);
    require_keys_eq!(pda, ctx.accounts.account.key());
    let account: &mut Account<MailAccount> = &mut ctx.accounts.account;
    account.set_bump_original(bump);
    account.set_authority(signer);
    account.init_mails();
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
