#[doc = "Register `CFG0_GTC_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0GtcClkselProxySpec>;
#[doc = "Register `CFG0_GTC_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0GtcClkselProxySpec>;
#[doc = "Field `GTC_CLKSEL_CLK_SEL_PROXY` reader - 2:0\\]
Selects the GTC timebase clock source"]
pub type GtcClkselClkSelProxyR = crate::FieldReader;
#[doc = "Field `GTC_CLKSEL_CLK_SEL_PROXY` writer - 2:0\\]
Selects the GTC timebase clock source"]
pub type GtcClkselClkSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the GTC timebase clock source"]
    #[inline(always)]
    pub fn gtc_clksel_clk_sel_proxy(&self) -> GtcClkselClkSelProxyR {
        GtcClkselClkSelProxyR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Selects the GTC timebase clock source"]
    #[inline(always)]
    #[must_use]
    pub fn gtc_clksel_clk_sel_proxy(&mut self) -> GtcClkselClkSelProxyW<Cfg0GtcClkselProxySpec> {
        GtcClkselClkSelProxyW::new(self, 0)
    }
}
#[doc = "CFG0_GTC_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_gtc_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_gtc_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0GtcClkselProxySpec;
impl crate::RegisterSpec for Cfg0GtcClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_gtc_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0GtcClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_gtc_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0GtcClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_GTC_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0GtcClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
