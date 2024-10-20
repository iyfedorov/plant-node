use anyhow::Error;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::Drawable;
use esp_idf_hal::i2c::I2cDriver;

use embedded_graphics::prelude::Point;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

pub trait Display<'a> {
    fn new(_i2c_driver: I2cDriver<'a>) -> Self;

    fn print_next(&mut self, text: &str) -> Result<(), Error>;
    fn print_at_position(&mut self, text: &str, point: Point) -> Result<(), Error>;
    fn clear(&mut self) -> Result<(), Error>;
}

pub struct DisplaySsd1306<'a> {
    dsp: Ssd1306<
        I2CInterface<I2cDriver<'a>>,
        DisplaySize128x64,
        ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
    >,
    style: MonoTextStyle<'a, BinaryColor>,
}

impl<'a> Display<'a> for DisplaySsd1306<'a> {
    fn new(_i2c_driver: I2cDriver<'a>) -> DisplaySsd1306<'a> {
        let interface = I2CDisplayInterface::new(_i2c_driver);

        let mut display: Ssd1306<
            I2CInterface<I2cDriver<'_>>,
            DisplaySize128x64,
            ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
        > = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();

        display.init().unwrap();

        let style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        Self {
            dsp: display,
            style,
        }
    }

    fn print_next(&mut self, text: &str) -> Result<(), Error> {
        Text::with_baseline(text, Point::zero(), self.style, Baseline::Top)
            .draw(&mut self.dsp)
            .and_then(|_| self.dsp.flush())
            .map_err(|e| anyhow::anyhow!("Txt error: {:?}", e))
    }

    fn print_at_position(&mut self, text: &str, point: Point) -> Result<(), Error> {
        Text::with_baseline(text, point, self.style, Baseline::Top)
            .draw(&mut self.dsp)
            .and_then(|_| self.dsp.flush())
            .map_err(|e| anyhow::anyhow!("Txt error: {:?}", e))
    }

    fn clear(&mut self) -> Result<(), Error> {
        self.dsp.clear_buffer();
        self.dsp
            .flush()
            .map_err(|e| anyhow::anyhow!("Txt error: {:?}", e))
    }
}

// pub fn display(i2c0: i2c::I2C0, scl: Gpio22, sda: Gpio21) {
//     let mut i2c = i2c::I2cDriver::new(
//         i2c0,
//         sda,
//         scl,
//         &Config {
//             baudrate: 50.Hz().into(),
//             ..Default::default()
//         },
//     )
//     .unwrap();

//     for device in 0..15 {
//         let to_send = [6u8 + 4 * device, 0, 0 >> 8, 255, 255];
//         i2c.write(0x40, &to_send, 300u32).ok();
//         println!("Sent: {:02x?}", to_send);
//     }

//     // let interface = I2CDisplayInterface::new(i2c);

//     // let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
//     //     .into_buffered_graphics_mode();

//     // display
//     //     .init()
//     //     .map_err(|_| EspError::from_infallible::<ESP_ERR_INVALID_STATE>())?;

//     // let text_style = MonoTextStyleBuilder::new()
//     //     .font(&FONT_6X10)
//     //     .text_color(BinaryColor::On)
//     //     .build();

//     // match Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
//     //     .draw(&mut display)
//     //     .map_err(|e| anyhow::anyhow!("Txt error: {:?}", e))?;

//     // Text::with_baseline(
//     //     &format!("Yeah: {}", 128_u8),
//     //     Point::new(0, 16),
//     //     text_style,
//     //     Baseline::Top,
//     // )
//     // .draw(&mut display)
//     // .map_err(|e| anyhow::anyhow!("Txt2 error: {:?}", e))?;

//     // display
//     //     .flush()
//     //     .map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;

//     // Ok(display)
// }
