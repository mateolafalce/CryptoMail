use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("The text size you want to enter is too long")]LenghtError,
    #[msg("That is not the sender's pubkey")]PubkeyError,
}