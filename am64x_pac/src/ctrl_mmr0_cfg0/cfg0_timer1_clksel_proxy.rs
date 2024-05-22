#[doc = "Register `CFG0_TIMER1_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Timer1ClkselProxySpec>;
#[doc = "Register `CFG0_TIMER1_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Timer1ClkselProxySpec>;
#[doc = "Field `TIMER1_CLKSEL_CLK_SEL_PROXY` reader - 3:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
pub type Timer1ClkselClkSelProxyR = crate::FieldReader;
#[doc = "Field `TIMER1_CLKSEL_CLK_SEL_PROXY` writer - 3:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
pub type Timer1ClkselClkSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
    #[inline(always)]
    pub fn timer1_clksel_clk_sel_proxy(&self) -> Timer1ClkselClkSelProxyR {
        Timer1ClkselClkSelProxyR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
    #[inline(always)]
    #[must_use]
    pub fn timer1_clksel_clk_sel_proxy(
        &mut self,
    ) -> Timer1ClkselClkSelProxyW<Cfg0Timer1ClkselProxySpec> {
        Timer1ClkselClkSelProxyW::new(self, 0)
    }
}
#[doc = "CFG0_TIMER1_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer1_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer1_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Timer1ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Timer1ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_timer1_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Timer1ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_timer1_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Timer1ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_TIMER1_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Timer1ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
