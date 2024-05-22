#[doc = "Register `CFG0_MCU_OBSCLK_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0McuObsclkCtrlProxySpec>;
#[doc = "Register `CFG0_MCU_OBSCLK_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0McuObsclkCtrlProxySpec>;
#[doc = "Field `MCU_OBSCLK_CTRL_CLK_SEL_PROXY` reader - 2:0\\]
MCU_OBSCLK pin clock selection"]
pub type McuObsclkCtrlClkSelProxyR = crate::FieldReader;
#[doc = "Field `MCU_OBSCLK_CTRL_CLK_SEL_PROXY` writer - 2:0\\]
MCU_OBSCLK pin clock selection"]
pub type McuObsclkCtrlClkSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCU_OBSCLK_CTRL_CLK_DIV_PROXY` reader - 11:8\\]
MCU_OBSCLK pin clock selection output divider"]
pub type McuObsclkCtrlClkDivProxyR = crate::FieldReader;
#[doc = "Field `MCU_OBSCLK_CTRL_CLK_DIV_PROXY` writer - 11:8\\]
MCU_OBSCLK pin clock selection output divider"]
pub type McuObsclkCtrlClkDivProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCU_OBSCLK_CTRL_CLK_DIV_LD_PROXY` reader - 16:16\\]
Load the output divider value"]
pub type McuObsclkCtrlClkDivLdProxyR = crate::BitReader;
#[doc = "Field `MCU_OBSCLK_CTRL_CLK_DIV_LD_PROXY` writer - 16:16\\]
Load the output divider value"]
pub type McuObsclkCtrlClkDivLdProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_OBSCLK_CTRL_OUT_MUX_SEL_PROXY` reader - 24:24\\]
MCU_OBSCLK pin output mux selection."]
pub type McuObsclkCtrlOutMuxSelProxyR = crate::BitReader;
#[doc = "Field `MCU_OBSCLK_CTRL_OUT_MUX_SEL_PROXY` writer - 24:24\\]
MCU_OBSCLK pin output mux selection."]
pub type McuObsclkCtrlOutMuxSelProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
MCU_OBSCLK pin clock selection"]
    #[inline(always)]
    pub fn mcu_obsclk_ctrl_clk_sel_proxy(&self) -> McuObsclkCtrlClkSelProxyR {
        McuObsclkCtrlClkSelProxyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
MCU_OBSCLK pin clock selection output divider"]
    #[inline(always)]
    pub fn mcu_obsclk_ctrl_clk_div_proxy(&self) -> McuObsclkCtrlClkDivProxyR {
        McuObsclkCtrlClkDivProxyR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Load the output divider value"]
    #[inline(always)]
    pub fn mcu_obsclk_ctrl_clk_div_ld_proxy(&self) -> McuObsclkCtrlClkDivLdProxyR {
        McuObsclkCtrlClkDivLdProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
MCU_OBSCLK pin output mux selection."]
    #[inline(always)]
    pub fn mcu_obsclk_ctrl_out_mux_sel_proxy(&self) -> McuObsclkCtrlOutMuxSelProxyR {
        McuObsclkCtrlOutMuxSelProxyR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
MCU_OBSCLK pin clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsclk_ctrl_clk_sel_proxy(
        &mut self,
    ) -> McuObsclkCtrlClkSelProxyW<Cfg0McuObsclkCtrlProxySpec> {
        McuObsclkCtrlClkSelProxyW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
MCU_OBSCLK pin clock selection output divider"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsclk_ctrl_clk_div_proxy(
        &mut self,
    ) -> McuObsclkCtrlClkDivProxyW<Cfg0McuObsclkCtrlProxySpec> {
        McuObsclkCtrlClkDivProxyW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Load the output divider value"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsclk_ctrl_clk_div_ld_proxy(
        &mut self,
    ) -> McuObsclkCtrlClkDivLdProxyW<Cfg0McuObsclkCtrlProxySpec> {
        McuObsclkCtrlClkDivLdProxyW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
MCU_OBSCLK pin output mux selection."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsclk_ctrl_out_mux_sel_proxy(
        &mut self,
    ) -> McuObsclkCtrlOutMuxSelProxyW<Cfg0McuObsclkCtrlProxySpec> {
        McuObsclkCtrlOutMuxSelProxyW::new(self, 24)
    }
}
#[doc = "CFG0_MCU_OBSCLK_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_obsclk_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_obsclk_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuObsclkCtrlProxySpec;
impl crate::RegisterSpec for Cfg0McuObsclkCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_obsclk_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0McuObsclkCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_obsclk_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0McuObsclkCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_OBSCLK_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0McuObsclkCtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
