#[doc = "Register `CFG0_ICSSG1_CLKSEL` reader"]
pub type R = crate::R<Cfg0Icssg1ClkselSpec>;
#[doc = "Register `CFG0_ICSSG1_CLKSEL` writer"]
pub type W = crate::W<Cfg0Icssg1ClkselSpec>;
#[doc = "Field `ICSSG1_CLKSEL_CORE_CLKSEL` reader - 0:0\\]
Selects the ICSSG1 functional clock source"]
pub type Icssg1ClkselCoreClkselR = crate::BitReader;
#[doc = "Field `ICSSG1_CLKSEL_CORE_CLKSEL` writer - 0:0\\]
Selects the ICSSG1 functional clock source"]
pub type Icssg1ClkselCoreClkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICSSG1_CLKSEL_IEP_CLKSEL` reader - 18:16\\]
Selects the ICSSG1 IEP clock source"]
pub type Icssg1ClkselIepClkselR = crate::FieldReader;
#[doc = "Field `ICSSG1_CLKSEL_IEP_CLKSEL` writer - 18:16\\]
Selects the ICSSG1 IEP clock source"]
pub type Icssg1ClkselIepClkselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the ICSSG1 functional clock source"]
    #[inline(always)]
    pub fn icssg1_clksel_core_clksel(&self) -> Icssg1ClkselCoreClkselR {
        Icssg1ClkselCoreClkselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Selects the ICSSG1 IEP clock source"]
    #[inline(always)]
    pub fn icssg1_clksel_iep_clksel(&self) -> Icssg1ClkselIepClkselR {
        Icssg1ClkselIepClkselR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the ICSSG1 functional clock source"]
    #[inline(always)]
    #[must_use]
    pub fn icssg1_clksel_core_clksel(&mut self) -> Icssg1ClkselCoreClkselW<Cfg0Icssg1ClkselSpec> {
        Icssg1ClkselCoreClkselW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Selects the ICSSG1 IEP clock source"]
    #[inline(always)]
    #[must_use]
    pub fn icssg1_clksel_iep_clksel(&mut self) -> Icssg1ClkselIepClkselW<Cfg0Icssg1ClkselSpec> {
        Icssg1ClkselIepClkselW::new(self, 16)
    }
}
#[doc = "CFG0_ICSSG1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_icssg1_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_icssg1_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Icssg1ClkselSpec;
impl crate::RegisterSpec for Cfg0Icssg1ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_icssg1_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0Icssg1ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_icssg1_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0Icssg1ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ICSSG1_CLKSEL to value 0"]
impl crate::Resettable for Cfg0Icssg1ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
