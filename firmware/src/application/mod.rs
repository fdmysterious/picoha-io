// ============================================================================

/// Number of io on the rp2040
pub const NB_IO_RP2040: usize = 27;
pub const MAX_IO_INDEX_RP2040: usize = 28;

/// Max message string length in answer
pub const MAX_MSG_SIZE: usize = 128;

// HAL
use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal::{self, gpio::DYN_PULL_DOWN_INPUT};
use rp_pico::hal::gpio::{DYN_PULL_UP_INPUT, DYN_READABLE_OUTPUT};
use rp_pico::hal::gpio::dynpin::DynPin;

use embedded_hal::digital::v2::InputPin;

// Algos
use heapless::String;
use core::str::FromStr;
use core::write;
use core::fmt::Write;

// ============================================================================

use serde::{Deserialize, Serialize};
use serde_json_core;

#[derive(Deserialize, Debug)]
struct Command {
    /// 0 set mode / 1 write val / 2 read val / 10 test
    cod: u8,
    /// id of the pin (X => gpioX)
    pin: u8,
    /// if cmd = 0 mode  { 0 mode input_pullup, 1 mode input_pulldown, 2 mode output }
    /// if cmd = 1 write { the io value 0 or 1 }
    /// if cmd = 2 read  { none }
    arg: u8,
}

type AnswerText=String<MAX_MSG_SIZE>;

#[derive(Serialize, Debug)]
pub struct Answer {
    /// Status code
    sts: u8,
    /// id of the pin (X => gpioX)
    pin: u8,
    ///
    arg: u8,

    /// Text message
    msg: AnswerText,
}

// ============================================================================

pub enum AnsStatus {
    Ok = 0,
    Error = 1,
}

enum CmdError {
    ArgError(u8),
    HalError(hal::gpio::Error),
}

// ============================================================================

mod buffer;
use buffer::UsbBuffer;

// ============================================================================

/// Store all the usefull objects for the application
pub struct PicohaIo {
    /// To manage delay
    delay: cortex_m::delay::Delay,

    /// Objects to control io of the board
    dyn_ios: [DynPin; NB_IO_RP2040],
    /// Map to convert gpio index into *dyn_ios* index
    /// This is because some gpioX does not exist (or not driveable) and create hole in the array
    map_ios: [usize; MAX_IO_INDEX_RP2040],

    /// Buffer to hold incomnig data
    usb_buffer: UsbBuffer<512>,
}

// ============================================================================

/// Implementation of the App
impl PicohaIo {

    // ------------------------------------------------------------------------

    /// Application intialization
    pub fn new(
        delay: cortex_m::delay::Delay,
        dyn_ios: [DynPin; NB_IO_RP2040],
        map_ios: [usize; MAX_IO_INDEX_RP2040]
    ) -> Self {
        Self {
            delay:      delay,
            dyn_ios:    dyn_ios,
            map_ios:    map_ios,
            usb_buffer: UsbBuffer::new(),
        }
    }

    // -----------------------------------------------------------------------

    fn cmd_pin_set_io(io: &mut DynPin, mode: u8) -> Result<(), CmdError> {
        match mode {
            // Pull up input
            0 => match io.try_into_mode(DYN_PULL_UP_INPUT) {
                Err(e) => Err(CmdError::HalError(e)),
                Ok(_) => Ok(())
            },

            // Pull down input
            1 => match io.try_into_mode(DYN_PULL_DOWN_INPUT) {
                Err(e) => Err(CmdError::HalError(e)),
                Ok(_) => Ok(())
            },

            // Readable output
            2 => match io.try_into_mode(DYN_READABLE_OUTPUT) {
                Err(e) => Err(CmdError::HalError(e)),
                Ok(_) => Ok(())
            },

            invalid_arg => Err(CmdError::ArgError(invalid_arg))
        }
    }

    /// To configure the  mode of the io
    ///
    fn process_set_io_mode(&mut self, cmd: &Command) -> Answer {
        // Get io from cmd
        let idx = self.map_ios[cmd.pin as usize];
        let io = &mut self.dyn_ios[idx];

        // Process the argument
        match Self::cmd_pin_set_io(io, cmd.arg) {
            Ok(())             => Answer {sts: AnsStatus::Ok as u8, pin: cmd.pin, arg: 0, msg: AnswerText::from_str("m").unwrap()},
            Err(err) => match err {
                CmdError::HalError(_) => Answer{
                    sts: AnsStatus::Error as u8,
                    pin: 0,
                    arg: 0,
                    msg: AnswerText::from_str("Cannot set desired I/O mode").unwrap(),
                },

                CmdError::ArgError(x) => {
                    let mut txt = AnswerText::new();
                    write!(txt, "Invalid arg: {}", x).unwrap();

                    return Answer {
                        sts: AnsStatus::Error as u8,
                        pin: 0,
                        arg: 0,
                        msg: txt
                    };
                }
            }
        }
    }

