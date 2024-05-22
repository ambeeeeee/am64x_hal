#[doc = "Register `CFG0_CPSW_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0CpswClkselProxySpec>;
#[doc = "Register `CFG0_CPSW_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0CpswClkselProxySpec>;
#[doc = "Field `CPSW_CLKSEL_CPTS_CLKSEL_PROXY` reader - 3:0\\]
Selects the clock source for the CPSW Ethernet switch Common Platform Time Stamp module"]
pub type CpswClkselCptsClkselProxyR = crate::FieldReader;
#[doc = "Field `CPSW_CLKSEL_CPTS_CLKSEL_PROXY` writer - 3:0\\]
Selects the clock source for the CPSW Ethernet switch Common Platform Time Stamp module"]
pub type CpswClkselCptsClkselProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Selects the clock source for the CPSW Ethernet switch Common Platform Time Stamp module"]
    #[inline(always)]
    pub fn cpsw_clksel_cpts_clksel_proxy(&self) -> CpswClkselCptsClkselProxyR {
        CpswClkselCptsClkselProxyR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Selects the clock source for the CPSW Ethernet switch Common Platform Time Stamp module"]
    #[inline(always)]
    #[must_use]
    pub fn cpsw_clksel_cpts_clksel_proxy(
        &mut self,
    ) -> CpswClkselCptsClkselProxyW<Cfg0CpswClkselProxySpec> {
        CpswClkselCptsClkselProxyW::new(self, 0)
    }
}
#[doc = "CFG0_CPSW_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_cpsw_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_cpsw_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0CpswClkselProxySpec;
impl crate::RegisterSpec for Cfg0CpswClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_cpsw_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0CpswClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_cpsw_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0CpswClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CPSW_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0CpswClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
