//! # Length

use proc_bitfield::bitfield;

bitfield! {
    /// Audio interface length register.
    pub struct AiLenReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub transfer_length_v1: u32 @ 0..15,
        pub transfer_length_v2: u32 @ 0..18,
    }
}
