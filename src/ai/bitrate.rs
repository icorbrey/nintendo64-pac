//! # Bitrate

use proc_bitfield::bitfield;

bitfield! {
    /// Audio interface bitrate register.
    pub struct AiBitrateReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub bitrate: u8 [write_only] @ 0..4,
    }
}
