#[doc = "Register `CFG0_MCU_TIMER0_CLKSEL` reader"]
pub type R = crate::R<Cfg0McuTimer0ClkselSpec>;
#[doc = "Register `CFG0_MCU_TIMER0_CLKSEL` writer"]
pub type W = crate::W<Cfg0McuTimer0ClkselSpec>;
#[doc = "Field `MCU_TIMER0_CLKSEL_CLK_SEL` reader - 2:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
pub type McuTimer0ClkselClkSelR = crate::FieldReader;
#[doc = "Field `MCU_TIMER0_CLKSEL_CLK_SEL` writer - 2:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
pub type McuTimer0ClkselClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
    #[inline(always)]
    pub fn mcu_timer0_clksel_clk_sel(&self) -> McuTimer0ClkselClkSelR {
        McuTimer0ClkselClkSelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_timer0_clksel_clk_sel(&mut self) -> McuTimer0ClkselClkSelW<Cfg0McuTimer0ClkselSpec> {
        McuTimer0ClkselClkSelW::new(self, 0)
    }
}
#[doc = "CFG0_MCU_TIMER0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer0_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer0_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuTimer0ClkselSpec;
impl crate::RegisterSpec for Cfg0McuTimer0ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_timer0_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0McuTimer0ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_timer0_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0McuTimer0ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_TIMER0_CLKSEL to value 0"]
impl crate::Resettable for Cfg0McuTimer0ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
