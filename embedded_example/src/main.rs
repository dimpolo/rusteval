#![no_main]
#![no_std]

use core::fmt::Write;

use cortex_m_rt::entry;
use noline::complete::Completer;
use noline::history::StaticHistory;
use noline::line_buffer::StaticBuffer;
use noline::sync::Editor;
use stm32f4xx_hal::gpio::{Alternate, ErasedPin, Input, OpenDrain, Output, Pin};
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::pac::I2C1;
use stm32f4xx_hal::{pac, prelude::*};
use usb_device::prelude::*;

use rusteval::InteractiveRoot;

use crate::usb::UsbSerial;

mod usb;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // ---------- Clocks -----------------
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(12.MHz()).sysclk(168.MHz()).freeze();

    // ---------- GPIO -----------------
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    let pin_a1 = gpioa.pa1.into_floating_input().erase();
    let pin_a2 = gpioa.pa2.into_floating_input().erase();
    let pin_a3 = gpioa.pa3.into_push_pull_output().erase();
    let pin_a4 = gpioa.pa4.into_push_pull_output().erase();

    // ---------- I2C -----------------
    let sda = gpiob.pb7.into_alternate_open_drain();
    let scl = gpiob.pb6.into_alternate_open_drain();
    let i2c = I2c::new(dp.I2C1, (scl, sda), 100.kHz(), &clocks);

    // ---------- USB -----------------
    let usb = USB {
        usb_global: dp.OTG_FS_GLOBAL,
        usb_device: dp.OTG_FS_DEVICE,
        usb_pwrclk: dp.OTG_FS_PWRCLK,
        pin_dm: gpioa.pa11.into_alternate(),
        pin_dp: gpioa.pa12.into_alternate(),
        hclk: clocks.hclk(),
    };

    // SAFETY: EP_MEMORY is not accessed from anywhere else
    static mut EP_MEMORY: [u32; 1024] = [0; 1024];

    let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });

    let serial = usbd_serial::SerialPort::new(&usb_bus);

    let usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();

    let mut usb_serial = UsbSerial::new(usb_dev, serial);

    // ---------- REPL -----------------
    let terminal = loop {
        match Terminal::new(&mut usb_serial) {
            Ok(terminal) => break terminal,
            Err(e) => {
                writeln!(usb_serial, "Could not create Terminal").unwrap();
            }
        }
    };

    let mut repl = Repl {
        pin_a1,
        pin_a2,
        pin_a3,
        pin_a4,
        i2c,
    };

    repl.run(terminal, usb_serial);
}

type Terminal<IO> = Editor<StaticBuffer<256>, StaticHistory<128>, IO>;
type I2C = I2c<
    I2C1,
    (
        Pin<'B', 6, Alternate<4, OpenDrain>>,
        Pin<'B', 7, Alternate<4, OpenDrain>>,
    ),
>;

#[derive(InteractiveRoot)]
struct Repl {
    pin_a1: ErasedPin<Input>,
    pin_a2: ErasedPin<Input>,
    pin_a3: ErasedPin<Output>,
    pin_a4: ErasedPin<Output>,
    i2c: I2C,
}

impl Repl {
    pub fn run<'a>(
        &mut self,
        mut terminal: Terminal<UsbSerial<'a>>,
        mut serial: UsbSerial<'a>,
    ) -> ! {
        loop {
            match terminal.readline(">>> ", &mut serial, &*self) {
                Ok(s) => self.try_eval_mut(s, |r| match r {
                    Ok(r) => writeln!(serial, "{r:?}").unwrap(),
                    Err(r) => writeln!(serial, "{r:?}").unwrap(),
                }),
                Err(e) => writeln!(serial, "{e:?}").unwrap(),
            }
        }
    }
}

impl Completer for Repl {
    fn complete(&self, line: &str, idx: usize) -> Option<&str> {
        if let Ok((current_object, rest_line)) = self.get_queried_object(line) {
            let field_names = current_object.get_all_field_names();
            let method_names = current_object
                .try_as_methods()
                .map(|methods| methods.get_all_method_names())
                .unwrap_or(&[]);

            let mut candidates = field_names
                .iter()
                .chain(method_names)
                .filter(|candidate| candidate.starts_with(rest_line));

            candidates
                .nth(idx)
                .map(|candidate| &candidate[rest_line.len()..])
        } else {
            None
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
