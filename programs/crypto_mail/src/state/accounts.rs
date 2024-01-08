use crate::utils::constants::ANCHOR_BUFFER;
use anchor_lang::prelude::*;

#[account]
pub struct MailAccount {
    pub bump_original: u8, // 1
    pub pubkey: Pubkey,    // 32
    pub total_mails: u16,  // 4
}

#[account]
pub struct Mail {
    pub bump_original: u8, // 1
    pub mail: String,      // 4 + text
    pub sender: Pubkey,    // 32
    pub receiver: Pubkey,  // 32
}

impl Mail {
    pub const SIZE: usize = 1 + 4 + 32 + 32 + ANCHOR_BUFFER;

    pub fn set_bump_original(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn set_receiver(&mut self, receiver: Pubkey) {
        self.receiver = receiver;
    }

    pub fn set_sender(&mut self, sender: Pubkey) {
        self.sender = sender;
    }

    pub fn set_mail(&mut self, mail: String) {
        self.mail = mail;
    }
}

impl MailAccount {
    pub const SIZE: usize = 1 + 32 + 4 + ANCHOR_BUFFER;

    pub fn set_bump_original(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn set_authority(&mut self, auth: Pubkey) {
        self.pubkey = auth;
    }

    pub fn init_mails(&mut self) {
        self.total_mails = 0;
    }

    pub fn add_mails(&mut self) {
        self.total_mails += 1;
    }
}
