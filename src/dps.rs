use tock_registers::{register_bitfields, register_structs, registers::ReadWrite};

const DPS_REGS_BASE: usize = 0x0420_0000;

pub struct Dps;

impl Dps {
    pub fn registers<'a>(&self) -> &'a DpsRegisters {
        unsafe { &mut *(DPS_REGS_BASE as *mut DpsRegisters) }
    }
}

register_structs! {
    pub DpsRegisters {
        (0x0000 => pub texture_memory_bist: ReadWrite<u32, TmemBist::Register>),
        (0x0004 => pub buffer_test_mode: ReadWrite<u32, BufferTestMode::Register>),
        (0x0008 => pub buffer_test_address: ReadWrite<u32, BufferTestAddress::Register>),
        (0x000C => pub buffer_test_data: ReadWrite<u32, BufferTestData::Register>),
        (0x0010 => @END),
    }
}

register_bitfields! {
    u32,

    pub TmemBist [
        BIST_CHECK OFFSET(0) NUMBITS(1)  [],
        BIST_GO    OFFSET(1) NUMBITS(1)  [],
        BIST_CLEAR OFFSET(2) NUMBITS(1)  [],
        BIST_DONE  OFFSET(2) NUMBITS(1)  [],
        BIST_FAIL  OFFSET(3) NUMBITS(8)  [],
    ],

    pub BufferTestMode [
        TEST_MODE  OFFSET(0) NUMBITS(1)  [],
    ],

    pub BufferTestAddress [
        TEST_ADDR  OFFSET(0) NUMBITS(7)  [],
    ],

    pub BufferTestData [
        TEST_DATA  OFFSET(0) NUMBITS(32) [],
    ]
}