    // ------------------------------------------------------------------------

    fn cmd_pin_set_value(io: &mut DynPin, value: u8) -> Result<(), CmdError> {
        match value {
            1 => match io.set_high() {
                Err(e) => Err(CmdError::HalError(e)),
                Ok(_)  => Ok(()),
            },

            0 => match io.set_low() {
                Err(e) => Err(CmdError::HalError(e)),
                Ok(_) => Ok(()),
            },

            invalid_arg => Err(CmdError::ArgError(invalid_arg)),
        }
    }

    /// To write a value on the io
    ///
    fn process_write_io(&mut self, cmd: &Command) -> Answer {
        // Get io from cmd
        let idx = self.map_ios[cmd.pin as usize];
        let io = &mut self.dyn_ios[idx];

        // Process the argument
        match Self::cmd_pin_set_value(io, cmd.arg) {
            Ok(())             => Answer {sts: AnsStatus::Ok as u8, pin: cmd.pin, arg: 0, msg: AnswerText::from_str("m").unwrap()},
            Err(err) => match err {
                CmdError::HalError(_) => Answer{
                    sts: AnsStatus::Error as u8,
                    pin: cmd.pin,
                    arg: 0,
                    msg: AnswerText::from_str("Cannot set desired pin value. Is direction correct?").unwrap(),
                },

                CmdError::ArgError(x) => {
                    let mut txt = AnswerText::new();
                    write!(txt, "Invalid arg: {}", x).unwrap();

                    return Answer {
                        sts: AnsStatus::Error as u8,
                        pin: cmd.pin,
                        arg: 0,
                        msg: txt
                    };
                }
            }
        }
    }

    // ------------------------------------------------------------------------

    fn cmd_pin_get_value(io: &mut DynPin) -> Result<bool, CmdError> {
        match io.is_high() {
            Err(e) => Err(CmdError::HalError(e)),
            Ok(v) => Ok(v)
        }
    }

    /// To read an io
    fn process_read_io(&mut self, cmd: &Command) -> Answer {
        // Get io from cmd
        let idx = self.map_ios[cmd.pin as usize];
        let io  = &mut self.dyn_ios[idx];

        match Self::cmd_pin_get_value(io) {
            Ok(v) => Answer {
                sts: AnsStatus::Ok as u8,
                pin: cmd.pin,
                arg: match v { true => 1, false => 0},
                msg: AnswerText::from_str("r").unwrap(),
            },

            Err(_) => Answer {
                sts: AnsStatus::Error as u8,
                pin: cmd.pin,
                arg: 0,
                msg: AnswerText::from_str("Cannot read pin value. Is direction correct?").unwrap()
            }
        }
    }

    // ------------------------------------------------------------------------

    /// Process incoming commands
    ///
    pub fn update_command_processing(&mut self) -> Option<Answer> {
        let mut cmd_buffer = [0u8; 512];

        match self.usb_buffer.get_command(&mut cmd_buffer) {
            None => None,
            Some(cmd_end_index) => {
                let cmd_slice_ref = &cmd_buffer[0..cmd_end_index];

                match serde_json_core::de::from_slice::<Command>(cmd_slice_ref) {

                    // Process parsing error
                    Err(_e) => Some(Answer {
                        sts: AnsStatus::Error as u8,
                        pin: 0,
                        arg: 0,
                        msg: AnswerText::from_str("error parsing command").unwrap(),
                    }),

                    Ok(cmd) => {
                        let data = &cmd.0;
                        match data.cod {
                            0  => Some(self.process_set_io_mode(data)),
                            1  => Some(self.process_write_io(data)),
                            2  => Some(self.process_read_io(data)),

                            10 => Some(Answer{
                                sts: AnsStatus::Ok as u8,
                                pin: 0,
                                arg: 1,
                                msg: AnswerText::from_str("").unwrap(),
                            }),

                            invalid_cmd => {
                                let mut txt = AnswerText::new();
                                write!(txt, "Unknown command: {}", invalid_cmd).unwrap();

                                return Some(Answer{
                                    sts: AnsStatus::Error as u8,
                                    pin: 0,
                                    arg: 0,
                                    msg: txt
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------------

    /// Feed input buffer
    ///
    pub fn feed_cmd_buffer(&mut self, buf: &[u8], count: usize) {
        self.usb_buffer.load(buf, count);
    }
}