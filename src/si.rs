//! # Serial Interface Wrapper
//!
//! This module wraps the Nintendo 64's Serial interface registers and provides
//! type- and memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadWrite, WriteOnly},
};

use crate::{register_access, HARDWARE};

register_access!(0x0460_0000, Registers);

/// A zero-size wrapper around the Nintendo 64's serial interface.
#[non_exhaustive]
pub struct SerialInterface {
    /// Contains getters and setters for `SI_PIF_ADDR_WR64B_REG`.
    pub pif_address_write_64_bits: PifAddressWrite64Bits,

    /// Contains getters and setters for `SI_PIF_ADDR_RD64B_REG`.
    pub pif_address_read_64_bits: PifAddressRead64Bits,

    /// Contains getters and setters for `SI_DRAM_ADDR_REG`.
    pub dram_address: DramAddress,

    /// Contains getters and setters for `SI_STATUS_REG`.
    pub status: Status,
}

impl SerialInterface {
    /// Returns ownership of the serial interface registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.serial_interface.drop(self) }
    }
}

/// A zero-size wrapper around `SI_DRAM_ADDR_REG`.
#[non_exhaustive]
pub struct DramAddress;

impl DramAddress {
    pub fn get(&self) -> u32 {
        registers().dram_addr.read(SiDramAddrReg::ADDRESS)
    }

    pub fn set(&self, dram_address: u32) {
        registers()
            .dram_addr
            .write(SiDramAddrReg::ADDRESS.val(dram_address))
    }
}

/// A zero-size wrapper around `SI_PIF_ADDR_RD64B_REG`.
#[non_exhaustive]
pub struct PifAddressRead64Bits;

impl PifAddressRead64Bits {
    pub fn start(&self) {
        registers()
            .pif_addr_rd_64b
            .write(SiPifAddrRd64BReg::START::SET)
    }
}

/// A zero-size wrapper around `SI_PIF_ADDR_WR64B_REG`.
#[non_exhaustive]
pub struct PifAddressWrite64Bits;

impl PifAddressWrite64Bits {
    pub fn start(&self) {
        registers()
            .pif_addr_wr_64b
            .write(SiPifAddrWr64BReg::START::SET)
    }
}

/// A zero-size wrapper around `SI_STATUS_REG`.
#[non_exhaustive]
pub struct Status;

impl Status {
    pub fn dma_busy(&self) -> bool {
        registers().status.is_set(SiStatusReg::DMA_BUSY)
    }

    pub fn io_busy(&self) -> bool {
        registers().status.is_set(SiStatusReg::IO_BUSY)
    }

    pub fn dma_error(&self) -> bool {
        registers().status.is_set(SiStatusReg::DMA_ERROR)
    }

    pub fn interrupt(&self) -> bool {
        registers().status.is_set(SiStatusReg::INTERRUPT)
    }

    pub fn clear_interrupt(&self) {
        registers().status.write(SiStatusReg::CLEAR_INTERRUPT::SET)
    }
}

register_structs! {
    Registers {
        (0x0000 => pub dram_addr: ReadWrite<u32, SiDramAddrReg::Register>),
        (0x0004 => pub pif_addr_rd_64b: WriteOnly<u32, SiPifAddrRd64BReg::Register>),
        (0x0008 => _reserved0),
        (0x0010 => pub pif_addr_wr_64b: WriteOnly<u32, SiPifAddrWr64BReg::Register>),
        (0x0014 => _reserved1),
        (0x0018 => pub status: ReadWrite<u32, SiStatusReg::Register>),
        (0x001C => @END),
    }
}

register_bitfields! {
    u32,

    SiDramAddrReg [
        ADDRESS         OFFSET(0)  NUMBITS(24) [],
    ],

    SiPifAddrRd64BReg [
        START           OFFSET(0)  NUMBITS(24) [],
    ],

    SiPifAddrWr64BReg [
        START           OFFSET(0)  NUMBITS(24) [],
    ],

    SiStatusReg [
        DMA_BUSY        OFFSET(0)  NUMBITS(1)  [],
        IO_BUSY         OFFSET(1)  NUMBITS(1)  [],
        DMA_ERROR       OFFSET(3)  NUMBITS(1)  [],
        INTERRUPT       OFFSET(12) NUMBITS(1)  [],

        CLEAR_INTERRUPT OFFSET(0)  NUMBITS(1) [],
    ],
}
