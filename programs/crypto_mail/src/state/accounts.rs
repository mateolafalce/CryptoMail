use anchor_lang::prelude::*;

#[account]
pub struct MailAccount {
    pub bump_original: u8,         // 1
    pub pubkey: Pubkey,            // 32
}

#[account]
pub struct Mail {
    pub bump_original: u8,          // 1
    pub mail: String,               // 4 + len
    pub sender: Pubkey,             // 32
    pub receiver: Pubkey            // 32
}

impl Mail {
    pub const SIZE: usize =  1 + 4 + 32 + 32;
}

impl MailAccount {
    pub const SIZE: usize =  1 + 32;
}
