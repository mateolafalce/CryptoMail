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
    // Check if the mail length exceeds the maximum allowed length
    require!(mail.len() <= 9923, ErrorCode::LenghtError);
    // Check if the sender's public key matches the user's key
    require!(ctx.accounts.sender.pubkey.key() == ctx.accounts.user.key(), ErrorCode::PubkeyError);
    let mail: &mut Account<Mail> = &mut ctx.accounts.mail;
    mail.bump_original = bump;
    // Set the receiver's key in the mail account
    mail.receiver = ctx.accounts.receiver.key();
    // Set the sender's key in the mail account
    mail.sender = ctx.account.user.key();
    Ok(())
}

#[derive(Accounts)]
#[instruction(len: u16)]
pub struct SendMail<'info> {
    #[account(
        init,
        // Seeds for initializing the mail account
        seeds = [
            user.key().to_bytes().as_ref(),
            receiver.key().to_bytes().as_ref(),
        ],
        bump,
        // Payer of the account initialization transaction
        payer = user,
        // Required space for the mail account
        space = 8 + 4 + len as usize
    )]
    pub mail: Account<'info, Mail>,
    #[account(
        mut,
        seeds = [sender.pubkey.key().as_ref()],
        bump = sender.bump_original
    )]
    pub sender: Account<'info, MailAccount>,
    #[account(
        mut,
        seeds = [receiver.pubkey.key().as_ref()],
        bump = receiver.bump_original
    )]
    pub receiver: Account<'info, MailAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
