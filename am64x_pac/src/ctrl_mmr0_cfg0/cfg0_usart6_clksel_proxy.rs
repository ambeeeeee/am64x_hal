#[doc = "Register `CFG0_USART6_CLKSEL_PROXY` reader"]
pub type R = crate::R<Cfg0Usart6ClkselProxySpec>;
#[doc = "Register `CFG0_USART6_CLKSEL_PROXY` writer"]
pub type W = crate::W<Cfg0Usart6ClkselProxySpec>;
#[doc = "Field `USART6_CLKSEL_CLK_SEL_PROXY` reader - 0:0\\]
Selects the clock source for UART6:"]
pub type Usart6ClkselClkSelProxyR = crate::BitReader;
#[doc = "Field `USART6_CLKSEL_CLK_SEL_PROXY` writer - 0:0\\]
Selects the clock source for UART6:"]
pub type Usart6ClkselClkSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for UART6:"]
    #[inline(always)]
    pub fn usart6_clksel_clk_sel_proxy(&self) -> Usart6ClkselClkSelProxyR {
        Usart6ClkselClkSelProxyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for UART6:"]
    #[inline(always)]
    #[must_use]
    pub fn usart6_clksel_clk_sel_proxy(
        &mut self,
    ) -> Usart6ClkselClkSelProxyW<Cfg0Usart6ClkselProxySpec> {
        Usart6ClkselClkSelProxyW::new(self, 0)
    }
}
#[doc = "CFG0_USART6_CLKSEL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart6_clksel_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart6_clksel_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Usart6ClkselProxySpec;
impl crate::RegisterSpec for Cfg0Usart6ClkselProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_usart6_clksel_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Usart6ClkselProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_usart6_clksel_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Usart6ClkselProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_USART6_CLKSEL_PROXY to value 0"]
impl crate::Resettable for Cfg0Usart6ClkselProxySpec {
    const RESET_VALUE: u32 = 0;
}
