#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_165` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl165Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_165` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl165Spec>;
#[doc = "Field `LPI_TIMER_WAKEUP_F2` reader - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when the LPI timer expires. FC=2"]
pub type LpiTimerWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_TIMER_WAKEUP_F2` writer - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when the LPI timer expires. FC=2"]
pub type LpiTimerWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_WAKEUP_EN` reader - 13:8\\]
Enables the various low power state wakeup parameters for LPI request uses. Bit \\[0\\]
enables controller idle wakeup, bit \\[1\\]
enables power-down wakeup, bit \\[2\\]
enables either self-refresh short, self-refresh long with or without mem clk gating, either self-refresh power-down short, or self-refresh power-down long with or without mem clk gating, bit \\[3\\]
enables self-refresh long with mem and ctlr clk gating or self-refresh power-down long with mem and ctlr clk gating, bit \\[4\\]
enables the LPI timer expire wakeup, and bit \\[5\\]
is reserved. Set each bit to 1 to enable the respective LP_WAKEUP value for the LPI request."]
pub type LpiWakeupEnR = crate::FieldReader;
#[doc = "Field `LPI_WAKEUP_EN` writer - 13:8\\]
Enables the various low power state wakeup parameters for LPI request uses. Bit \\[0\\]
enables controller idle wakeup, bit \\[1\\]
enables power-down wakeup, bit \\[2\\]
enables either self-refresh short, self-refresh long with or without mem clk gating, either self-refresh power-down short, or self-refresh power-down long with or without mem clk gating, bit \\[3\\]
enables self-refresh long with mem and ctlr clk gating or self-refresh power-down long with mem and ctlr clk gating, bit \\[4\\]
enables the LPI timer expire wakeup, and bit \\[5\\]
is reserved. Set each bit to 1 to enable the respective LP_WAKEUP value for the LPI request."]
pub type LpiWakeupEnW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LPI_CTRL_REQ_EN` reader - 16:16\\]
Enables the dfi_lpi_ctrl_req signal for the LPI. This signal is only relevant for DFI versions 3.1 and beyond. Set to 1 to enable or clear to 0 to disable."]
pub type LpiCtrlReqEnR = crate::BitReader;
#[doc = "Field `LPI_CTRL_REQ_EN` writer - 16:16\\]
Enables the dfi_lpi_ctrl_req signal for the LPI. This signal is only relevant for DFI versions 3.1 and beyond. Set to 1 to enable or clear to 0 to disable."]
pub type LpiCtrlReqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when the LPI timer expires. FC=2"]
    #[inline(always)]
    pub fn lpi_timer_wakeup_f2(&self) -> LpiTimerWakeupF2R {
        LpiTimerWakeupF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Enables the various low power state wakeup parameters for LPI request uses. Bit \\[0\\]
enables controller idle wakeup, bit \\[1\\]
enables power-down wakeup, bit \\[2\\]
enables either self-refresh short, self-refresh long with or without mem clk gating, either self-refresh power-down short, or self-refresh power-down long with or without mem clk gating, bit \\[3\\]
enables self-refresh long with mem and ctlr clk gating or self-refresh power-down long with mem and ctlr clk gating, bit \\[4\\]
enables the LPI timer expire wakeup, and bit \\[5\\]
is reserved. Set each bit to 1 to enable the respective LP_WAKEUP value for the LPI request."]
    #[inline(always)]
    pub fn lpi_wakeup_en(&self) -> LpiWakeupEnR {
        LpiWakeupEnR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables the dfi_lpi_ctrl_req signal for the LPI. This signal is only relevant for DFI versions 3.1 and beyond. Set to 1 to enable or clear to 0 to disable."]
    #[inline(always)]
    pub fn lpi_ctrl_req_en(&self) -> LpiCtrlReqEnR {
        LpiCtrlReqEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when the LPI timer expires. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_timer_wakeup_f2(
        &mut self,
    ) -> LpiTimerWakeupF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl165Spec> {
        LpiTimerWakeupF2W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Enables the various low power state wakeup parameters for LPI request uses. Bit \\[0\\]
enables controller idle wakeup, bit \\[1\\]
enables power-down wakeup, bit \\[2\\]
enables either self-refresh short, self-refresh long with or without mem clk gating, either self-refresh power-down short, or self-refresh power-down long with or without mem clk gating, bit \\[3\\]
enables self-refresh long with mem and ctlr clk gating or self-refresh power-down long with mem and ctlr clk gating, bit \\[4\\]
enables the LPI timer expire wakeup, and bit \\[5\\]
is reserved. Set each bit to 1 to enable the respective LP_WAKEUP value for the LPI request."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_wakeup_en(&mut self) -> LpiWakeupEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl165Spec> {
        LpiWakeupEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables the dfi_lpi_ctrl_req signal for the LPI. This signal is only relevant for DFI versions 3.1 and beyond. Set to 1 to enable or clear to 0 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_ctrl_req_en(&mut self) -> LpiCtrlReqEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl165Spec> {
        LpiCtrlReqEnW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_165\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_165::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_165::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl165Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl165Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_165::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl165Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_165::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl165Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_165 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl165Spec {
    const RESET_VALUE: u32 = 0;
}
