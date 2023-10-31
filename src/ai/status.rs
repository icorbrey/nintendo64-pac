//! # Status

use proc_bitfield::bitfield;

bitfield! {
    /// Audio interface status register.
    pub struct AiStatusReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub ai_busy: bool [read_only] @ 30,
        pub ai_full: bool [read_only] @ 31,
        pub clear_ai_intr: bool [write_only] @ 0,
    }
}
