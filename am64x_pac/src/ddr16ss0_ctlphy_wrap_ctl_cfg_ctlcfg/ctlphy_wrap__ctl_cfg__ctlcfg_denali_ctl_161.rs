#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_161` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl161Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_161` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl161Spec>;
#[doc = "Field `LPI_SR_SHORT_WAKEUP_F1` reader - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when LPDDR4 memory is in the self-refresh short state \\[with or without memory clock gating\\]. For LPDDR4, SR_SHORT is used to send few commands so this wakeup time must be cleared to ZERO and no LPI request needs to be asserted. FC=1"]
pub type LpiSrShortWakeupF1R = crate::FieldReader;
#[doc = "Field `LPI_SR_SHORT_WAKEUP_F1` writer - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when LPDDR4 memory is in the self-refresh short state \\[with or without memory clock gating\\]. For LPDDR4, SR_SHORT is used to send few commands so this wakeup time must be cleared to ZERO and no LPI request needs to be asserted. FC=1"]
pub type LpiSrShortWakeupF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SR_LONG_WAKEUP_F1` reader - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long state \\[with or without memory clock gating\\]. FC=1"]
pub type LpiSrLongWakeupF1R = crate::FieldReader;
#[doc = "Field `LPI_SR_LONG_WAKEUP_F1` writer - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long state \\[with or without memory clock gating\\]. FC=1"]
pub type LpiSrLongWakeupF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SR_LONG_MCCLK_GATE_WAKEUP_F1` reader - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long with memory and controller clock gating state. FC=1"]
pub type LpiSrLongMcclkGateWakeupF1R = crate::FieldReader;
#[doc = "Field `LPI_SR_LONG_MCCLK_GATE_WAKEUP_F1` writer - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long with memory and controller clock gating state. FC=1"]
pub type LpiSrLongMcclkGateWakeupF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_PD_WAKEUP_F1` reader - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in any of the power-down states \\[with or without memory clock gating\\]. FC=1"]
pub type LpiPdWakeupF1R = crate::FieldReader;
#[doc = "Field `LPI_PD_WAKEUP_F1` writer - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in any of the power-down states \\[with or without memory clock gating\\]. FC=1"]
pub type LpiPdWakeupF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when LPDDR4 memory is in the self-refresh short state \\[with or without memory clock gating\\]. For LPDDR4, SR_SHORT is used to send few commands so this wakeup time must be cleared to ZERO and no LPI request needs to be asserted. FC=1"]
    #[inline(always)]
    pub fn lpi_sr_short_wakeup_f1(&self) -> LpiSrShortWakeupF1R {
        LpiSrShortWakeupF1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long state \\[with or without memory clock gating\\]. FC=1"]
    #[inline(always)]
    pub fn lpi_sr_long_wakeup_f1(&self) -> LpiSrLongWakeupF1R {
        LpiSrLongWakeupF1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long with memory and controller clock gating state. FC=1"]
    #[inline(always)]
    pub fn lpi_sr_long_mcclk_gate_wakeup_f1(&self) -> LpiSrLongMcclkGateWakeupF1R {
        LpiSrLongMcclkGateWakeupF1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in any of the power-down states \\[with or without memory clock gating\\]. FC=1"]
    #[inline(always)]
    pub fn lpi_pd_wakeup_f1(&self) -> LpiPdWakeupF1R {
        LpiPdWakeupF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when LPDDR4 memory is in the self-refresh short state \\[with or without memory clock gating\\]. For LPDDR4, SR_SHORT is used to send few commands so this wakeup time must be cleared to ZERO and no LPI request needs to be asserted. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_sr_short_wakeup_f1(
        &mut self,
    ) -> LpiSrShortWakeupF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl161Spec> {
        LpiSrShortWakeupF1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long state \\[with or without memory clock gating\\]. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_sr_long_wakeup_f1(
        &mut self,
    ) -> LpiSrLongWakeupF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl161Spec> {
        LpiSrLongWakeupF1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh long with memory and controller clock gating state. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_sr_long_mcclk_gate_wakeup_f1(
        &mut self,
    ) -> LpiSrLongMcclkGateWakeupF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl161Spec> {
        LpiSrLongMcclkGateWakeupF1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in any of the power-down states \\[with or without memory clock gating\\]. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_pd_wakeup_f1(&mut self) -> LpiPdWakeupF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl161Spec> {
        LpiPdWakeupF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_161\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_161::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_161::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl161Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl161Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_161::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl161Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_161::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl161Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_161 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl161Spec {
    const RESET_VALUE: u32 = 0;
}
