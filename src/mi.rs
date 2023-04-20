use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::HARDWARE;

/// The static address of the Nintendo 64's MIPS interface registers.
#[cfg(target_vendor = "nintendo64")]
const MI_REGS_BASE: usize = 0x0430_0000;

#[cfg(not(target_vendor = "nintendo64"))]
lazy_static::lazy_static! {
    /// A registry access analogue for development and testing.
    ///
    /// We have to modify the registry access mechanism when building for
    /// architectures other than the Nintendo 64 since the production registry
    /// access mechanism accesses a static memory location. This is disallowed
    /// on modern operating systems, so we instead dynamically allocate the
    /// memory so that testing and development can occur.
    static ref REGISTERS: MipsInterfaceRegisters = unsafe { std::mem::zeroed() };
}

#[non_exhaustive]
pub struct MipsInterface;

impl MipsInterface {
    /// Gets a reference to the MIPS interface registers.
    #[cfg(target_vendor = "nintendo64")]
    fn registers<'a>(&self) -> &'a MipsInterfaceRegisters {
        unsafe { &mut *(MI_REGS_BASE as *mut MipsInterfaceRegisters) }
    }

    /// Returns a reference to the MIPS interface registers.
    #[cfg(not(target_vendor = "nintendo64"))]
    fn registers<'a>(&self) -> &'a REGISTERS {
        &REGISTERS
    }

    /// Returns ownership of the MIPS interface registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.mips_interface.drop(self) }
    }
}

// This is a hack to allow code to run for development.
#[cfg(not(target_vendor = "nintendo64"))]
unsafe impl Sync for MipsInterfaceRegisters {}

register_structs! {
    MipsInterfaceRegisters {
        (0x0000 => pub init_mode: ReadWrite<u32, InitMode::Register>),
        (0x0004 => pub version: ReadOnly<u32, Version::Register>),
        (0x0008 => pub interrupts: ReadOnly<u32, Interrupts::Register>),
        (0x000C => pub interrupt_mask: ReadWrite<u32, InterruptMasks::Register>),
        (0x0010 => @END),
    }
}

register_bitfields! {
    u32,

    InitMode [
        INIT_LENGTH             OFFSET(0)  NUMBITS(7) [],
        INIT_MODE               OFFSET(7)  NUMBITS(1) [],
        EBUS_TEST_MODE          OFFSET(8)  NUMBITS(1) [],
        RDRAM_REG_MODE          OFFSET(9)  NUMBITS(1) [],

        CLEAR_INIT_MODE         OFFSET(7)  NUMBITS(1) [],
        SET_INIT_MODE           OFFSET(8)  NUMBITS(1) [],
        CLEAR_EBUS_TEST_MODE    OFFSET(9)  NUMBITS(1) [],
        SET_EBUS_TEST_MODE      OFFSET(10) NUMBITS(1) [],
        CLEAR_DP_INTERRUPT      OFFSET(11) NUMBITS(1) [],
        CLEAR_RDRAM_REG         OFFSET(12) NUMBITS(1) [],
        SET_RDRAM_REG           OFFSET(13) NUMBITS(1) [],
    ],

    Version [
        IO                      OFFSET(0)  NUMBITS(8) [],
        RAC                     OFFSET(8)  NUMBITS(8) [],
        RDP                     OFFSET(16) NUMBITS(8) [],
        RSP                     OFFSET(24) NUMBITS(8) [],
    ],

    Interrupts [
        SP_INTERRUPT            OFFSET(0)  NUMBITS(1) [],
        SI_INTERRUPT            OFFSET(1)  NUMBITS(1) [],
        AI_INTERRUPT            OFFSET(2)  NUMBITS(1) [],
        VI_INTERRUPT            OFFSET(3)  NUMBITS(1) [],
        PI_INTERRUPT            OFFSET(4)  NUMBITS(1) [],
        DP_INTERRUPT            OFFSET(5)  NUMBITS(1) [],
    ],

    InterruptMasks [
        SP_INTERRUPT_MASK       OFFSET(0)  NUMBITS(1) [],
        SI_INTERRUPT_MASK       OFFSET(1)  NUMBITS(1) [],
        AI_INTERRUPT_MASK       OFFSET(2)  NUMBITS(1) [],
        VI_INTERRUPT_MASK       OFFSET(3)  NUMBITS(1) [],
        PI_INTERRUPT_MASK       OFFSET(4)  NUMBITS(1) [],
        DP_INTERRUPT_MASK       OFFSET(5)  NUMBITS(1) [],

        CLEAR_SP_INTERRUPT_MASK OFFSET(0)  NUMBITS(1) [],
        SET_SP_INTERRUPT_MASK   OFFSET(1)  NUMBITS(1) [],
        CLEAR_SI_INTERRUPT_MASK OFFSET(2)  NUMBITS(1) [],
        SET_SI_INTERRUPT_MASK   OFFSET(3)  NUMBITS(1) [],
        CLEAR_AI_INTERRUPT_MASK OFFSET(4)  NUMBITS(1) [],
        SET_AI_INTERRUPT_MASK   OFFSET(5)  NUMBITS(1) [],
        CLEAR_VI_INTERRUPT_MASK OFFSET(6)  NUMBITS(1) [],
        SET_VI_INTERRUPT_MASK   OFFSET(7)  NUMBITS(1) [],
        CLEAR_PI_INTERRUPT_MASK OFFSET(8)  NUMBITS(1) [],
        SET_PI_INTERRUPT_MASK   OFFSET(9)  NUMBITS(1) [],
        CLEAR_DP_INTERRUPT_MASK OFFSET(10) NUMBITS(1) [],
        SET_DP_INTERRUPT_MASK   OFFSET(11) NUMBITS(1) [],
    ]
}
