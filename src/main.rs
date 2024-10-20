use std::{thread::sleep, time::Duration};

use anyhow::Result;

use display::{Display, DisplaySsd1306};
use embedded_can::{ExtendedId, Frame, Id, StandardId};
use embedded_graphics::geometry::Point;
use esp_idf_hal::{
    delay::{Delay, FreeRtos},
    gpio::PinDriver,
    i2c::{self, I2cDriver},
    peripherals::Peripherals,
    spi::{config, SpiDeviceDriver, SpiDriver, SpiDriverConfig, SPI2},
};
use mcp2515::{frame::CanFrame, regs::OpMode, CanSpeed, McpSpeed, Settings, MCP2515};

mod display;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    // DISPLAY INIT
    let i2c_driver = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio21,
        peripherals.pins.gpio22,
        &i2c::config::Config::default(),
    )?;

    let mut display: DisplaySsd1306<'_> = Display::new(i2c_driver);
    // !DISPLAY INIT

    // CAN INIT
    let spi = peripherals.spi2;

    let mut delay = Delay::new(10_000_000);

    let int_pin = PinDriver::input(peripherals.pins.gpio4)?;

    let sclk = peripherals.pins.gpio15; // SCK
    let serial_in = peripherals.pins.gpio16; // SDI
    let serial_out = peripherals.pins.gpio17; // SDO
    let cs = peripherals.pins.gpio18; // CS

    let config = SpiDriverConfig::new();
    let driver: SpiDriver<'_> =
        SpiDriver::new::<SPI2>(spi, sclk, serial_out, Some(serial_in), &config)?;

    let device_config = config::Config::new();

    let device = SpiDeviceDriver::new(driver, Some(cs), &device_config)?;

    let mut mcp2515 = MCP2515::new(device);
    match mcp2515.init(
        &mut delay,
        Settings {
            mode: OpMode::Normal,
            can_speed: CanSpeed::Kbps100,
            mcp_speed: McpSpeed::MHz8,
            clkout_en: false,
        },
    ) {
        Ok(_) => display.print_next("Start bus")?,
        Err(e) => display.print_next(format!("Error bus {:?}", e).as_str())?,
    };
    // !CAN INIT

    let mut id_inc: u16 = 200u16;
    loop {
        if int_pin.is_low() {
            match mcp2515.read_message() {
                Ok(frame) => {
                    let _can_id = frame.id();
                    let _data = frame.data();

                    log::info!("Slave got {:?}, {:?}", _can_id, _data);
                    display.print_at_position(
                        format!("Got message id: {:?}, content: {:?}", _can_id, _data).as_str(),
                        Point::new(0, 16),
                    )?;

                    let new_data = [_data.first().map_or(1, |v| v + 1)];
                    let next_id = StandardId::new(id_inc).unwrap();
                    let new_frame = CanFrame::new(next_id, &new_data).unwrap();

                    mcp2515
                        .send_message(new_frame)
                        .map_err(|er| {
                            log::error!("{:?}", er);
                            display.print_at_position(
                                format!("Error bus send {:?}", er).as_str(),
                                Point::new(0, 16),
                            )
                        })
                        .unwrap();

                    id_inc += 1;
                }
                Err(err) => {
                    log::error!("{:?}", err);
                    display
                        .print_at_position(
                            format!("Error receive{:?}", err).as_str(),
                            Point::new(0, 16),
                        )
                        .unwrap();
                    sleep(Duration::from_millis(500));
                }
            }
        }
        FreeRtos::delay_ms(100);
    }
}
