use super::PicohaIo;

use core::panic::PanicInfo;
use numtoa::NumToA;

// GPIO traits
use embedded_hal::digital::v2::OutputPin;

///
impl PicohaIo
{

    // pub struct PanicInfo<'a> {
    //     payload: &'a (dyn Any + Send),
    //     message: Option<&'a fmt::Arguments<'a>>,
    //     location: &'a Location<'a>,
    // }
    // pub struct Location<'a> {
    //     file: &'a str,
    //     line: u32,
    //     col: u32,
    // }

    /// Panic handler implementation for the application
    pub fn panic_handler(&mut self, _info: &PanicInfo) -> ! {
        let mut tmp_buf = [0u8; 20];

        self.usb_serial.write(b"{\"log\":\"").ok();
        self.usb_serial.write(b"PANIC! => ").ok();
        self.usb_serial
            .write(_info.location().unwrap().file().as_bytes())
            .ok();
        self.usb_serial.write(b":").ok();
        self.usb_serial
            .write(_info.location().unwrap().line().numtoa(10, &mut tmp_buf))
            .ok();
        self.usb_serial.write(b"\"}\r\n").ok();
        loop {
            // self.led_pin.set_high().ok();
            // self.delay.delay_ms(100);
            // self.led_pin.set_low().ok();
            // self.delay.delay_ms(100);
        }
    }
}
