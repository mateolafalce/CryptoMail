use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn send_mail(
    ctx: Context<SendMail>,
    mail: String,
    len: u16,
) -> Result<()> {
    let (_pda, bump) = Pubkey::find_program_address(&[ctx.accounts.user.key().as_ref()], ctx.program_id);
    require!(mail.len() <= 9923, ErrorCode::LenghtError);
    require!(ctx.accounts.sender.pubkey.key() == ctx.accounts.user.key(), ErrorCode::PubkeyError);
    let mail: &mut Account<Mail> = &mut ctx.accounts.account;
    mail.bump_original = bump;
    mail.receiver = ctx.accounts.receiver.key();
    mail.sender = ctx.account.user.key();
    Ok(())
}

#[derive(Accounts)]
#[instruction(len: u16)]
pub struct SendMail<'info> {
    #[account(init, seeds = [
        user.key().to_bytes().as_ref(),
        receiver.key().to_bytes().as_ref(),
        ], bump, payer = user, space = 8 + 4 + len as usize)]
    pub mail: Account<'info, Mail>,
    #[account(mut, seeds = [sender.pubkey.key().as_ref()], bump = sender.bump_original)]
    pub sender: Account<'info, MailAccount>,
    #[account(mut, seeds = [receiver.pubkey.key().as_ref()], bump = receiver.bump_original)]
    pub receiver: Account<'info, MailAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
