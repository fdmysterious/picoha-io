#![no_std]
#![no_main]

// The macro for our start-up function
use cortex_m_rt::entry;

// Time handling traits
use embedded_time::rate::*;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
// use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;
// use rp_pico::hal::gpio::dynpin::DynPin;

// USB Device support
use usb_device::class_prelude::*;

use embedded_hal::digital::v2::{
    OutputPin,
    ToggleableOutputPin,
    PinState,
};

// To use pin control stuff
//use embedded_hal::digital::v2::OutputPin;

// ============================================================================

mod application;
mod platform;

use application::buffer::UsbBufferIndex;
use protocols::{self, slip::Decoder};

// ============================================================================

/// Application object
//static mut APP_INSTANCE: Option<application::PicohaIo> = None;
//
///// USB bus allocator
//static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
///// USB device object
//static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
///// USB serial object
//static mut USB_SERIAL: Option<SerialPort<hal::usb::UsbBus>> = None;

// ============================================================================

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac  = pac::Peripherals::take().unwrap();
    let     core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut usb_serial = platform::init_usb_serial(&usb_bus);
    let mut usb_device = platform::init_usb_device(&usb_bus);
    let mut decoder    = Decoder::<16>::new();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led = pins.led.into_push_pull_output();

    // Init. the app
    //let mut app = application::PicohaIo::new(
    //    cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer()), // Append delay feature to the app
    //    pins,
    //);

    // Run the app
    loop {
        if usb_device.poll(&mut [&mut usb_serial]) {
            let mut buf = [0u8; 128];

            match usb_serial.read(&mut buf) {
                Err(_) => {}
                Ok(0)  => {}
                
                Ok(count) => {
                    let mut idx = 0;
                    while idx < count {
                        match decoder.feed(&buf[idx..(count)]) {
                            Err(e) => {
                                idx += e.pos;
                                decoder.reset(); // Reset decoder
                            },

                            Ok((nbytes, is_end)) => {
                                idx += nbytes;

                                if is_end {
                                    
                                    // Get slice
                                    let slice = decoder.slice();

                                    // Process incoming frame
                                    // TODO // Error managment
                                    let frame = match protocols::ha::MsgFrame::<16>::from_slice(slice) {
                                        Ok(x) => {
                                            let req = protocols::gpio::Request::consume_frame(x).unwrap();

                                            match req {
                                                protocols::gpio::Request::GpioWrite(idx, value) => {
                                                    match value {
                                                        protocols::gpio::GpioValue::Low  => led.set_low().unwrap(),
                                                        protocols::gpio::GpioValue::High => led.set_high().unwrap(),
                                                    }
                                                }

                                                _ => {
                                                    // TODO
                                                }
                                            }
                                        }

                                        Err(exc) => {
                                        }
                                    };

                                    decoder.reset();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ============================================================================

// PANIC MANAGEMENT
use core::panic::PanicInfo;
#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

// ============================================================================

// End of file
