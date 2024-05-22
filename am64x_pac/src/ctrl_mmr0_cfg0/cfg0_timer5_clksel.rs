#[doc = "Register `CFG0_TIMER5_CLKSEL` reader"]
pub type R = crate::R<Cfg0Timer5ClkselSpec>;
#[doc = "Register `CFG0_TIMER5_CLKSEL` writer"]
pub type W = crate::W<Cfg0Timer5ClkselSpec>;
#[doc = "Field `TIMER5_CLKSEL_CLK_SEL` reader - 3:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
pub type Timer5ClkselClkSelR = crate::FieldReader;
#[doc = "Field `TIMER5_CLKSEL_CLK_SEL` writer - 3:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
pub type Timer5ClkselClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
    #[inline(always)]
    pub fn timer5_clksel_clk_sel(&self) -> Timer5ClkselClkSelR {
        Timer5ClkselClkSelR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
    #[inline(always)]
    #[must_use]
    pub fn timer5_clksel_clk_sel(&mut self) -> Timer5ClkselClkSelW<Cfg0Timer5ClkselSpec> {
        Timer5ClkselClkSelW::new(self, 0)
    }
}
#[doc = "CFG0_TIMER5_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_timer5_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_timer5_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Timer5ClkselSpec;
impl crate::RegisterSpec for Cfg0Timer5ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_timer5_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0Timer5ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_timer5_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0Timer5ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_TIMER5_CLKSEL to value 0"]
impl crate::Resettable for Cfg0Timer5ClkselSpec {
    const RESET_VALUE: u32 = 0;
}