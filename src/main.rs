// ✅ ❌ 🚧
// #![no_std]
#![feature(generic_const_exprs)]
fn main() {}

use spirit1_rs::prelude::*;

// struct DummySpiritHal();

// impl Spirit1HalBlocking for DummySpiritHal {
//     fn get_base_frequency(&self) -> u32 {
//         433_400_000
//     }

//     fn get_frequency_band(&self) -> BandSelect {
//         BandSelect::from_hz(self.get_base_frequency()).unwrap()
//     }

//     fn get_xtal_frequency(&self) -> u32 {
//         50_000_000
//     }

//     fn read_register<R>(&mut self) -> R where R: Register<u8> + ReadableRegister<u8> {
//         R::reset_value()
//     }

//     fn write_register<R>(&mut self, value: R) -> RadioResult<()> where R: WriteableRegister<u8>, [(); R::LENGTH]: Sized {
//         let bytes = value.into_bytes().unwrap();
//         // println!("write_register: [{:x?}] <- {:x?}", R::ADDRESS, bytes);
//         Ok(())
//     }

//     fn write_raw(&mut self, base: u8, value: &mut [u8]) -> RadioResult<()> {
//         // println!("write_raw: [{:x}] <- {:x?}", base, value);
//         Ok(())
//     }
// }

// fn main() {
//     let mut radio = DummySpiritHal();

//     _configure_radio(&mut radio).unwrap();
//     _simple_transmitter(&mut radio).unwrap();
// }

fn _simple_transmitter(radio: &mut impl Spirit1) -> RadioResult<()> {
    // let mut radio =
    // Spirit1::new(spi, irq_pin, 50_000_000, 433_400_000).expect("Invalid Radio Configuration");

    let payload = "Hello World".as_bytes();
    _configure_radio(radio)?;

    // Set basic destination address

    // loop {

    // }

    Ok(())
}

/// Copy of the implementation @ [https://forum.digikey.com/t/getting-started-with-the-spirit1-transceiver/15624]
fn _configure_radio(radio: &mut impl Spirit1) -> RadioResult<()> {
    // Restart the radio (can't do on this device)
    radio.management_wa_extra_current()?;
    radio.wait_for_ready()?;

    // Initialize the radio
    radio.init(RadioInitOpts {
        xtal_offset_ppm: 50,
        ..Default::default()
    })?;

    // Set the transmitter power level
    let pa_level_1 = PaPower1::from_dbm(0.0, radio.get_base_frequency()).expect("Invalid output power");
    let mut pa_config: PaPower = radio.read_register();
    pa_config.level_max_index = PaSlot::Slot1;
    radio.write_register(pa_level_1)?;
    radio.write_register(pa_config)?;

    // 🚧  Configure the packet format
    radio.configure_packet_protocol(PacketConfiguration::Basic(BasicProtocolOpts {
        preamble_length: PreambleLength::Bytes01,
        sync_length: PacketSyncLength::Bytes01,
        sync_words: (0, 0, 0, 0),
        fix_var_length: PacketLengthMode::Fixed,
        packet_length_width: 1,
        crc_mode: CrcMode::Crc0x07,
        control_length: PacketControlLength::Bytes01,
        address_field: true,
        fec: false,
        data_whitening: false,
    }))?;

    radio.configure_basic_filter(BasicAddressOpts {
        filter_on_my_address: false,
        my_address: 0,
        filter_on_multicast_address: false,
        multicast_address: 0,
        filter_on_broadcast_address: true,
        broadcast_address: 255,
    })?;

    // Configure interrupt pin
    // We have GPIO0 and GPIO1 wired up
    let gpio_0 = GpioConf::new(GpioMode::OutputLowPower(DigitalOutputMode::nIRQ));
    radio.write_register(Gpio0Conf(gpio_0))?;

    // Configure IRQ listener
    // TODO: Did I implement this?
    radio.irq_silence()?;
    let mut irq = IrqMaskBuilder::new();
    irq.set(InterruptEvent::TxDataSent)
        .set(InterruptEvent::RxDataReady)
        .set(InterruptEvent::RxDataDiscarded)
        .set(InterruptEvent::TimerRxTimeout);
    radio.write_register(IrqMask::from(irq))?;
    radio.irq_clear()?;

    // Receiver Quality Indicator Configuration
    // Enable the SQI threshold to 0 to require a perfect match between
    // the expected synchronization byte and the received synchronization byte
    let mut qi: QI = radio.read_register();
    qi.sqi_th = 0;
    qi.sqi_en = true;
    radio.write_register(qi)?;

    radio.set_rssi_threshold(-120)?;

    // Timer Configuration ❔
    radio.set_rx_timeout(2000.0)?;
    radio.set_rx_timeout_stop_condition(RxTimeoutStopCondition::SqiAboveThreshold)?;
    

    // Ok(radio)
    Ok(())
}