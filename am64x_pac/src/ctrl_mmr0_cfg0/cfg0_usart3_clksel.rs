#[doc = "Register `CFG0_USART3_CLKSEL` reader"]
pub type R = crate::R<Cfg0Usart3ClkselSpec>;
#[doc = "Register `CFG0_USART3_CLKSEL` writer"]
pub type W = crate::W<Cfg0Usart3ClkselSpec>;
#[doc = "Field `USART3_CLKSEL_CLK_SEL` reader - 0:0\\]
Selects the clock source for UART3:"]
pub type Usart3ClkselClkSelR = crate::BitReader;
#[doc = "Field `USART3_CLKSEL_CLK_SEL` writer - 0:0\\]
Selects the clock source for UART3:"]
pub type Usart3ClkselClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for UART3:"]
    #[inline(always)]
    pub fn usart3_clksel_clk_sel(&self) -> Usart3ClkselClkSelR {
        Usart3ClkselClkSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for UART3:"]
    #[inline(always)]
    #[must_use]
    pub fn usart3_clksel_clk_sel(&mut self) -> Usart3ClkselClkSelW<Cfg0Usart3ClkselSpec> {
        Usart3ClkselClkSelW::new(self, 0)
    }
}
#[doc = "CFG0_USART3_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart3_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart3_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Usart3ClkselSpec;
impl crate::RegisterSpec for Cfg0Usart3ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_usart3_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0Usart3ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_usart3_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0Usart3ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_USART3_CLKSEL to value 0"]
impl crate::Resettable for Cfg0Usart3ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
