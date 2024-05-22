#[doc = "Register `CFG0_MCU_TIMER1_CLKSEL` reader"]
pub type R = crate::R<Cfg0McuTimer1ClkselSpec>;
#[doc = "Register `CFG0_MCU_TIMER1_CLKSEL` writer"]
pub type W = crate::W<Cfg0McuTimer1ClkselSpec>;
#[doc = "Field `MCU_TIMER1_CLKSEL_CLK_SEL` reader - 2:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
pub type McuTimer1ClkselClkSelR = crate::FieldReader;
#[doc = "Field `MCU_TIMER1_CLKSEL_CLK_SEL` writer - 2:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
pub type McuTimer1ClkselClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
    #[inline(always)]
    pub fn mcu_timer1_clksel_clk_sel(&self) -> McuTimer1ClkselClkSelR {
        McuTimer1ClkselClkSelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Timer functional clock input select mux control (Reserved values default to HFOSC1_CLK)"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_timer1_clksel_clk_sel(&mut self) -> McuTimer1ClkselClkSelW<Cfg0McuTimer1ClkselSpec> {
        McuTimer1ClkselClkSelW::new(self, 0)
    }
}
#[doc = "CFG0_MCU_TIMER1_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_timer1_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_timer1_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuTimer1ClkselSpec;
impl crate::RegisterSpec for Cfg0McuTimer1ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_timer1_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0McuTimer1ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_timer1_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0McuTimer1ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_TIMER1_CLKSEL to value 0"]
impl crate::Resettable for Cfg0McuTimer1ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
