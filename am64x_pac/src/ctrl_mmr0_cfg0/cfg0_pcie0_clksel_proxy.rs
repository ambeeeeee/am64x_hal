#[doc = "Register `CFG0_PCIE0_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Pcie0ClkselProxySpec>;
#[doc = "Register `CFG0_PCIE0_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Pcie0ClkselProxySpec>;
#[doc = "Field `PCIE0_CLKSEL_CPTS_CLKSEL_PROXY` reader - 2:0\\]
Selects the clock source for the PCIE0 Common Platform Time Stamp module"]
pub type Pcie0ClkselCptsClkselProxyR = crate::FieldReader;
#[doc = "Field `PCIE0_CLKSEL_CPTS_CLKSEL_PROXY` writer - 2:0\\]
Selects the clock source for the PCIE0 Common Platform Time Stamp module"]
pub type Pcie0ClkselCptsClkselProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the clock source for the PCIE0 Common Platform Time Stamp module"]
    #[inline(always)]
    pub fn pcie0_clksel_cpts_clksel_proxy(&self) -> Pcie0ClkselCptsClkselProxyR {
        Pcie0ClkselCptsClkselProxyR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the clock source for the PCIE0 Common Platform Time Stamp module"]
    #[inline(always)]
    #[must_use]
    pub fn pcie0_clksel_cpts_clksel_proxy(
        &mut self,
    ) -> Pcie0ClkselCptsClkselProxyW<Cfg0Pcie0ClkselProxySpec> {
        Pcie0ClkselCptsClkselProxyW::new(self, 0)
    }
}
#[doc = "CFG0_PCIE0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pcie0_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pcie0_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Pcie0ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Pcie0ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pcie0_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Pcie0ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pcie0_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Pcie0ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PCIE0_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Pcie0ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
