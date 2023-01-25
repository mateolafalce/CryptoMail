use anchor_lang::prelude::*;

#[account]
pub struct MailAccount { 
    pub bump_original: u8,         // 1
    pub pubkey: Pubkey,            // 32
}

#[account]
pub struct Mail { 
    pub mail: String,         // 4 + len
}

impl MailAccount {
    pub const SIZE: usize =  1 + 32;
}
