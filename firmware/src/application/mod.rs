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

// ============================================================================

const VERSION: &str = "0.0.1";
const ID: &str      = "this_is_my_id";

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
            common::Request::IdGet   => common::Response::IdResp(&ID.as_bytes()),
        }
    }

    // -----------------------------------------------------------------------

    // Converts the mode argument to the hal mode constant
    //fn mode_arg_to_hal(mode: protocols::gpio::GpioDir) -> DynPinMode {
    //    match mode {
    //        protocols::gpio::GpioDir::PullUpInput    => DYN_PULL_UP_INPUT,
    //        protocols::gpio::GpioDir::PullDownInput  => DYN_PULL_DOWN_INPUT,
    //        protocols::gpio::GpioDir::ReadableOutput => DYN_READABLE_OUTPUT,
    //    }
    //}

    //fn cmd_pin_set_io(io: &mut DynPin, mode: protocols::gpio::GpioDir) {
    //}

    //fn cmd_pin_set_io(io: &mut DynPin, mode: u8) -> Result<(), CmdError> {
    //    match CmdPinDirValue::from_u8(mode) {
    //        Some(x) => match io.try_into_mode(Self::mode_arg_to_hal(x)) {
    //            Ok(_) => Ok(()),
    //            Err(err) => Err(CmdError::HalError(err)),
    //        },
    //        None => Err(CmdError::ArgError(mode)),
    //    }
    //}

    ///// To configure the  mode of the io
    /////
    //fn process_set_io_mode(&mut self, cmd: &Command) -> Answer {
    //    // Get pin value
    //    match self.gpio_ctrl.borrow(cmd.pin) {
    //        Some(io) => match Self::cmd_pin_set_io(io, cmd.arg) {
    //            Ok(_) => Answer::ok(
    //                cmd.pin,
    //                0,
    //                AnswerText::from_str("m").unwrap()
    //            ),

    //            Err(err) => match err {
    //                CmdError::HalError(_) => Answer::error(
    //                    0,
    //                    0,
    //                    AnswerText::from_str("Cannot set desired I/O mode").unwrap(),
    //                ),

    //                CmdError::ArgError(x) => {
    //                    let mut txt = AnswerText::new();
    //                    write!(txt, "Invalid arg: {}", x).unwrap();

    //                    Answer::error(
    //                        0,
    //                        0,
    //                        txt
    //                    )
    //                },
    //            }
    //        },

    //        None => Answer::error(cmd.pin, 0, AnswerText::from_str("Invalid pin").unwrap()),
    //    }

    //}

    //// ------------------------------------------------------------------------

    //fn cmd_pin_set_value(io: &mut DynPin, value: u8) -> Result<(), CmdError> {
    //    // Parse argument
    //    match CmdPinWriteValue::from_u8(value) {
    //        Some(x) => {
    //            match x {
    //                CmdPinWriteValue::High => match io.set_high() {
    //                    Ok(_) => Ok(()),
    //                    Err(e) => Err(CmdError::HalError(e)),
    //                },

    //                CmdPinWriteValue::Low => match io.set_low() {
    //                    Ok(_) => Ok(()),
    //                    Err(e) => Err(CmdError::HalError(e)),
    //                }
    //            }
    //        }

    //        None => Err(CmdError::ArgError(value)),
    //    }
    //}

    ///// To write a value on the io
    //fn process_write_io(&mut self, cmd: &Command) -> Answer {
    //    match self.gpio_ctrl.borrow(cmd.pin) {
    //        Some(io) => match Self::cmd_pin_set_value(io, cmd.arg) {
    //            Ok(())             => Answer::ok(cmd.pin, 0, AnswerText::from_str("m").unwrap()),
    //            Err(err) => match err {
    //                CmdError::HalError(_) => Answer::error(
    //                    cmd.pin,
    //                    0,
    //                    AnswerText::from_str("Cannot set desired pin value. Is direction correct?").unwrap(),
    //                ),

    //                CmdError::ArgError(x) => {
    //                    let mut txt = AnswerText::new();
    //                    write!(txt, "Invalid arg: {}", x).unwrap();

    //                    Answer::error(
    //                        cmd.pin,
    //                        0,
    //                        txt
    //                    )
    //                }
    //            }
    //        },

    //        None => Answer::error(cmd.pin, 0, AnswerText::from_str("Invalid pin").unwrap()),
    //    }
    //}

    //// ------------------------------------------------------------------------

    //fn cmd_pin_get_value(io: &mut DynPin) -> Result<bool, CmdError> {
    //    match io.is_high() {
    //        Err(e) => Err(CmdError::HalError(e)),
    //        Ok(v) => Ok(v)
    //    }
    //}

    ///// To read an io
    //fn process_read_io(&mut self, cmd: &Command) -> Answer {
    //    match self.gpio_ctrl.borrow(cmd.pin) {
    //        Some(io) => match Self::cmd_pin_get_value(io) {
    //            Ok(v) => Answer::ok(
    //                cmd.pin,
    //                match v { true => 1, false => 0},
    //                AnswerText::from_str("r").unwrap(),
    //            ),

    //            Err(_) => Answer::error(
    //                cmd.pin,
    //                0,
    //                AnswerText::from_str("Cannot read pin value. Is direction correct?").unwrap()
    //            )
    //        },

    //        None => Answer::error(cmd.pin, 0, AnswerText::from_str("Invalid pin").unwrap()),
    //    }
    //}

    // ------------------------------------------------------------------------

    ///// Process incoming commands
    /////
    //pub fn update_command_processing(&mut self) -> Option<Answer> {
    //}

    // ------------------------------------------------------------------------

    ///// Feed input buffer
    /////
    //pub fn feed_cmd_buffer(&mut self, buf: &[u8], count: usize) {
    //    self.usb_buffer.load(buf, count);
    //}
}
