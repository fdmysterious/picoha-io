#![no_std]
#![no_main]

// The macro for our start-up function
use cortex_m_rt::entry;

// The macro for marking our interrupt functions
use rp_pico::hal::pac::interrupt;

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
use usb_device::{class_prelude::*, prelude::*};

// USB Communications Class Device support
use usbd_serial::SerialPort;

use hal::prelude::*;

// To use pin control stuff
use embedded_hal::digital::v2::OutputPin;

use heapless::String;
use ufmt;

// ============================================================================

//mod application;
mod platform;

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

    let mut delay      = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer()); // Append delay feature to the app

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.led.into_push_pull_output();

    /// Run the application
    //usb_serial.write(b"Hello world\r\n").unwrap();

    loop{
        if usb_device.poll(&mut [&mut usb_serial]) {
            let mut buf = [0u8; 64];

            match usb_serial.read(&mut buf) {
                Err(_) => {}

                Ok(0) => {
                    // Do nothing
                }

                Ok(count) => {
                    // Convert to upper case
                    buf.iter_mut().take(count).for_each(|b| {
                        b.make_ascii_uppercase();
                    });

                    // Send back to the host
                    let mut wr_ptr = &buf[..count];

                    while !wr_ptr.is_empty() {
                        match usb_serial.write(wr_ptr) {
                            Ok(len) => wr_ptr = &wr_ptr[len..],

                            // On error, just drop unwritten data.
                            // One possible error is Err(WouldBlock), meaning the USB
                            // write buffer is full.
                            Err(_)  => break,
                        }
                    }
                }
            }
        }
    }
}

// ============================================================================

/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
///
/// We do all our USB work under interrupt, so the main thread can continue on
/// knowing nothing about USB.
//#[allow(non_snake_case)]
//#[interrupt]
//unsafe fn USBCTRL_IRQ() {
//    //let app = APP_INSTANCE.as_mut().unwrap();
//    //app.usbctrl_irq();
//    USBIT_FLAG.store(true, Ordering::SeqCst);
//}

// ============================================================================

// PANIC MANAGEMENT
use core::panic::PanicInfo;
#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    //let app = APP_INSTANCE.as_mut().unwrap();
    //app.panic_handler(_info);
    loop{}
}

// ============================================================================

// End of file
