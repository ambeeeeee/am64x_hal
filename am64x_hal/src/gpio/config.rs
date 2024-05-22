// pub trait GpioCfg {
//     /// The pad number for the GPIO.
//     fn pad() -> u32;

//     /// Whether the GPIO is configured as an input.
//     fn is_input_enabled(&self) -> bool;
//     /// Enables the GPIO as an input.
//     fn input_enable(&self, enable: bool);
// }

// macro_rules! gpio_cfg {
//     ($gpio:ident,$pad:expr) => {
//         impl $crate::gpio::GpioCfg for $gpio {
//             fn pad() -> u32 {
//                 $pad
//             }

//             fn is_input_enabled(&self) -> bool {
//                 self.mem_binten
//             }
// //
//             fn input_enable(&self, enable: bool) {
//                 self.modify(|_, w| {
//                     if enable {
//                         w.ie().set_bit()
//                     } else {
//                         w.ie().clear_bit()
//                     }
//                 })
//             }
//         }
//     }
// }
// use am64x_pac::Gpio0;
// gpio_cfg!(Gpio0, 1);

use am64x_pac::{
    generic::{FieldReader, FieldWriter},
    padcfg_ctrl0_cfg0::Cfg0Padconfig0,
    Gpio0,
};
use bitvec::{bits, store::BitStore, view::BitViewSized};

use crate::pad::PadCfg;

pub struct GPIO0(am64x_pac::Gpio0);

impl GPIO0 {
    pub fn new(periph: Gpio0) -> Self {
        Self(periph)
    }

    pub fn free(self) -> Gpio0 {
        self.0
    }
}

/// A pin from the GPIO0 bank
pub struct GP0 {
    pin: u8,
}

macro_rules! gpio_pin {
    ($gpio:ident,$number:expr,$erase_target:ident) => {
        pub struct $gpio;

        impl $gpio {
            pub fn erase_pin(self) -> $erase_target {
                $erase_target { pin: $number }
            }
        }
    };
    ($gpio:ident, $bank:expr, $bank_offset:expr) => {
        // impl GpioCfg for $gpio {
        //     fn interrupt_rising_edge(&self) -> bool {
        //         self.
        //     }
        // }
    };
}

gpio_pin!(GP00, 0, GP0);
gpio_pin!(GP01, 1, GP0);
gpio_pin!(GP02, 2, GP0);
gpio_pin!(GP03, 3, GP0);
gpio_pin!(GP04, 4, GP0);
gpio_pin!(GP05, 5, GP0);
// TODO: More entries up to eighty freaking six
use bitvec::prelude::Lsb0;
// impl GpioCfg for Cfg0Padconfig0 {
//     fn interrupt_rising_edge(&self) -> bool {
//         let gpio0 = unsafe { Gpio0::steal() };

//         let bits = gpio0.mem_set_ris_trig01().read().bits().into_bitarray();

//         bits.get(0usize).as_deref().unwrap()
//     }

//     fn set_interrupt_rising_edge(&self, enable: bool) {
//         todo!()
//     }

//     fn falling_edge_interrupt(&self) -> bool {
//         todo!()
//     }

//     fn set_falling_edge_interrupt(&self, enable: bool) {
//         todo!()
//     }

//     fn interrupt_enabled(&self) -> bool {
//         todo!()
//     }

//     fn set_interrupt_enabled(&self, enable: bool) {
//         todo!()
//     }

//     fn is_input_enabled(&self) -> bool {
//         todo!()
//     }

//     fn input_enable(&self, enable: bool) {
//         todo!()
//     }
// }

pub trait GpioCfg: PadCfg {
    fn interrupt_rising_edge(&self) -> bool;
    fn set_interrupt_rising_edge(&self, enable: bool);

    fn falling_edge_interrupt(&self) -> bool;
    fn set_falling_edge_interrupt(&self, enable: bool);

    fn interrupt_enabled(&self) -> bool;
    fn set_interrupt_enabled(&self, enable: bool);

    fn is_input_enabled(&self) -> bool;
    fn input_enable(&self, enable: bool);
}

// gpio_pin(pin, bank, bank_offset)
gpio_pin!(Cfg0Padconfig0, 0, 0);
gpio_pin!(Cfg0Padconfig1, 0, 1);
gpio_pin!(Cfg0PadConfig2, 0, 2);
gpio_pin!(Cfg0PadConfig3, 0, 3);
gpio_pin!(Cfg0PadConfig4, 0, 4);

pub(super) trait GpioRegisters {}
