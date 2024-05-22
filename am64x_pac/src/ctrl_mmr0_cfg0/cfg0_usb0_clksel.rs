#[doc = "Register `CFG0_USB0_CLKSEL` reader"]
pub type R = crate::R<Cfg0Usb0ClkselSpec>;
#[doc = "Register `CFG0_USB0_CLKSEL` writer"]
pub type W = crate::W<Cfg0Usb0ClkselSpec>;
#[doc = "Field `USB0_CLKSEL_REFCLK_SEL` reader - 0:0\\]
Selects the clock source for the USB0 ref_clk."]
pub type Usb0ClkselRefclkSelR = crate::BitReader;
#[doc = "Field `USB0_CLKSEL_REFCLK_SEL` writer - 0:0\\]
Selects the clock source for the USB0 ref_clk."]
pub type Usb0ClkselRefclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for the USB0 ref_clk."]
    #[inline(always)]
    pub fn usb0_clksel_refclk_sel(&self) -> Usb0ClkselRefclkSelR {
        Usb0ClkselRefclkSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for the USB0 ref_clk."]
    #[inline(always)]
    #[must_use]
    pub fn usb0_clksel_refclk_sel(&mut self) -> Usb0ClkselRefclkSelW<Cfg0Usb0ClkselSpec> {
        Usb0ClkselRefclkSelW::new(self, 0)
    }
}
#[doc = "CFG0_USB0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb0_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb0_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Usb0ClkselSpec;
impl crate::RegisterSpec for Cfg0Usb0ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_usb0_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0Usb0ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_usb0_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0Usb0ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_USB0_CLKSEL to value 0"]
impl crate::Resettable for Cfg0Usb0ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
