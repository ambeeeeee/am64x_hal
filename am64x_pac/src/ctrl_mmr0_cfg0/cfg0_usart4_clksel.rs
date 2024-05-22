#[doc = "Register `CFG0_USART4_CLKSEL` reader"]
pub type R = crate::R<Cfg0Usart4ClkselSpec>;
#[doc = "Register `CFG0_USART4_CLKSEL` writer"]
pub type W = crate::W<Cfg0Usart4ClkselSpec>;
#[doc = "Field `USART4_CLKSEL_CLK_SEL` reader - 0:0\\]
Selects the clock source for UART4:"]
pub type Usart4ClkselClkSelR = crate::BitReader;
#[doc = "Field `USART4_CLKSEL_CLK_SEL` writer - 0:0\\]
Selects the clock source for UART4:"]
pub type Usart4ClkselClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for UART4:"]
    #[inline(always)]
    pub fn usart4_clksel_clk_sel(&self) -> Usart4ClkselClkSelR {
        Usart4ClkselClkSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for UART4:"]
    #[inline(always)]
    #[must_use]
    pub fn usart4_clksel_clk_sel(&mut self) -> Usart4ClkselClkSelW<Cfg0Usart4ClkselSpec> {
        Usart4ClkselClkSelW::new(self, 0)
    }
}
#[doc = "CFG0_USART4_CLKSEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart4_clksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart4_clksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Usart4ClkselSpec;
impl crate::RegisterSpec for Cfg0Usart4ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_usart4_clksel::R`](R) reader structure"]
impl crate::Readable for Cfg0Usart4ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_usart4_clksel::W`](W) writer structure"]
impl crate::Writable for Cfg0Usart4ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_USART4_CLKSEL to value 0"]
impl crate::Resettable for Cfg0Usart4ClkselSpec {
    const RESET_VALUE: u32 = 0;
}
