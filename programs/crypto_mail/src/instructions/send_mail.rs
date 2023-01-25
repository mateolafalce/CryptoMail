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
    timestamp: i64
) -> Result<()> {
    require!(mail.len() <= 9988, ErrorCode::LenghtError);
    require!(ctx.accounts.sender.pubkey.key() == ctx.accounts.user.key(), ErrorCode::PubkeyError);
    /*let account: &mut Account<MailAccount> = &mut ctx.accounts.account;
    let (_pda, bump) = Pubkey::find_program_address(&[ctx.accounts.user.key().as_ref()], ctx.program_id);
    account.bump_original = bump;*/
    Ok(())
}

#[derive(Accounts)]
#[instruction(len: u16, timestamp: i64)]
pub struct SendMail<'info> {
    #[account(init, seeds = [timestamp.to_le_bytes().as_ref()], bump, payer = user, space = 8 + 4 + len as usize)]
    pub mail: Account<'info, Mail>,
    #[account(mut, seeds = [sender.pubkey.key().as_ref()], bump = sender.bump_original)]
    pub sender: Account<'info, MailAccount>,
    #[account(mut, seeds = [receiver.pubkey.key().as_ref()], bump = receiver.bump_original)]
    pub receiver: Account<'info, MailAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}