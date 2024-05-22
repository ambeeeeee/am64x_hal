#[doc = "Register `CFG0_USB0_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Usb0ClkselProxySpec>;
#[doc = "Register `CFG0_USB0_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Usb0ClkselProxySpec>;
#[doc = "Field `USB0_CLKSEL_REFCLK_SEL_PROXY` reader - 0:0\\]
Selects the clock source for the USB0 ref_clk."]
pub type Usb0ClkselRefclkSelProxyR = crate::BitReader;
#[doc = "Field `USB0_CLKSEL_REFCLK_SEL_PROXY` writer - 0:0\\]
Selects the clock source for the USB0 ref_clk."]
pub type Usb0ClkselRefclkSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for the USB0 ref_clk."]
    #[inline(always)]
    pub fn usb0_clksel_refclk_sel_proxy(&self) -> Usb0ClkselRefclkSelProxyR {
        Usb0ClkselRefclkSelProxyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for the USB0 ref_clk."]
    #[inline(always)]
    #[must_use]
    pub fn usb0_clksel_refclk_sel_proxy(
        &mut self,
    ) -> Usb0ClkselRefclkSelProxyW<Cfg0Usb0ClkselProxySpec> {
        Usb0ClkselRefclkSelProxyW::new(self, 0)
    }
}
#[doc = "CFG0_USB0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb0_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb0_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Usb0ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Usb0ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_usb0_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Usb0ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_usb0_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Usb0ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_USB0_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Usb0ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
