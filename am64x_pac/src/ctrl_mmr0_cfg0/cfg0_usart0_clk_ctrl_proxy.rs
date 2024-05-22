#[doc = "Register `CFG0_USART0_CLK_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0Usart0ClkCtrlProxySpec>;
#[doc = "Register `CFG0_USART0_CLK_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0Usart0ClkCtrlProxySpec>;
#[doc = "Field `USART0_CLK_CTRL_CLK_DIV_PROXY` reader - 1:0\\]
Selects the clock divider value. Supports divide values of 1 to 4 Default is /4. To load the new divider value the clk_div_ld bit must be cleared and then set to 1."]
pub type Usart0ClkCtrlClkDivProxyR = crate::FieldReader;
#[doc = "Field `USART0_CLK_CTRL_CLK_DIV_PROXY` writer - 1:0\\]
Selects the clock divider value. Supports divide values of 1 to 4 Default is /4. To load the new divider value the clk_div_ld bit must be cleared and then set to 1."]
pub type Usart0ClkCtrlClkDivProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USART0_CLK_CTRL_CLK_DIV_LD_PROXY` reader - 16:16\\]
Load the output divider value"]
pub type Usart0ClkCtrlClkDivLdProxyR = crate::BitReader;
#[doc = "Field `USART0_CLK_CTRL_CLK_DIV_LD_PROXY` writer - 16:16\\]
Load the output divider value"]
pub type Usart0ClkCtrlClkDivLdProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the clock divider value. Supports divide values of 1 to 4 Default is /4. To load the new divider value the clk_div_ld bit must be cleared and then set to 1."]
    #[inline(always)]
    pub fn usart0_clk_ctrl_clk_div_proxy(&self) -> Usart0ClkCtrlClkDivProxyR {
        Usart0ClkCtrlClkDivProxyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Load the output divider value"]
    #[inline(always)]
    pub fn usart0_clk_ctrl_clk_div_ld_proxy(&self) -> Usart0ClkCtrlClkDivLdProxyR {
        Usart0ClkCtrlClkDivLdProxyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the clock divider value. Supports divide values of 1 to 4 Default is /4. To load the new divider value the clk_div_ld bit must be cleared and then set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn usart0_clk_ctrl_clk_div_proxy(
        &mut self,
    ) -> Usart0ClkCtrlClkDivProxyW<Cfg0Usart0ClkCtrlProxySpec> {
        Usart0ClkCtrlClkDivProxyW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Load the output divider value"]
    #[inline(always)]
    #[must_use]
    pub fn usart0_clk_ctrl_clk_div_ld_proxy(
        &mut self,
    ) -> Usart0ClkCtrlClkDivLdProxyW<Cfg0Usart0ClkCtrlProxySpec> {
        Usart0ClkCtrlClkDivLdProxyW::new(self, 16)
    }
}
#[doc = "CFG0_USART0_CLK_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart0_clk_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart0_clk_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Usart0ClkCtrlProxySpec;
impl crate::RegisterSpec for Cfg0Usart0ClkCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_usart0_clk_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Usart0ClkCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_usart0_clk_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Usart0ClkCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_USART0_CLK_CTRL_PROXY to value 0x03"]
impl crate::Resettable for Cfg0Usart0ClkCtrlProxySpec {
    const RESET_VALUE: u32 = 0x03;
}
