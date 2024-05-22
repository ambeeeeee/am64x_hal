#[doc = "Register `CFG0_CPTS_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0CptsClkselProxySpec>;
#[doc = "Register `CFG0_CPTS_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0CptsClkselProxySpec>;
#[doc = "Field `CPTS_CLKSEL_CPTS_CLKSEL_PROXY` reader - 2:0\\]
Selects the clock source for the SoC Common Platform Time Stamp module"]
pub type CptsClkselCptsClkselProxyR = crate::FieldReader;
#[doc = "Field `CPTS_CLKSEL_CPTS_CLKSEL_PROXY` writer - 2:0\\]
Selects the clock source for the SoC Common Platform Time Stamp module"]
pub type CptsClkselCptsClkselProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the clock source for the SoC Common Platform Time Stamp module"]
    #[inline(always)]
    pub fn cpts_clksel_cpts_clksel_proxy(&self) -> CptsClkselCptsClkselProxyR {
        CptsClkselCptsClkselProxyR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the clock source for the SoC Common Platform Time Stamp module"]
    #[inline(always)]
    #[must_use]
    pub fn cpts_clksel_cpts_clksel_proxy(
        &mut self,
    ) -> CptsClkselCptsClkselProxyW<Cfg0CptsClkselProxySpec> {
        CptsClkselCptsClkselProxyW::new(self, 0)
    }
}
#[doc = "CFG0_CPTS_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_cpts_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_cpts_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0CptsClkselProxySpec;
impl crate::RegisterSpec for Cfg0CptsClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_cpts_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0CptsClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_cpts_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0CptsClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CPTS_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0CptsClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
