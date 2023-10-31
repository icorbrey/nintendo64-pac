//! # DRAM Address

use proc_bitfield::bitfield;

bitfield! {
    /// Audio interface DRAM address register.
    pub struct AiDramAddrReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub dram_addr: u32 [write_only] @ 0..24,
    }
}
