#[doc = "Register `CFG0_CLKOUT_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0ClkoutCtrlProxySpec>;
#[doc = "Register `CFG0_CLKOUT_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0ClkoutCtrlProxySpec>;
#[doc = "Field `CLKOUT_CTRL_CLK_SEL_PROXY` reader - 0:0\\]
Selects CLKOUT clock source"]
pub type ClkoutCtrlClkSelProxyR = crate::BitReader;
#[doc = "Field `CLKOUT_CTRL_CLK_SEL_PROXY` writer - 0:0\\]
Selects CLKOUT clock source"]
pub type ClkoutCtrlClkSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUT_CTRL_CLK_EN_PROXY` reader - 4:4\\]
When set, activates CLKOUT output"]
pub type ClkoutCtrlClkEnProxyR = crate::BitReader;
#[doc = "Field `CLKOUT_CTRL_CLK_EN_PROXY` writer - 4:4\\]
When set, activates CLKOUT output"]
pub type ClkoutCtrlClkEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects CLKOUT clock source"]
    #[inline(always)]
    pub fn clkout_ctrl_clk_sel_proxy(&self) -> ClkoutCtrlClkSelProxyR {
        ClkoutCtrlClkSelProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
When set, activates CLKOUT output"]
    #[inline(always)]
    pub fn clkout_ctrl_clk_en_proxy(&self) -> ClkoutCtrlClkEnProxyR {
        ClkoutCtrlClkEnProxyR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects CLKOUT clock source"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_ctrl_clk_sel_proxy(&mut self) -> ClkoutCtrlClkSelProxyW<Cfg0ClkoutCtrlProxySpec> {
        ClkoutCtrlClkSelProxyW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
When set, activates CLKOUT output"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_ctrl_clk_en_proxy(&mut self) -> ClkoutCtrlClkEnProxyW<Cfg0ClkoutCtrlProxySpec> {
        ClkoutCtrlClkEnProxyW::new(self, 4)
    }
}
#[doc = "CFG0_CLKOUT_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_clkout_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_clkout_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0ClkoutCtrlProxySpec;
impl crate::RegisterSpec for Cfg0ClkoutCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_clkout_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0ClkoutCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_clkout_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0ClkoutCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CLKOUT_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0ClkoutCtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
