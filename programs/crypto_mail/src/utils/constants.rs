pub const TOTAL_TEXT_BUFFER: usize = 10_000;
pub const ANCHOR_BUFFER: usize = 8;
pub const MAX_TX_BUFFER: usize = TOTAL_TEXT_BUFFER - (1 + 4 + 32 + 32 + ANCHOR_BUFFER);
