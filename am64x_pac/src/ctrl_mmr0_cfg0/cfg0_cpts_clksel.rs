#[doc = "Register `CFG0_CPTS_CLKSEL` reader"]
pub type R = crate::R<Cfg0CptsClkselSpec>;
#[doc = "Register `CFG0_CPTS_CLKSEL` writer"]
pub type W = crate::W<Cfg0CptsClkselSpec>;
#[doc = "Field `CPTS_CLKSEL_CPTS_CLKSEL` reader - 2:0\\]
Selects the clock source for the SoC Common Platform Time Stamp module"]
pub type CptsClkselCptsClkselR = crate::FieldReader;
#[doc = "Field `CPTS_CLKSEL_CPTS_CLKSEL` writer - 2:0\\]
Selects the clock source for the SoC Common Platform Time Stamp module"]
pub type CptsClkselCptsClkselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the clock source for the SoC Common Platform Time Stamp module"]
    #[inline(always)]
    pub fn cpts_clksel_cpts_clksel(&self) -> CptsClkselCptsClkselR {
        CptsClkselCptsClkselR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the clock source for the SoC Common Platform Time Stamp module"]
    #[inline(always)]
    #[must_use]
    pub fn cpts_clksel_cpts_clksel(&mut self) -> CptsClkselCptsClkselW<Cfg0CptsClkselSpec> {
        CptsClkselCptsClkselW::new(self, 0)
    }
}
#[doc = "CFG0_CPTS_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_cpts_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_cpts_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0CptsClkselSpec;
impl crate::RegisterSpec for Cfg0CptsClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_cpts_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0CptsClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_cpts_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0CptsClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CPTS_CLKSEL to value 0"]
impl crate::Resettable for Cfg0CptsClkselSpec {
    const RESET_VALUE: u32 = 0;
}
