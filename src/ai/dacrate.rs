//! # DAC Rate

use proc_bitfield::bitfield;

bitfield! {
    /// Audio interface DAC rate register.
    pub struct AiDacrateReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub dacrate: u16 [write_only] @ 0..14,
    }
}
