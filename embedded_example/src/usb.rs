use core::fmt::Write;
use stm32f4xx_hal::otg_fs;
use usb_device::{device, UsbError};
use usbd_serial::SerialPort;

type Serial<'a> = SerialPort<'a, otg_fs::UsbBus<otg_fs::USB>>;
type UsbDevice<'a> = device::UsbDevice<'a, otg_fs::UsbBus<otg_fs::USB>>;

pub struct UsbSerial<'a> {
    usb_device: UsbDevice<'a>,
    serial: Serial<'a>,
}

impl<'a> UsbSerial<'a> {
    pub fn new(usb_device: UsbDevice<'a>, serial: Serial<'a>) -> Self {
        UsbSerial { usb_device, serial }
    }

    fn poll(&mut self) -> bool {
        self.usb_device.poll(&mut [&mut self.serial])
    }

    fn wait_ready(&mut self) {
        while !self.poll() {}
    }

    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), UsbError> {
        let count = buf.len();

        let mut write_offset = 0;
        while write_offset < count {
            match self.serial.write(&buf[write_offset..count]) {
                Ok(len) => {
                    write_offset += len;
                }
                Err(UsbError::WouldBlock) => {
                    self.wait_ready();
                }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

impl Write for UsbSerial<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| core::fmt::Error)
    }
}

impl noline::sync::Read for UsbSerial<'_> {
    type Error = UsbError;

    fn read(&mut self) -> Result<u8, Self::Error> {
        let mut buf = [0];
        loop {
            match self.serial.read(&mut buf) {
                Ok(1) => return Ok(buf[0]),
                Ok(_) | Err(UsbError::WouldBlock) => {
                    self.wait_ready();
                }
                Err(e) => return Err(e),
            }
        }
    }
}
impl noline::sync::Write for UsbSerial<'_> {
    type Error = UsbError;

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.write_all(buf)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        loop {
            let res = self.serial.flush();
            if matches!(res, Err(UsbError::WouldBlock)) {
                self.wait_ready();
            } else {
                return res;
            }
        }
    }
}
