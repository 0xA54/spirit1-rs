// ✅ ❌ 🚧
#![no_std]

use spirit1_rs::prelude::*;
fn main() {
}

/// Copy of the implementation @ [https://forum.digikey.com/t/getting-started-with-the-spirit1-transceiver/15624]
fn _configure_radio(spi: impl SpiDevice) -> RadioResult<()> {
    let mut radio = Spirit1::new(
        spi, 
        50_000_000, 
        433_400_000
    ).expect("Invalid Radio Configuration");
    
    // Restart the radio (can't do on this device)
    radio.management_wa_extra_current()?;
    radio.wait_for_ready()?;

    // Initialize the radio
    radio.init(RadioInitOpts { 
        xtal_offset_ppm: 50,
        ..Default::default()
    })?;

    // Set the transmitter power level
    let pa_level_1 = PaPower1::from_dbm(0.0, radio.base_frequency).expect("Invalid output power");
    let mut pa_config: PaPower = radio.read_register();
    pa_config.level_max_index = PaSlot::Slot1;
    radio.write_register(pa_level_1)?;
    radio.write_register(pa_config)?;
    
    // Configure the packet format
    


    Ok(())
}
