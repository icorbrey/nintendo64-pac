//! # Program Counter Wrapper
//!
//! This module wraps the Nintendo 64's program counter registers and provides
//! type- and memory safe ways of interacting with it.

use tock_registers::{
    interfaces::Writeable, register_bitfields, register_structs, registers::ReadWrite,
};

use crate::HARDWARE;

/// The static address of the Nintendo 64's program counter registers.
#[cfg(target_vendor = "nintendo64")]
const PC_REGS_BASE: usize = 0x0408_0000;

#[cfg(not(target_vendor = "nintendo64"))]
lazy_static::lazy_static! {
    /// A registry access analogue for development and testing.
    ///
    /// We have to modify the registry access mechanism when building for
    /// architectures other than the Nintendo 64 since the production registry
    /// access mechanism accesses a static memory location. This is disallowed
    /// on modern operating systems, so we instead dynamically allocate the
    /// memory so that testing and development can occur.
    static ref REGISTERS: ProgramCounterRegisters = unsafe { std::mem::zeroed() };
}

#[non_exhaustive]
pub struct ProgramCounter;

impl ProgramCounter {
    /// Gets a reference to the program counter registers.
    #[cfg(target_vendor = "nintendo64")]
    fn registers<'a>(&self) -> &'a ProgramCounterRegisters {
        unsafe { &mut *(PC_REGS_BASE as *mut ProgramCounterRegisters) }
    }

    /// Gets a reference to the program counter registers.
    #[cfg(not(target_vendor = "nintendo64"))]
    fn registers<'a>(&self) -> &'a REGISTERS {
        &REGISTERS
    }

    /// Returns ownership of the program counter registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.program_counter.drop(self) }
    }

    pub fn halt_rsp(&self) -> &Self {
        self.registers()
            .program_counter
            .write(Control::HALT_RSP::SET);
        self
    }
}

// This is a hack to allow code to run for development.
#[cfg(not(target_vendor = "nintendo64"))]
unsafe impl Sync for ProgramCounterRegisters {}

register_structs! {
    ProgramCounterRegisters {
        (0x0000 => pub program_counter: ReadWrite<u32, Control::Register>),
        (0x0004 => pub imem_bist: ReadWrite<u32, ImemBist::Register>),
        (0x0008 => @END),
    }
}

register_bitfields! {
    u32,

    Control [
        PROGRAM_COUNTER          OFFSET(0)  NUMBITS(12) [],
        HALT_RSP                 OFFSET(12) NUMBITS(1)  [],
    ],

    ImemBist [
        BIST_CHECK               OFFSET(0)  NUMBITS(1)  [],
        BIST_GO                  OFFSET(1)  NUMBITS(1)  [],
        BIST_CLEAR               OFFSET(2)  NUMBITS(1)  [],
        BIST_DONE                OFFSET(2)  NUMBITS(1)  [],
        BIST_FAIL                OFFSET(3)  NUMBITS(4)  [],
    ]
}
