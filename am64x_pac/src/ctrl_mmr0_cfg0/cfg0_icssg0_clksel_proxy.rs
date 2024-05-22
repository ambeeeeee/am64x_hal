#[doc = "Register `CFG0_ICSSG0_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Icssg0ClkselProxySpec>;
#[doc = "Register `CFG0_ICSSG0_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Icssg0ClkselProxySpec>;
#[doc = "Field `ICSSG0_CLKSEL_CORE_CLKSEL_PROXY` reader - 0:0\\]
Selects the ICSSG0 functional clock source"]
pub type Icssg0ClkselCoreClkselProxyR = crate::BitReader;
#[doc = "Field `ICSSG0_CLKSEL_CORE_CLKSEL_PROXY` writer - 0:0\\]
Selects the ICSSG0 functional clock source"]
pub type Icssg0ClkselCoreClkselProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICSSG0_CLKSEL_IEP_CLKSEL_PROXY` reader - 18:16\\]
Selects the ICSSG0 IEP clock source"]
pub type Icssg0ClkselIepClkselProxyR = crate::FieldReader;
#[doc = "Field `ICSSG0_CLKSEL_IEP_CLKSEL_PROXY` writer - 18:16\\]
Selects the ICSSG0 IEP clock source"]
pub type Icssg0ClkselIepClkselProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the ICSSG0 functional clock source"]
    #[inline(always)]
    pub fn icssg0_clksel_core_clksel_proxy(&self) -> Icssg0ClkselCoreClkselProxyR {
        Icssg0ClkselCoreClkselProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Selects the ICSSG0 IEP clock source"]
    #[inline(always)]
    pub fn icssg0_clksel_iep_clksel_proxy(&self) -> Icssg0ClkselIepClkselProxyR {
        Icssg0ClkselIepClkselProxyR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the ICSSG0 functional clock source"]
    #[inline(always)]
    #[must_use]
    pub fn icssg0_clksel_core_clksel_proxy(
        &mut self,
    ) -> Icssg0ClkselCoreClkselProxyW<Cfg0Icssg0ClkselProxySpec> {
        Icssg0ClkselCoreClkselProxyW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Selects the ICSSG0 IEP clock source"]
    #[inline(always)]
    #[must_use]
    pub fn icssg0_clksel_iep_clksel_proxy(
        &mut self,
    ) -> Icssg0ClkselIepClkselProxyW<Cfg0Icssg0ClkselProxySpec> {
        Icssg0ClkselIepClkselProxyW::new(self, 16)
    }
}
#[doc = "CFG0_ICSSG0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg0_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg0_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Icssg0ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Icssg0ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_icssg0_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Icssg0ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_icssg0_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Icssg0ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ICSSG0_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Icssg0ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
