#[doc = "Register `CFG0_ADC0_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Adc0ClkselProxySpec>;
#[doc = "Register `CFG0_ADC0_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Adc0ClkselProxySpec>;
#[doc = "Field `ADC0_CLKSEL_CLK_SEL_PROXY` reader - 1:0\\]
Selects the sampling clock source for ADC0"]
pub type Adc0ClkselClkSelProxyR = crate::FieldReader;
#[doc = "Field `ADC0_CLKSEL_CLK_SEL_PROXY` writer - 1:0\\]
Selects the sampling clock source for ADC0"]
pub type Adc0ClkselClkSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the sampling clock source for ADC0"]
    #[inline(always)]
    pub fn adc0_clksel_clk_sel_proxy(&self) -> Adc0ClkselClkSelProxyR {
        Adc0ClkselClkSelProxyR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the sampling clock source for ADC0"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_clksel_clk_sel_proxy(&mut self) -> Adc0ClkselClkSelProxyW<Cfg0Adc0ClkselProxySpec> {
        Adc0ClkselClkSelProxyW::new(self, 0)
    }
}
#[doc = "CFG0_ADC0_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_adc0_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_adc0_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Adc0ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Adc0ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_adc0_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Adc0ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_adc0_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Adc0ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_ADC0_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Adc0ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
