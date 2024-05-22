#[doc = "Register `CFG0_ADC0_CLKSEL` reader"]
pub type R = crate::R<Cfg0Adc0ClkselSpec>;
#[doc = "Register `CFG0_ADC0_CLKSEL` writer"]
pub type W = crate::W<Cfg0Adc0ClkselSpec>;
#[doc = "Field `ADC0_CLKSEL_CLK_SEL` reader - 1:0\\]
Selects the sampling clock source for ADC0"]
pub type Adc0ClkselClkSelR = crate::FieldReader;
#[doc = "Field `ADC0_CLKSEL_CLK_SEL` writer - 1:0\\]
Selects the sampling clock source for ADC0"]
pub type Adc0ClkselClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the sampling clock source for ADC0"]
    #[inline(always)]
    pub fn adc0_clksel_clk_sel(&self) -> Adc0ClkselClkSelR {
        Adc0ClkselClkSelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the sampling clock source for ADC0"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_clksel_clk_sel(&mut self) -> Adc0ClkselClkSelW<Cfg0Adc0ClkselSpec> {
        Adc0ClkselClkSelW::new(self, 0)
    }
}
#[doc = "CFG0_ADC0_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_adc0_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_adc0_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Adc0ClkselSpec;
impl crate::RegisterSpec for Cfg0Adc0ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_adc0_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0Adc0ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_adc0_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0Adc0ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ADC0_CLKSEL to value 0"]
impl crate::Resettable for Cfg0Adc0ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
