#[doc = "Register `CFG0_GTC_CLKSEL` reader"]
pub type R = crate::R<Cfg0GtcClkselSpec>;
#[doc = "Register `CFG0_GTC_CLKSEL` writer"]
pub type W = crate::W<Cfg0GtcClkselSpec>;
#[doc = "Field `GTC_CLKSEL_CLK_SEL` reader - 2:0\\]
Selects the GTC timebase clock source"]
pub type GtcClkselClkSelR = crate::FieldReader;
#[doc = "Field `GTC_CLKSEL_CLK_SEL` writer - 2:0\\]
Selects the GTC timebase clock source"]
pub type GtcClkselClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the GTC timebase clock source"]
    #[inline(always)]
    pub fn gtc_clksel_clk_sel(&self) -> GtcClkselClkSelR {
        GtcClkselClkSelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the GTC timebase clock source"]
    #[inline(always)]
    #[must_use]
    pub fn gtc_clksel_clk_sel(&mut self) -> GtcClkselClkSelW<Cfg0GtcClkselSpec> {
        GtcClkselClkSelW::new(self, 0)
    }
}
#[doc = "CFG0_GTC_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gtc_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gtc_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0GtcClkselSpec;
impl crate::RegisterSpec for Cfg0GtcClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_gtc_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0GtcClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_gtc_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0GtcClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_GTC_CLKSEL to value 0"]
impl crate::Resettable for Cfg0GtcClkselSpec {
    const RESET_VALUE: u32 = 0;
}
