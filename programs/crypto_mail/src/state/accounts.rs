use crate::utils::constants::ANCHOR_BUFFER;
use anchor_lang::prelude::*;

#[account]
pub struct MailAccount {
    pub bump_original: u8, // 1
    pub pubkey: Pubkey,    // 32
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
}

impl MailAccount {
    pub const SIZE: usize = 1 + 32 + ANCHOR_BUFFER;

    pub fn set_bump_original(&mut self, bump: u8) {
        self.bump_original = bump;
    }

    pub fn set_authority(&mut self, auth: Pubkey) {
        self.pubkey = auth;
    }
}
