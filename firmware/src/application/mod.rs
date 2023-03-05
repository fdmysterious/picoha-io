// ============================================================================

// HAL
use embedded_hal::digital::v2::OutputPin;

use rp_pico::hal;
use rp_pico::hal::gpio::{DYN_PULL_DOWN_INPUT, DYN_PULL_UP_INPUT, DYN_READABLE_OUTPUT, DynPinMode};
use rp_pico::hal::gpio::dynpin::DynPin;

use embedded_hal::digital::v2::InputPin;

// GPIO Control
mod gpio_ctrl;
use gpio_ctrl::GpioController;

use protocols::{
    self,
    ha,

    common,
    gpio,
};

use const_random::const_random;

// ============================================================================

#[cfg(debug_assertions)]
const VERSION: &str = env!("GIT_HASH");

#[cfg(not(debug_assertions))]
const VERSION: &str = env!("CARGO_PKG_VERSION");

const ID: [u8;8]    = const_random!([u8;8]);

// ============================================================================

enum CmdError {
    /// Arg value is invalid
    ArgError(u8),

    /// A HAL error occured
    HalError(hal::gpio::Error),
}

// ============================================================================

/// Store all the usefull objects for the application
pub struct PicohaIo {
    /// To manage delay
    delay: cortex_m::delay::Delay,

    /// Controls gpios
    gpio_ctrl: GpioController,
}

// ============================================================================

/// Implementation of the App
impl PicohaIo {

    // ------------------------------------------------------------------------

    /// Application intialization
    pub fn new(
        delay: cortex_m::delay::Delay,
        pins: rp_pico::Pins,
    ) -> Self {
        Self {
            delay:      delay,
            gpio_ctrl: GpioController::new(pins),
        }
    }

    // -----------------------------------------------------------------------
    
    pub fn process_generic(&self, req: common::Request) -> common::Response {
        match req {
            common::Request::Ping    => common::Response::Good,
            common::Request::ItfType => common::Response::ItfTypeResp(ha::ItfType::Gpio),
            common::Request::Version => common::Response::VersionResp(&VERSION),
            common::Request::IdGet   => common::Response::IdResp(&ID),
        }
    }

    // -----------------------------------------------------------------------
 
    fn mode_arg_to_hal(mode: protocols::gpio::GpioDir) -> DynPinMode {
        match mode {
            protocols::gpio::GpioDir::PullUpInput    => DYN_PULL_UP_INPUT,
            protocols::gpio::GpioDir::PullDownInput  => DYN_PULL_DOWN_INPUT,
            protocols::gpio::GpioDir::Output         => DYN_READABLE_OUTPUT,
        }
    }

    fn hal_arg_to_mode(hal: DynPinMode) -> Option<protocols::gpio::GpioDir> {
        match hal {
            DYN_PULL_UP_INPUT   => Some(protocols::gpio::GpioDir::PullUpInput),
            DYN_PULL_DOWN_INPUT => Some(protocols::gpio::GpioDir::PullDownInput),
            DYN_READABLE_OUTPUT => Some(protocols::gpio::GpioDir::Output),
            _                   => None
        }
    }

    pub fn process_gpio(&mut self, req: gpio::Request) -> gpio::Response {
        match req {
            gpio::Request::GpioDirSet(idx, dir) => {
                let dir_value = Self::mode_arg_to_hal(dir);
                let pin = match self.gpio_ctrl.borrow(idx) {
                    Some(x) => x,
                    None    => {return gpio::Response::ErrInvalidArgs;}
                };
                
                match pin.try_into_mode(dir_value) {
                    Ok(_)  => gpio::Response::Good,
                    Err(x) => gpio::Response::ErrGeneric("Cannot set into desired mode"),
                }
            }

            gpio::Request::GpioDirGet(idx) => {
                let pin = match self.gpio_ctrl.borrow(idx) {
                    Some(x) => x,
                    None    => {return gpio::Response::ErrInvalidArgs;}
                };

                match Self::hal_arg_to_mode(pin.mode()) {
                    Some(x) => gpio::Response::GpioDir(idx, x),
                    None    => gpio::Response::ErrGeneric("Uknown pin direction")
                }
            }

            gpio::Request::GpioWrite(idx, value) => {
                let pin = match self.gpio_ctrl.borrow(idx) {
                    Some(x) => x,
                    None    => {return gpio::Response::ErrInvalidArgs;}
                };

                let ret = match value {
                    gpio::GpioValue::High => pin.set_high(),
                    gpio::GpioValue::Low  => pin.set_low()
                };

                match ret {
                    Ok(_)  => gpio::Response::Good,
                    Err(_) => gpio::Response::ErrGeneric("Cannot set desired value")
                }
            }

            gpio::Request::GpioRead(idx) => {
                let pin = match self.gpio_ctrl.borrow(idx) {
                    Some(x) => x,
                    None    => {return gpio::Response::ErrInvalidArgs;}
                };

                let value = match pin.is_high() {
                    Ok(x)  => if x {protocols::gpio::GpioValue::High} else {protocols::gpio::GpioValue::Low},
                    Err(_) => {return gpio::Response::ErrGeneric("Error reading pin");}
                };

                gpio::Response::GpioValue(idx, value)
            }
        }
    }
}
