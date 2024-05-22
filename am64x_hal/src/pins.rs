// macro_rules! pin {
//     ($gpio:ident,$number:expr,$erase_target:ident) => {
//         pub struct $gpio;

//         impl $gpio {
//             pub fn erase_pin(self) -> $erase_target {
//                 $erase_target { pin: $number }
//             }
//         }
//     }
// }

// pub struct PD {
//     pin: u8
// }

// pub struct PE {
//     pin: u8
// }

// pin!(PD10, 10, PD);
// pin!(PD18, 18, PD);

// pin!(PE10, )

pub trait PinId {
    #[inline]
    fn as_dyn(&self) -> DynPinId;
}

pub struct DynPinId {
    pub bank: DynBankId,
    pub num: usize,
}

pub enum DynBankId {
    Gpio0,
}
