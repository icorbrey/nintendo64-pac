//! # Program Counter Wrapper
//!
//! This module wraps the Nintendo 64's program counter registers and provides
//! type- and memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

use crate::{register_access, HARDWARE};

register_access!(0x0408_0000, Registers);

#[non_exhaustive]
pub struct ProgramCounter {
    /// Contains getters and setters for `SP_IBIST_REG`.
    pub imem_bist: ImemBist,

    /// Contains getters and setters for `SP_PC_REG`.
    pub control: Control,
}

impl ProgramCounter {
    /// Returns ownership of the program counter registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.program_counter.drop(self) }
    }
}

/// A zero-size wrapper around `SP_PC_REG`.
#[non_exhaustive]
pub struct Control;

impl Control {
    pub fn program_counter(&self) -> u32 {
        registers().pc.read(SpPcReg::PROGRAM_COUNTER)
    }

    pub fn set_program_counter(&self, pc: u32) {
        registers().pc.write(SpPcReg::PROGRAM_COUNTER.val(pc))
    }
}

/// A zero-size wrapper around `SP_IBIST_REG`.
#[non_exhaustive]
pub struct ImemBist;

impl ImemBist {
    pub fn get_check(&self) -> bool {
        registers().ibist.is_set(SpIbistReg::BIST_CHECK)
    }

    pub fn get_go(&self) -> bool {
        registers().ibist.is_set(SpIbistReg::BIST_GO)
    }

    pub fn get_done(&self) -> bool {
        registers().ibist.is_set(SpIbistReg::BIST_DONE)
    }

    pub fn get_fail(&self) -> u32 {
        registers().ibist.read(SpIbistReg::BIST_FAIL)
    }

    pub fn set_check(&self) {
        registers().ibist.write(SpIbistReg::BIST_CHECK::SET)
    }

    pub fn set_go(&self) {
        registers().ibist.write(SpIbistReg::BIST_GO::SET)
    }

    pub fn set_clear(&self) {
        registers().ibist.write(SpIbistReg::BIST_CLEAR::SET)
    }
}

register_structs! {
    Registers {
        (0x0000 => pub pc: ReadWrite<u32, SpPcReg::Register>),
        (0x0004 => pub ibist: ReadWrite<u32, SpIbistReg::Register>),
        (0x0008 => @END),
    }
}

register_bitfields! {
    u32,

    SpPcReg [
        PROGRAM_COUNTER OFFSET(0)  NUMBITS(12) [],
    ],

    SpIbistReg [
        BIST_CHECK      OFFSET(0)  NUMBITS(1)  [],
        BIST_GO         OFFSET(1)  NUMBITS(1)  [],
        BIST_CLEAR      OFFSET(2)  NUMBITS(1)  [],
        BIST_DONE       OFFSET(2)  NUMBITS(1)  [],
        BIST_FAIL       OFFSET(3)  NUMBITS(4)  [],
    ]
}
