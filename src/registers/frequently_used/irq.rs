use register_rs::*;

/// `IRQ_MASK` register
#[derive(Register, ReadableRegister, WriteableRegister)]
#[register(address = 0x90, length = 4, endian = "big")]
pub struct IrqMask {
    // IRQ_MASK[3]
    /// INT_MASK[31:24]
    #[register(bits = "24..31", reset = 0)]
    int_mask_31_24: u8,

    // IRQ_MASK[2]
    /// INT_MASK[23:16]
    #[register(bits = "16..23", reset = 0)]
    int_mask_23_16: u8,

    // IRQ_MASK[1]
    /// INT_MASK[15:8]
    #[register(bits = "8..15", reset = 0)]
    int_mask_15_8: u8,

    // IRQ_MASK[0]
    /// INT_MASK[7:0]
    #[register(bits = "0..7", reset = 0)]
    int_mask_7_0: u8,
}

impl IrqMask {
    pub fn new(irq_mask: u32) -> Self {
        let word = irq_mask.to_be_bytes();
        Self {
            int_mask_31_24: word[0],
            int_mask_23_16: word[1],
            int_mask_15_8: word[2],
            int_mask_7_0: word[3],
        }
    }
}

/// `IRQ_STATUS` register. Read & Reset type register
#[derive(Register, ReadableRegister)]
#[register(address = 0xFA, length = 4, endian = "big")]
pub struct IrqStatus {
    // IRQ_STATUS[3]
    /// IRQ_STATUS[31:24]
    #[register(bits = "24..31", reset = 0)]
    int_status_31_24: u8,

    // IRQ_STATUS[2]
    /// IRQ_STATUS[23:16]
    #[register(bits = "16..23", reset = 0)]
    int_status_23_16: u8,

    // IRQ_STATUS[1]
    /// IRQ_STATUS[15:8]
    #[register(bits = "8..15", reset = 0)]
    int_status_15_8: u8,

    // IRQ_STATUS[0]
    /// IRQ_STATUS[7:0]
    #[register(bits = "0..7", reset = 0)]
    int_status_7_0: u8,
}

pub trait IrqType {
    fn as_u32(&self) -> u32;

    fn is_set(&self, event: InterruptEvent) -> bool {
        let word = self.as_u32();
        let pos: u32 = event.try_into().unwrap();

        word.bit(pos)
    }
}

impl IrqType for IrqMask {
    fn as_u32(&self) -> u32 {
        u32::from_be_bytes([
            self.int_mask_31_24,
            self.int_mask_23_16,
            self.int_mask_15_8,
            self.int_mask_7_0,
        ])
    }
}

impl IrqType for IrqStatus {
    fn as_u32(&self) -> u32 {
        u32::from_be_bytes([
            self.int_status_31_24,
            self.int_status_23_16,
            self.int_status_15_8,
            self.int_status_7_0,
        ])
    }
}

/// Interrupts
/// In order to notify the MCU of a certain number of events an interrupt signal is generated on 
/// a selectable GPIO.
/// 
/// All interrupts are reported on a set of interrupt status registers and are individually 
/// maskable. The interrupt status register must be cleared upon a read event from the MCU.
/// 
/// The status of all the interrupts is reported on the IRQ_STATUS[3:0] registers: bits are high 
/// for the events that have generated any interrupts. The interrupts are individually maskable 
/// using the IRQ_MASK[3:0] registers: if the mask bit related to a particular event is 
/// programmed at 0, that event does not generate any interrupt request.
#[derive(TryValued)]
#[valued(type = u32)]
pub enum InterruptEvent {
    #[valued(0)]
    RxDataReady,
    #[valued(1)]
    RxDataDiscarded,
    #[valued(2)]
    TxDataSent,
    #[valued(3)]
    MaxReTxReached,
    #[valued(4)]
    CrcError,
    #[valued(5)]
    TxFifoError,
    #[valued(6)]
    RxFifoError,
    #[valued(7)]
    TxFifoAlmostFull,
    #[valued(8)]
    TxFifoAlmostEmpty,
    #[valued(9)]
    RxFifoAlmostFull,
    #[valued(10)]
    RxFifoAlmostEmpty,
    #[valued(11)]
    MaxBackoffDuringCCA,
    #[valued(12)]
    ValidPreambleDetected,
    #[valued(13)]
    SyncWordDetected,
    #[valued(14)]
    RssiAboveThreshold,
    /// The interrupt flag n.15 is set (and consequently the interrupt request) only when the XO clock is 
    /// available for the state machine. This time may be delayed compared to the actual timer 
    /// expiration. However, the real time event can be sensed putting the end-of-counting signal on a 
    /// GPIO output.
    #[valued(15)]
    WakeUpTimeout,
    /// The interrupt flag n.16 is set each time the SPIRIT1 goes to READY state and the XO has 
    /// completed its setting transient (XO ready condition detected).
    #[valued(16)]
    Ready,
    #[valued(17)]
    StandbyStateSwitching,
    #[valued(18)]
    LowBatteryLevel,
    #[valued(19)]
    PowerOnReset,
    #[valued(20)]
    BrownoutEvent,
    #[valued(21)]
    Lock,
    #[valued(29)]
    TimerRxTimeout,
    #[valued(30)]
    OthersAesEndOfOperation,
}
