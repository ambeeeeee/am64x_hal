// pub struct Gpio<'g, GPIO: Gpio, ENABLED, DIRECTION, MODE> {

pub mod bank;
mod config;
pub use config::*;

pub struct Gpio<'g, GPIO: GpioCfg, ENABLED, DIRECTION, MODE> {
    periph: &'g GPIO,
    _enabled: ENABLED,
    _direction: DIRECTION,
    _mode: MODE,
}

pub struct Disabled;
pub struct Nop;

/// Creates a new [Gpio].
///
/// Example:
///
/// ```no_run
/// use jh71xx_hal::{gpio, pac};
///
/// let dp = pac::Peripherals::take().unwrap();
/// let gpio0 = gpio::get_gpio(dp.sys_pinctrl.padcfg().gpio0());
/// ```
pub fn get_gpio<GPIO: GpioCfg>(periph: &GPIO) -> Gpio<GPIO, Disabled, Nop, Nop> {
    Gpio {
        periph,
        _enabled: Disabled,
        _direction: Nop,
        _mode: Nop,
    }
}

use crate::pins::*;

macro_rules! pin_ids {
    ($bank:ident: $($id:expr;$name:ident),*) => {
        pin_ids!($bank as $bank: $($id;$name),*);
    };
    ($bank:ident as $prefix:ident: $($id:tt),*) => {
        pin_ids!($bank as $prefix: $($id;$id),*);
    };
    ($bank:ident as $prefix:ident: $($id:expr;$name:tt),*) => {
        paste::paste!{
            $(
                #[doc = "Type level variant for the pin `" $name "` in bank `" $prefix "`."]
                pub struct [<$prefix $name>] (pub(crate) ());
                // impl crate::typelevel::Sealed for [<$prefix $name>] {}
                impl PinId for [<$prefix $name>] {
                    #[inline]
                    fn as_dyn(&self) -> DynPinId {
                        DynPinId {
                            bank: DynBankId::$bank,
                            num: $id
                        }
                    }
                }
                // impl pin_sealed::TypeLevelPinId for [<$prefix $name>] {
                //     type Bank = [<Bank $bank>];

                //     const ID: DynPinId = DynPinId {
                //         bank: DynBankId::$bank,
                //         num: $id
                //     };

                //     fn new() -> Self {
                //         Self(())
                //     }
                // }
            )*
        }
    };
}

macro_rules! pin_valid_func {
    ($bank:ident as $prefix:ident, [$head:ident $(, $func:ident)*], [$($name:tt),+]) => {
        pin_valid_func!($bank as $prefix, [$($func),*], [$($name),+]);
        paste::paste!{$(
            impl ValidFunction<[<Function $head>]> for crate::gpio::[<$prefix $name>] {}
        )+}
    };
    ($bank:ident as $prefix:ident, [], [$($name:tt),+]) => {};
}

pin_ids!(Gpio0 as Gpio: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
pin_ids!(Gpio0 as Gpio: 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
pin_ids!(Gpio0 as Gpio: 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47);
pin_ids!(Gpio0 as Gpio: 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63);
pin_ids!(Gpio0 as Gpio: 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79);
pin_ids!(Gpio0 as Gpio: 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95);
pin_ids!(Gpio0 as Gpio: 96, 97, 98, 99, 100);
pin_valid_func!(
    Gpio0 as Gpio,
    [Ospi0, Null],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
);

pin_valid_func!(Gpio0 as Gpio, [Uart2, Null], [15, 16, 17, 23]);

pin_valid_func!(Gpio0 as Gpio, [Uart3, Null], [18, 19, 20, 24]);

pin_valid_func!(Gpio0 as Gpio, [Uart4, Null], [21, 22, 25]);

pin_valid_func!(Gpio0 as Gpio, [Uart5, Null], [26]);

pin_valid_func!(Gpio0 as Gpio, [Uart6, Null], [27, 29, 30, 36]);

pin_valid_func!(
    Gpio0 as Gpio,
    [Null],
    [
        37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
        60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82,
        83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100
    ]
);

pub trait ValidFunction<F: PinFunction>: PinId {}

pub trait PinFunction {}

pub struct FunctionOspi0;

impl PinFunction for FunctionOspi0 {}

pub struct FunctionNull;

impl PinFunction for FunctionNull {}

pub struct FunctionUart2;

impl PinFunction for FunctionUart2 {}

pub struct FunctionUart3;

impl PinFunction for FunctionUart3 {}

pub struct FunctionUart4;

impl PinFunction for FunctionUart4 {}

pub struct FunctionUart5;

impl PinFunction for FunctionUart5 {}

pub struct FunctionUart6;

impl PinFunction for FunctionUart6 {}
