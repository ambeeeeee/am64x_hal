pub mod gpio;
pub mod pad;
pub mod pins;

pub mod pac {
    pub use am64x_pac::*;
}

fn main() {
    // let peripherals = am64x_pac::Peripherals::take().unwrap();
    // let gpio0 = gpio::get_gpio(peripherals.padcfg_ctrl0_cfg0.cfg0_padconfig0());
}
