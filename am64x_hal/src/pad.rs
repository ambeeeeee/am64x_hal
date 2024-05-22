// use am64x_pac::;

use am64x_pac::padcfg_ctrl0_cfg0::*;

#[derive(PartialEq, Eq)]
pub enum MuxMode {
    GPIO,
    OSPI0,
}

impl TryFrom<MuxMode> for u8 {
    type Error = ();

    fn try_from(value: MuxMode) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub enum PullType {
    Pullup = 0,
    Pulldown = 1,
}

impl TryFrom<u8> for PullType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<PullType> for u8 {
    fn from(value: PullType) -> Self {
        todo!()
    }
}

pub trait PadCfg {
    /// Whether the pad's configuration is locked.
    fn is_locked(&self) -> bool;
    /// Locks the pad's configuration if `locked` is `true`.
    fn locked(&self, locked: bool);

    /// Whether input is enabled for this pad
    fn input_enabled(&self) -> bool;
    /// Enables or disables input for this pad.
    fn input_enable(&self, enabled: bool);

    /// Whether the internal pullup / pulldown resistor is enabled.
    fn pull_enabled(&self) -> bool;
    /// Enables or disables the internal pullup / pulldown resistor.
    fn pull_enable(&self, enabled: bool);

    /// The current pullup / pulldown state for this pad.
    fn get_pull_state(&self) -> PullType;
    /// Sets the current pullup / pulldown state for this pad.
    fn pull_state(&self, state: PullType);

    /// The current mux mode for the pad.
    fn get_mux_mode(&self) -> MuxMode;
    /// Sets the current mux mode for this pad.
    fn mux_mode(&self, mode: MuxMode);
}

// impl PadCfg for Cfg0Padconfig0 {
//     fn is_locked(&self) -> bool {
//         self.read().padconfig0_lock().bit()
//     }

//     fn locked(&self, locked: bool) {
//         if locked {
//             self.write(|w| w.padconfig0_lock().set_bit())
//         }
//     }

//     fn input_enabled(&self) -> bool {
//         self.read().padconfig0_rxactive().bit()
//     }

//     fn input_enable(&self, enabled: bool) {
//         if enabled {
//             self.write(|w| w.padconfig0_lock().set_bit())
//         } else {
//             self.write(|w| w.padconfig0_lock().clear_bit())
//         }
//     }

//     fn pull_enabled(&self) -> bool {
//         self.read().padconfig0_pulluden().bit()
//     }

//     fn pull_enable(&self, enabled: bool) {
//         if enabled {
//             self.write(|w| w.padconfig0_pulluden().set_bit())
//         } else {
//             self.write(|w| w.padconfig0_pulluden().clear_bit())
//         }
//     }

//     fn get_pull_state(&self) -> PullType {
//         (self.read().padconfig0_pulluden().bit() as u8)
//             .try_into()
//             .unwrap()
//     }

//     fn pull_state(&self, state: PullType) {
//         let state = u8::from(state);

//         match state {
//             0 => self.write(|w| w.padconfig0_pulltypesel().clear_bit()),
//             1 => self.write(|w| w.padconfig0_pulltypesel().clear_bit()),
//             _ => unreachable!(),
//         }
//     }

//     fn get_mux_mode(&self) -> u8 {
//         self.read().padconfig0_muxmode().bits()
//     }

//     fn mux_mode(&self, mode: u8) {
//         assert!(mode < 2);

//         self.write(|w| unsafe { w.padconfig0_muxmode().bits(mode) });
//     }

// }
use paste::paste;

macro_rules! pad_cfg {
    ($pad:ident,$number:expr,$mux_modes:expr) => {
        paste! {
            impl PadCfg for $pad {
                fn is_locked(&self) -> bool {
                    paste! {
                        self.read().[<padconfig $number _lock>]().bit()
                    }
                }

                fn locked(&self, locked: bool) {
                    paste! {
                        if locked {
                            self.write(|w| w.[<padconfig $number _lock>]().set_bit())
                        }
                    }
                }

                fn input_enabled(&self) -> bool {
                    self.read().[<padconfig $number _rxactive>]().bit()
                }

                fn input_enable(&self, enabled: bool) {
                    if enabled {
                        self.write(|w| w.[<padconfig $number _lock>]().set_bit())
                    } else {
                        self.write(|w| w.[<padconfig $number _lock>]().clear_bit())
                    }
                }

                fn pull_enabled(&self) -> bool {
                    self.read().[<padconfig $number _pulluden>]().bit()
                }

                fn pull_enable(&self, enabled: bool) {
                    if enabled {
                        self.write(|w| w.[<padconfig $number _pulluden>]().set_bit())
                    } else {
                        self.write(|w| w.[<padconfig $number _pulluden>]().clear_bit())
                    }
                }

                fn get_pull_state(&self) -> PullType {
                    (self.read().[<padconfig $number _pulluden>]().bit() as u8)
                        .try_into()
                        .unwrap()
                }

                fn pull_state(&self, state: PullType) {
                    let state = u8::from(state);

                    match state {
                        0 => self.write(|w| w.[<padconfig $number _pulltypesel>]().clear_bit()),
                        1 => self.write(|w| w.[<padconfig $number _pulltypesel>]().clear_bit()),
                        _ => unreachable!(),
                    }
                }

                fn get_mux_mode(&self) -> MuxMode {
                    // self.read().padconfig0_muxmode().bits().try_into().unwrap()
                    todo!()
                }

                fn mux_mode(&self, mode: MuxMode) {
                    for (supported_mode, value) in $mux_modes {
                        if mode == supported_mode {
                            self.write(|w| unsafe { w.[<padconfig $number _muxmode>]().bits(value) });

                            return;
                        }
                    }

                    panic!("invalid mux mode for pad");
                }
            }
        }
    };
}

pad_cfg!(Cfg0Padconfig0, 0, [(MuxMode::OSPI0, 0), (MuxMode::GPIO, 7)]);
pad_cfg!(Cfg0Padconfig1, 1, [(MuxMode::OSPI0, 0), (MuxMode::GPIO, 7)]);
pad_cfg!(Cfg0Padconfig2, 2, [(MuxMode::OSPI0, 0), (MuxMode::GPIO, 7)]);
pad_cfg!(Cfg0Padconfig3, 3, [(MuxMode::OSPI0, 0), (MuxMode::GPIO, 7)]);
pad_cfg!(Cfg0Padconfig4, 4, [(MuxMode::OSPI0, 0), (MuxMode::GPIO, 7)]);
pad_cfg!(Cfg0Padconfig5, 5, [(MuxMode::OSPI0, 0), (MuxMode::GPIO, 7)]);
pad_cfg!(Cfg0Padconfig6, 6, [(MuxMode::OSPI0, 0), (MuxMode::GPIO, 7)]);
pad_cfg!(Cfg0Padconfig7, 7, [(MuxMode::OSPI0, 0), (MuxMode::GPIO, 7)]);
pad_cfg!(Cfg0Padconfig8, 8, [(MuxMode::OSPI0, 0), (MuxMode::GPIO, 7)]);
pad_cfg!(Cfg0Padconfig9, 9, [(MuxMode::OSPI0, 0), (MuxMode::GPIO, 7)]);
pad_cfg!(
    Cfg0Padconfig10,
    10,
    [(MuxMode::OSPI0, 0), (MuxMode::GPIO, 7)]
);
