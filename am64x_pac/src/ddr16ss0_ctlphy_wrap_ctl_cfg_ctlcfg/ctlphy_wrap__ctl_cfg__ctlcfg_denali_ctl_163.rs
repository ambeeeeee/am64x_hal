#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_163` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl163Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_163` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl163Spec>;
#[doc = "Field `LPI_IDLE_WAKEUP_F2` reader - 3:0\\]
Defines the DFI tLP_CTRL_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when controller is idle. FC=2"]
pub type LpiIdleWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_IDLE_WAKEUP_F2` writer - 3:0\\]
Defines the DFI tLP_CTRL_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when controller is idle. FC=2"]
pub type LpiIdleWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SR_SHORT_WAKEUP_F2` reader - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when LPDDR4 memory is in the self-refresh short state \\[with or without memory clock gating\\]. For LPDDR4, SR_SHORT is used to send few commands so this wakeup time must be cleared to ZERO and no LPI request needs to be asserted. FC=2"]
pub type LpiSrShortWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_SR_SHORT_WAKEUP_F2` writer - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when LPDDR4 memory is in the self-refresh short state \\[with or without memory clock gating\\]. For LPDDR4, SR_SHORT is used to send few commands so this wakeup time must be cleared to ZERO and no LPI request needs to be asserted. FC=2"]
pub type LpiSrShortWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SR_LONG_WAKEUP_F2` reader - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long state \\[with or without memory clock gating\\]. FC=2"]
pub type LpiSrLongWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_SR_LONG_WAKEUP_F2` writer - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long state \\[with or without memory clock gating\\]. FC=2"]
pub type LpiSrLongWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SR_LONG_MCCLK_GATE_WAKEUP_F2` reader - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long with memory and controller clock gating state. FC=2"]
pub type LpiSrLongMcclkGateWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_SR_LONG_MCCLK_GATE_WAKEUP_F2` writer - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long with memory and controller clock gating state. FC=2"]
pub type LpiSrLongMcclkGateWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tLP_CTRL_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when controller is idle. FC=2"]
    #[inline(always)]
    pub fn lpi_idle_wakeup_f2(&self) -> LpiIdleWakeupF2R {
        LpiIdleWakeupF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when LPDDR4 memory is in the self-refresh short state \\[with or without memory clock gating\\]. For LPDDR4, SR_SHORT is used to send few commands so this wakeup time must be cleared to ZERO and no LPI request needs to be asserted. FC=2"]
    #[inline(always)]
    pub fn lpi_sr_short_wakeup_f2(&self) -> LpiSrShortWakeupF2R {
        LpiSrShortWakeupF2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long state \\[with or without memory clock gating\\]. FC=2"]
    #[inline(always)]
    pub fn lpi_sr_long_wakeup_f2(&self) -> LpiSrLongWakeupF2R {
        LpiSrLongWakeupF2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long with memory and controller clock gating state. FC=2"]
    #[inline(always)]
    pub fn lpi_sr_long_mcclk_gate_wakeup_f2(&self) -> LpiSrLongMcclkGateWakeupF2R {
        LpiSrLongMcclkGateWakeupF2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tLP_CTRL_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when controller is idle. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_idle_wakeup_f2(
        &mut self,
    ) -> LpiIdleWakeupF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl163Spec> {
        LpiIdleWakeupF2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when LPDDR4 memory is in the self-refresh short state \\[with or without memory clock gating\\]. For LPDDR4, SR_SHORT is used to send few commands so this wakeup time must be cleared to ZERO and no LPI request needs to be asserted. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_sr_short_wakeup_f2(
        &mut self,
    ) -> LpiSrShortWakeupF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl163Spec> {
        LpiSrShortWakeupF2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long state \\[with or without memory clock gating\\]. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_sr_long_wakeup_f2(
        &mut self,
    ) -> LpiSrLongWakeupF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl163Spec> {
        LpiSrLongWakeupF2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long with memory and controller clock gating state. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_sr_long_mcclk_gate_wakeup_f2(
        &mut self,
    ) -> LpiSrLongMcclkGateWakeupF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl163Spec> {
        LpiSrLongMcclkGateWakeupF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_163\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_163::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_163::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl163Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl163Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_163::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl163Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_163::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl163Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_163 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl163Spec {
    const RESET_VALUE: u32 = 0;
}
