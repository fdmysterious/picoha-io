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

use heapless::Vec;

// To use pin control stuff
//use embedded_hal::digital::v2::OutputPin;

// ============================================================================

//mod application;
mod platform;
mod application;

use protocols::{self, slip::{Decoder, Encoder, SlipError}};

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

    let mut decoder    = Decoder::<64>::new();
    let mut encoder    = Encoder::<64>::new();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Init. the app
    let mut app = application::PicohaIo::new(
        cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer()), // Append delay feature to the app
        pins,
    );

    // Run the app
    loop {
        if usb_device.poll(&mut [&mut usb_serial]) {
            let mut buf = [0u8; 64];

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
                                    fn _process_slice(app: &application::PicohaIo, slice: &[u8]) -> Result<protocols::ha::MsgFrame, protocols::ha::MsgError> {
                                        let req_frame = protocols::ha::MsgFrame::from_slice(slice)?;
                                        
                                        match protocols::ha::CodeCategory::categorize(&req_frame.code) {
                                            protocols::ha::CodeCategory::ReqGeneric => {
                                                let req  = protocols::common::Request::consume_frame(req_frame)?;
                                                let resp = app.process_generic(req);

                                                Ok(resp.to_frame())
                                            }

                                            _ => Ok(protocols::common::Response::Good.to_frame())

                                        }
                                    }

                                    // Get and process incoming slice
                                    let slice          = decoder.slice();
                                    let response_frame = match _process_slice(&app, &slice){
                                        Ok(frame) => frame,
                                        Err(exc)  => {
                                            exc.to_frame()
                                        }
                                    };

                                    // Try encode frame
                                    fn _build_response<const BUFLEN: usize>(ff: &protocols::ha::MsgFrame, encoder: &mut Encoder<BUFLEN>) -> Result<(), SlipError> {
                                        encoder.feed(ff.code.to_u16().to_be_bytes().as_slice())?;
                                        encoder.feed(ff.data.as_slice())?;
                                        encoder.feed(ff.crc().to_be_bytes().as_slice())?;
                                        encoder.finish()?;

                                        Ok(())
                                    }

                                    match _build_response(&response_frame, &mut encoder) {
                                        Ok(_)  => {usb_serial.write(encoder.slice()).ok();}
                                        Err(_) => {}
                                    }

                                    decoder.reset();
                                    encoder.reset();
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
