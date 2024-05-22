use am64x_pac::{Gpio0, Gpio1};

pub struct GPIO0<'p> {
    peripheral: &'p Gpio0,
}

pub struct GPIO1<'p> {
    peripheral: &'p Gpio1,
}
