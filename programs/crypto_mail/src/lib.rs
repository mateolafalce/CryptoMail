use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;
pub mod utils;

use instructions::{init_crypto_mail::init_crypto_mail_, send_mail::send_mail_};

declare_id!("AemmGzS478dTy7FrEtpT6zzexCMT61TxMwpuygKPQ6tU");

#[program]
pub mod crypto_mail {
    use super::*;
    // for accounts
    pub fn init_crypto_mail(ctx: Context<InitCryptoMail>) -> Result<()> {
        init_crypto_mail_(ctx)
    }
    // for emails sharing
    pub fn send_mail(ctx: Context<SendMail>, mail: String) -> Result<()> {
        send_mail_(ctx, mail)
    }
}
