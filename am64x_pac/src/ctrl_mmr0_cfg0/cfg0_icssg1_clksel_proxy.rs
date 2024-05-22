#[doc = "Register `CFG0_ICSSG1_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Icssg1ClkselProxySpec>;
#[doc = "Register `CFG0_ICSSG1_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Icssg1ClkselProxySpec>;
#[doc = "Field `ICSSG1_CLKSEL_CORE_CLKSEL_PROXY` reader - 0:0\\]
Selects the ICSSG1 functional clock source"]
pub type Icssg1ClkselCoreClkselProxyR = crate::BitReader;
#[doc = "Field `ICSSG1_CLKSEL_CORE_CLKSEL_PROXY` writer - 0:0\\]
Selects the ICSSG1 functional clock source"]
pub type Icssg1ClkselCoreClkselProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICSSG1_CLKSEL_IEP_CLKSEL_PROXY` reader - 18:16\\]
Selects the ICSSG1 IEP clock source"]
pub type Icssg1ClkselIepClkselProxyR = crate::FieldReader;
#[doc = "Field `ICSSG1_CLKSEL_IEP_CLKSEL_PROXY` writer - 18:16\\]
Selects the ICSSG1 IEP clock source"]
pub type Icssg1ClkselIepClkselProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the ICSSG1 functional clock source"]
    #[inline(always)]
    pub fn icssg1_clksel_core_clksel_proxy(&self) -> Icssg1ClkselCoreClkselProxyR {
        Icssg1ClkselCoreClkselProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Selects the ICSSG1 IEP clock source"]
    #[inline(always)]
    pub fn icssg1_clksel_iep_clksel_proxy(&self) -> Icssg1ClkselIepClkselProxyR {
        Icssg1ClkselIepClkselProxyR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the ICSSG1 functional clock source"]
    #[inline(always)]
    #[must_use]
    pub fn icssg1_clksel_core_clksel_proxy(
        &mut self,
    ) -> Icssg1ClkselCoreClkselProxyW<Cfg0Icssg1ClkselProxySpec> {
        Icssg1ClkselCoreClkselProxyW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Selects the ICSSG1 IEP clock source"]
    #[inline(always)]
    #[must_use]
    pub fn icssg1_clksel_iep_clksel_proxy(
        &mut self,
    ) -> Icssg1ClkselIepClkselProxyW<Cfg0Icssg1ClkselProxySpec> {
        Icssg1ClkselIepClkselProxyW::new(self, 16)
    }
}
#[doc = "CFG0_ICSSG1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg1_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg1_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Icssg1ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Icssg1ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_icssg1_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Icssg1ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_icssg1_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Icssg1ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ICSSG1_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Icssg1ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
