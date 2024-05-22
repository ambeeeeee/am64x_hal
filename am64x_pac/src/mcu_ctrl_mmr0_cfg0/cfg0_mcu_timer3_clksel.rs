#[doc = "Register `CFG0_MCU_TIMER3_CLKSEL` reader"]
pub type R = crate::R<Cfg0McuTimer3ClkselSpec>;
#[doc = "Register `CFG0_MCU_TIMER3_CLKSEL` writer"]
pub type W = crate::W<Cfg0McuTimer3ClkselSpec>;
#[doc = "Field `MCU_TIMER3_CLKSEL_CLK_SEL` reader - 2:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
pub type McuTimer3ClkselClkSelR = crate::FieldReader;
#[doc = "Field `MCU_TIMER3_CLKSEL_CLK_SEL` writer - 2:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
pub type McuTimer3ClkselClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
    #[inline(always)]
    pub fn mcu_timer3_clksel_clk_sel(&self) -> McuTimer3ClkselClkSelR {
        McuTimer3ClkselClkSelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_timer3_clksel_clk_sel(&mut self) -> McuTimer3ClkselClkSelW<Cfg0McuTimer3ClkselSpec> {
        McuTimer3ClkselClkSelW::new(self, 0)
    }
}
#[doc = "CFG0_MCU_TIMER3_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer3_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer3_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuTimer3ClkselSpec;
impl crate::RegisterSpec for Cfg0McuTimer3ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_timer3_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0McuTimer3ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_timer3_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0McuTimer3ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_TIMER3_CLKSEL to value 0"]
impl crate::Resettable for Cfg0McuTimer3ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
