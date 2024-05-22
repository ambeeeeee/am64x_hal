// pub struct Gpio<'g, GPIO: Gpio, ENABLED, DIRECTION, MODE> {

pub mod bank;
mod config;
pub mod func;
pub mod pins;
pub use config::*;

pub use pins::ValidFunction;

macro_rules! gpio {
    ( $bank:ident:$prefix:ident, [ $(($id:expr, $pull_type:ident, $func:ident)),* ] ) => {
        paste::paste!{
            #[doc = "Pin bank " [<$bank>] ]
            pub mod [<$bank:snake>] {
                // use $crate::pac::{[<IO_ $bank:upper>],[<PADS_ $bank:upper>]};
                // use crate::sio::[<SioGpio $bank>];
                // use super::{Pin, pin, pull, func};
                // $(pub use super::pin::[<$bank:lower>]::[<$prefix $id>];)*

                // $(
                //     impl super::DefaultTypeState for [<$prefix $id>] {
                //         type Function = super::[<Function $func>];
                //         type PullType = super::[<Pull $pull_type>];
                //     }
                //  )*
                gpio!(struct: $bank $prefix $([<$prefix $id>], $id, $func, $pull_type),*);

                // impl Pins {
                //     /// Take ownership of the PAC peripherals and SIO slice and split it into discrete [`Pin`]s
                //     ///
                //     /// This clears the input-enable flag for all Bank0 pads.
                //     pub fn new(io : [<IO_ $bank:upper>], pads: [<PADS_ $bank:upper>], sio: [<SioGpio $bank>], reset : &mut $crate::pac::RESETS) -> Self {
                //         // use $crate::resets::SubsystemReset;
                //         // pads.reset_bring_down(reset);
                //         // io.reset_bring_down(reset);

                //         // {
                //         //     use $crate::gpio::pin::DynBankId;
                //         //     // SAFETY: this function owns the whole bank that will be affected.
                //         //     let sio = unsafe { &*$crate::pac::SIO::PTR };
                //         //     if DynBankId::$bank == DynBankId::Bank0 {
                //         //         sio.gpio_oe().reset();
                //         //         sio.gpio_out().reset();
                //         //     } else {
                //         //         sio.gpio_hi_oe().reset();
                //         //         sio.gpio_hi_out().reset();
                //         //     }
                //         // }

                //         // io.reset_bring_up(reset);
                //         // pads.reset_bring_up(reset);
                //         // reset_ie!($bank, pads);
                //         gpio!(members: io, pads, sio, $(([<$prefix $id>], $func, $pull_type)),+)
                //     }
                // }
            }
        }
    };
    (struct: $bank:ident $prefix:ident $($PXi:ident, $id:expr, $func:ident, $pull_type:ident),*) => {
        use crate::pins::Pin;
        paste::paste!{
                /// Collection of all the individual [`Pin`]s
                pub struct Pins {
                    // _io: [<IO_ $bank:upper>],
                    // _pads: [<PADS_ $bank:upper>],
                    // _sio: [<SioGpio $bank>],
                    $(
                        #[doc = "Pin " [<$PXi>] ]
                        pub [<$PXi:snake>]: Pin<crate::gpio::pins::[<$bank:lower>]::[<$prefix $id>] , crate::gpio::pins::[<Function $func>], crate::gpio::pins::[<Pull $pull_type>]>,
                     )*
                }
        }
    };
    (members: $io:ident, $pads:ident, $sio:ident, $(($PXi:ident, $func:ident, $pull_type:ident)),+) => {
        paste::paste!{
            Self {
                _io: $io,
                _pads: $pads,
                _sio: $sio,
                $(
                    [<$PXi:snake>]: Pin {
                        id: [<$PXi>] (()),
                        function: func::[<Function $func>] (()),
                        pull_type: pull::[<Pull $pull_type>] (())
                    },
                )+
            }
        }
    };
}

gpio!(
    Gpio0: Gpio,
    [
        (0, Down, Null),
        (1, Down, Null),
        (2, Down, Null),
        (3, Down, Null),
        (4, Down, Null),
        (5, Down, Null),
        (6, Down, Null),
        (7, Down, Null),
        (8, Down, Null),
        (9, Down, Null),
        (10, Down, Null),
        (11, Down, Null),
        (12, Down, Null),
        (13, Down, Null),
        (14, Down, Null),
        (15, Down, Null),
        (16, Down, Null),
        (17, Down, Null),
        (18, Down, Null),
        (19, Down, Null),
        (20, Down, Null),
        (21, Down, Null),
        (22, Down, Null),
        (23, Down, Null),
        (24, Down, Null),
        (25, Down, Null),
        (26, Down, Null),
        (27, Down, Null),
        (28, Down, Null),
        (29, Down, Null),
        (30, Down, Null),
        (31, Down, Null),
        (32, Down, Null),
        (33, Down, Null),
        (34, Down, Null),
        (35, Down, Null),
        (36, Down, Null),
        (37, Down, Null),
        (38, Down, Null),
        (39, Down, Null),
        (40, Down, Null),
        (41, Down, Null),
        (42, Down, Null),
        (43, Down, Null),
        (44, Down, Null),
        (45, Down, Null),
        (46, Down, Null),
        (47, Down, Null),
        (48, Down, Null),
        (49, Down, Null),
        (50, Down, Null),
        (51, Down, Null),
        (52, Down, Null),
        (53, Down, Null),
        (54, Down, Null),
        (55, Down, Null),
        (56, Down, Null),
        (57, Down, Null),
        (58, Down, Null),
        (59, Down, Null),
        (60, Down, Null),
        (61, Down, Null),
        (62, Down, Null),
        (63, Down, Null),
        (64, Down, Null),
        (65, Down, Null),
        (66, Down, Null),
        (67, Down, Null),
        (68, Down, Null),
        (69, Down, Null),
        (70, Down, Null),
        (71, Down, Null),
        (72, Down, Null),
        (73, Down, Null),
        (74, Down, Null),
        (75, Down, Null),
        (76, Down, Null),
        (77, Down, Null),
        (78, Down, Null),
        (79, Down, Null),
        (80, Down, Null),
        (81, Down, Null),
        (82, Down, Null),
        (83, Down, Null),
        (84, Down, Null),
        (85, Down, Null),
        (86, Down, Null),
        (87, Down, Null),
        (88, Down, Null),
        (89, Down, Null),
        (90, Down, Null),
        (91, Down, Null),
        (92, Down, Null),
        (93, Down, Null),
        (94, Down, Null),
        (95, Down, Null),
        (96, Down, Null),
        (97, Down, Null),
        (98, Down, Null),
        (99, Down, Null),
        (100, Down, Null)
    ]
);
