#[doc = "Register `CFG0_USART2_CLK_CTRL` reader"]
pub type R = crate::R<Cfg0Usart2ClkCtrlSpec>;
#[doc = "Register `CFG0_USART2_CLK_CTRL` writer"]
pub type W = crate::W<Cfg0Usart2ClkCtrlSpec>;
#[doc = "Field `USART2_CLK_CTRL_CLK_DIV` reader - 1:0\\]
Selects the clock divider value. Supports divide values of 1 to 4 Default is /4. To load the new divider value the clk_div_ld bit must be cleared and then set to 1."]
pub type Usart2ClkCtrlClkDivR = crate::FieldReader;
#[doc = "Field `USART2_CLK_CTRL_CLK_DIV` writer - 1:0\\]
Selects the clock divider value. Supports divide values of 1 to 4 Default is /4. To load the new divider value the clk_div_ld bit must be cleared and then set to 1."]
pub type Usart2ClkCtrlClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USART2_CLK_CTRL_CLK_DIV_LD` reader - 16:16\\]
Load the output divider value"]
pub type Usart2ClkCtrlClkDivLdR = crate::BitReader;
#[doc = "Field `USART2_CLK_CTRL_CLK_DIV_LD` writer - 16:16\\]
Load the output divider value"]
pub type Usart2ClkCtrlClkDivLdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the clock divider value. Supports divide values of 1 to 4 Default is /4. To load the new divider value the clk_div_ld bit must be cleared and then set to 1."]
    #[inline(always)]
    pub fn usart2_clk_ctrl_clk_div(&self) -> Usart2ClkCtrlClkDivR {
        Usart2ClkCtrlClkDivR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Load the output divider value"]
    #[inline(always)]
    pub fn usart2_clk_ctrl_clk_div_ld(&self) -> Usart2ClkCtrlClkDivLdR {
        Usart2ClkCtrlClkDivLdR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the clock divider value. Supports divide values of 1 to 4 Default is /4. To load the new divider value the clk_div_ld bit must be cleared and then set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn usart2_clk_ctrl_clk_div(&mut self) -> Usart2ClkCtrlClkDivW<Cfg0Usart2ClkCtrlSpec> {
        Usart2ClkCtrlClkDivW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Load the output divider value"]
    #[inline(always)]
    #[must_use]
    pub fn usart2_clk_ctrl_clk_div_ld(&mut self) -> Usart2ClkCtrlClkDivLdW<Cfg0Usart2ClkCtrlSpec> {
        Usart2ClkCtrlClkDivLdW::new(self, 16)
    }
}
#[doc = "CFG0_USART2_CLK_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usart2_clk_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usart2_clk_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Usart2ClkCtrlSpec;
impl crate::RegisterSpec for Cfg0Usart2ClkCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_usart2_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0Usart2ClkCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_usart2_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0Usart2ClkCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_USART2_CLK_CTRL to value 0x03"]
impl crate::Resettable for Cfg0Usart2ClkCtrlSpec {
    const RESET_VALUE: u32 = 0x03;
}
