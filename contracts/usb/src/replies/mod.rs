mod instantiate;
mod jackal;

pub use jackal::jackal_reply;
pub use instantiate::instantiate_reply;

pub const INSTANTIATE_REPLY_ID: u64 = 1u64;
pub const JACKAL_MSG_REPLY_ID: u64 = 2u64;
