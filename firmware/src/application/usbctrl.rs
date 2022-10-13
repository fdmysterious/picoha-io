use super::PicohaIo;

// ============================================================================

///
impl PicohaIo
{
    pub fn usbctrl_irq(&mut self) {
        // Poll the USB driver with all of our supported USB Classes
        if self.usb_device.poll(&mut [self.usb_serial]) {
            // Buffer to read the serial port
            let mut serial_buffer = [0u8; 512];
            match self.usb_serial.read(&mut serial_buffer) {
                Err(_e) => {
                    // Do nothing
                }
                Ok(0) => {
                    // Do nothing
                }
                Ok(count) => {
                    self.usb_buffer.load(&serial_buffer, count);
                }
            }
        }
    }
}
