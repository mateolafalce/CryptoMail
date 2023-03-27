use anchor_lang::prelude::*;
use instructions::*;
use crate::errors::ErrorCode;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("GJebPEsVvfuW1sgLSFWudJfzhQvj9ERuKfLmdn64bXb8");

#[program]
pub mod crypto_mail {
    use super::*;
    pub fn init_crypto_mail(
        ctx: Context<InitCryptoMail>
    ) -> Result<()> {
        instructions::init_crypto_mail::init_crypto_mail(
            ctx
        )
    }
    pub fn send_mail(
        ctx: Context<SendMail>,
        mail: String,
        len: u16,
    ) -> Result<()> {
        instructions::send_mail::send_mail(
            ctx,
            mail,
            len,
        )
    }
}
