use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

const _MI_REGS_BASE: usize = 0x0430_0000;

pub struct MipsInterface;

impl MipsInterface {
    fn _registers<'a>(&self) -> &'a MipsInterfaceRegisters {
        unsafe { &mut *(_MI_REGS_BASE as *mut MipsInterfaceRegisters) }
    }
}

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
