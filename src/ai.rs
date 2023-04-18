use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadWrite, WriteOnly},
};

const AI_REGS_BASE: usize = 0x0450_0000;

#[non_exhaustive]
pub struct AudioInterface;

impl AudioInterface {
    fn registers<'a>(&self) -> &'a AudioInterfaceRegisters {
        unsafe { &mut *(AI_REGS_BASE as *mut AudioInterfaceRegisters) }
    }

    pub fn set_dram_address(&self, dram_address: u32) {
        self.registers()
            .dram_address
            .write(DmaAddress::ADDRESS.val(dram_address));
    }

    pub fn length_v1(&self) -> u32 {
        self.registers().length.read(Length::TRANSFER_LENGTH_V1)
    }

    pub fn set_length_v1(&self, length: u32) {
        self.registers()
            .length
            .write(Length::TRANSFER_LENGTH_V1.val(length));
    }

    pub fn length_v2(&self) -> u32 {
        self.registers().length.read(Length::TRANSFER_LENGTH_V2)
    }

    pub fn set_length_v2(&self, length: u32) {
        self.registers()
            .length
            .write(Length::TRANSFER_LENGTH_V2.val(length));
    }

    pub fn enable_dma(&self) {
        self.registers().control.write(Control::DMA_ENABLE::SET);
    }

    pub fn disable_dma(&self) {
        self.registers().control.write(Control::DMA_ENABLE::CLEAR);
    }

    pub fn status(&self) -> AudioInterfaceStatus {
        let registers = self.registers();
        AudioInterfaceStatus {
            is_full: registers.status.is_set(Status::FULL),
            dac_counter: registers.status.read(Status::DAC_COUNTER),
            bitclock_state: registers.status.is_set(Status::BITCLOCK_STATE),
            abus_word_2: registers.status.is_set(Status::ABUS_WORD_2),
            word_select: registers.status.is_set(Status::WORD_SELECT),
            is_data_available: registers.status.is_set(Status::DATA_AVAILABLE),
            is_dfifo2_loaded: registers.status.is_set(Status::DFIFO2_LOADED),
            is_dma_enabled: registers.status.is_set(Status::DMA_ENABLE),
            is_dma_requesting: registers.status.is_set(Status::DMA_REQUEST),
            is_dma_busy: registers.status.is_set(Status::DMA_BUSY),
            is_busy: registers.status.is_set(Status::BUSY),
        }
    }

    pub fn set_status(&self, status: AudioInterfaceStatus) {
        self.registers().status.write(
            Status::FULL.val(status.is_full.into())
                + Status::DAC_COUNTER.val(status.dac_counter)
                + Status::BITCLOCK_STATE.val(status.bitclock_state.into())
                + Status::ABUS_WORD_2.val(status.abus_word_2.into())
                + Status::WORD_SELECT.val(status.word_select.into())
                + Status::DATA_AVAILABLE.val(status.is_data_available.into())
                + Status::DFIFO2_LOADED.val(status.is_dfifo2_loaded.into())
                + Status::DMA_ENABLE.val(status.is_dma_enabled.into())
                + Status::DMA_REQUEST.val(status.is_dma_requesting.into())
                + Status::DMA_BUSY.val(status.is_dma_busy.into())
                + Status::BUSY.val(status.is_busy.into()),
        )
    }

    pub fn set_dac_rate(&self, dac_rate: AudioInterfaceDacRate) {
        self.registers()
            .dacrate
            .write(DacRate::DAC_RATE.val(match dac_rate {
                AudioInterfaceDacRate::Ntsc => DacRate::DAC_RATE::Ntsc.into(),
                AudioInterfaceDacRate::Mpal => DacRate::DAC_RATE::Mpal.into(),
                AudioInterfaceDacRate::Pal => DacRate::DAC_RATE::Pal.into(),
            }))
    }

    pub fn set_bit_rate(&self, bit_rate: u32) {
        self.registers()
            .bitrate
            .write(BitRate::BIT_RATE.val(bit_rate))
    }
}

pub struct AudioInterfaceStatus {
    pub is_full: bool,
    pub dac_counter: u32,
    pub bitclock_state: bool,
    pub abus_word_2: bool,
    pub word_select: bool,
    pub is_data_available: bool,
    pub is_dfifo2_loaded: bool,
    pub is_dma_enabled: bool,
    pub is_dma_requesting: bool,
    pub is_dma_busy: bool,
    pub is_busy: bool,
}

pub enum AudioInterfaceDacRate {
    Ntsc,
    Pal,
    Mpal,
}

register_structs! {
    AudioInterfaceRegisters {
        (0x0000 => pub dram_address: WriteOnly<u32, DmaAddress::Register>),
        (0x0004 => pub length: ReadWrite<u32, Length::Register>),
        (0x0008 => pub control: WriteOnly<u32, Control::Register>),
        (0x000C => pub status: ReadWrite<u32, Status::Register>),
        (0x0010 => pub dacrate: WriteOnly<u32, DacRate::Register>),
        (0x0014 => pub bitrate: WriteOnly<u32, BitRate::Register>),
        (0x0018 => @END),
    }
}

register_bitfields! {
    u32,

    DmaAddress [
        ADDRESS OFFSET(0) NUMBITS(24) [],
    ],

    Length [
        TRANSFER_LENGTH_V1 OFFSET(0)  NUMBITS(15) [],
        TRANSFER_LENGTH_V2 OFFSET(0)  NUMBITS(18) [],
    ],

    Control [
        DMA_ENABLE         OFFSET(0)  NUMBITS(1)  [],
    ],

    Status [
        CLEAR_INTERRUPT    OFFSET(0)  NUMBITS(32) [],
        FULL               OFFSET(0)  NUMBITS(1)  [],
        DAC_COUNTER        OFFSET(1)  NUMBITS(14) [],
        BITCLOCK_STATE     OFFSET(16) NUMBITS(1)  [],
        ABUS_WORD_2        OFFSET(19) NUMBITS(1)  [],
        WORD_SELECT        OFFSET(21) NUMBITS(1)  [],
        DATA_AVAILABLE     OFFSET(22) NUMBITS(1)  [],
        DFIFO2_LOADED      OFFSET(23) NUMBITS(1)  [],
        DMA_ENABLE         OFFSET(25) NUMBITS(1)  [],
        DMA_REQUEST        OFFSET(26) NUMBITS(1)  [],
        DMA_BUSY           OFFSET(27) NUMBITS(1)  [],
        BUSY               OFFSET(30) NUMBITS(1)  [],
    ],

    DacRate [
        DAC_RATE           OFFSET(0)  NUMBITS(14) [
            Ntsc = 0x2E6D354,
            Pal  = 0x2F5B2D2,
            Mpal = 0x2E6025C,
        ],
    ],

    BitRate [
        BIT_RATE           OFFSET(0)  NUMBITS(4)  [],
    ],
}
