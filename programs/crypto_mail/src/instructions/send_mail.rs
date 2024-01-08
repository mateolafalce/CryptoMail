use crate::{state::accounts::*, utils::constants::MAX_TX_BUFFER};
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn send_mail_(ctx: Context<SendMail>, mail_txt: String) -> Result<()> {
    let signer: Pubkey = ctx.accounts.user.key();
    let sender: Pubkey = ctx.accounts.sender.pubkey.key();
    let receiver: Pubkey = ctx.accounts.receiver.key();
    let program_id: Pubkey = ctx.program_id.key();
    let (pda, bump): (Pubkey, u8) = Pubkey::find_program_address(
        &[
            &signer.to_bytes(),
            ctx.accounts.sender.total_mails.to_be_bytes().as_ref(),
        ],
        &program_id,
    );
    require_gte!(MAX_TX_BUFFER, mail_txt.len());
    require_keys_eq!(signer, sender);
    require_keys_eq!(pda, ctx.accounts.mail.key());
    let mail: &mut Account<Mail> = &mut ctx.accounts.mail;
    mail.set_bump_original(bump);
    mail.set_receiver(receiver);
    mail.set_sender(signer);
    mail.set_mail(mail_txt);
    let sender_account: &mut Account<MailAccount> = &mut ctx.accounts.sender;
    sender_account.add_mails();
    Ok(())
}

#[derive(Accounts)]
#[instruction(mail_txt: String)]
pub struct SendMail<'info> {
    #[account(
        init,
        seeds = [
            &user.key().to_bytes(),
            sender.total_mails.to_be_bytes().as_ref(),
        ],
        bump,
        payer = user,
        space = Mail::SIZE + mail_txt.len()
    )]
    pub mail: Account<'info, Mail>,
    #[account(
        mut,
        seeds = [&sender.pubkey.key().to_bytes()],
        bump = sender.bump_original
    )]
    pub sender: Account<'info, MailAccount>,
    #[account(
        mut,
        seeds = [&receiver.pubkey.key().to_bytes()],
        bump = receiver.bump_original
    )]
    pub receiver: Account<'info, MailAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
