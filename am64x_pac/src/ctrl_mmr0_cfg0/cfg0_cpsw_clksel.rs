#[doc = "Register `CFG0_CPSW_CLKSEL` reader"]
pub type R = crate::R<Cfg0CpswClkselSpec>;
#[doc = "Register `CFG0_CPSW_CLKSEL` writer"]
pub type W = crate::W<Cfg0CpswClkselSpec>;
#[doc = "Field `CPSW_CLKSEL_CPTS_CLKSEL` reader - 3:0\\]
Selects the clock source for the CPSW Ethernet switch Common Platform Time Stamp module"]
pub type CpswClkselCptsClkselR = crate::FieldReader;
#[doc = "Field `CPSW_CLKSEL_CPTS_CLKSEL` writer - 3:0\\]
Selects the clock source for the CPSW Ethernet switch Common Platform Time Stamp module"]
pub type CpswClkselCptsClkselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Selects the clock source for the CPSW Ethernet switch Common Platform Time Stamp module"]
    #[inline(always)]
    pub fn cpsw_clksel_cpts_clksel(&self) -> CpswClkselCptsClkselR {
        CpswClkselCptsClkselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Selects the clock source for the CPSW Ethernet switch Common Platform Time Stamp module"]
    #[inline(always)]
    #[must_use]
    pub fn cpsw_clksel_cpts_clksel(&mut self) -> CpswClkselCptsClkselW<Cfg0CpswClkselSpec> {
        CpswClkselCptsClkselW::new(self, 0)
    }
}
#[doc = "CFG0_CPSW_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_cpsw_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_cpsw_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0CpswClkselSpec;
impl crate::RegisterSpec for Cfg0CpswClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_cpsw_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0CpswClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_cpsw_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0CpswClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CPSW_CLKSEL to value 0"]
impl crate::Resettable for Cfg0CpswClkselSpec {
    const RESET_VALUE: u32 = 0;
}
