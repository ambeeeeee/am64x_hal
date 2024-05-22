#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_160` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl160Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_160` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl160Spec>;
#[doc = "Field `LPI_SRPD_LONG_WAKEUP_F0` reader - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long state \\[with or without memory clock gating\\]. FC=0"]
pub type LpiSrpdLongWakeupF0R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_LONG_WAKEUP_F0` writer - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long state \\[with or without memory clock gating\\]. FC=0"]
pub type LpiSrpdLongWakeupF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SRPD_LONG_MCCLK_GATE_WAKEUP_F0` reader - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long with memory and controller clock gating state. FC=0"]
pub type LpiSrpdLongMcclkGateWakeupF0R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_LONG_MCCLK_GATE_WAKEUP_F0` writer - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long with memory and controller clock gating state. FC=0"]
pub type LpiSrpdLongMcclkGateWakeupF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_TIMER_WAKEUP_F0` reader - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when the LPI timer expires. FC=0"]
pub type LpiTimerWakeupF0R = crate::FieldReader;
#[doc = "Field `LPI_TIMER_WAKEUP_F0` writer - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when the LPI timer expires. FC=0"]
pub type LpiTimerWakeupF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_IDLE_WAKEUP_F1` reader - 27:24\\]
Defines the DFI tLP_CTRL_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when controller is idle. FC=1"]
pub type LpiIdleWakeupF1R = crate::FieldReader;
#[doc = "Field `LPI_IDLE_WAKEUP_F1` writer - 27:24\\]
Defines the DFI tLP_CTRL_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when controller is idle. FC=1"]
pub type LpiIdleWakeupF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long state \\[with or without memory clock gating\\]. FC=0"]
    #[inline(always)]
    pub fn lpi_srpd_long_wakeup_f0(&self) -> LpiSrpdLongWakeupF0R {
        LpiSrpdLongWakeupF0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long with memory and controller clock gating state. FC=0"]
    #[inline(always)]
    pub fn lpi_srpd_long_mcclk_gate_wakeup_f0(&self) -> LpiSrpdLongMcclkGateWakeupF0R {
        LpiSrpdLongMcclkGateWakeupF0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when the LPI timer expires. FC=0"]
    #[inline(always)]
    pub fn lpi_timer_wakeup_f0(&self) -> LpiTimerWakeupF0R {
        LpiTimerWakeupF0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tLP_CTRL_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when controller is idle. FC=1"]
    #[inline(always)]
    pub fn lpi_idle_wakeup_f1(&self) -> LpiIdleWakeupF1R {
        LpiIdleWakeupF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long state \\[with or without memory clock gating\\]. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_long_wakeup_f0(
        &mut self,
    ) -> LpiSrpdLongWakeupF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl160Spec> {
        LpiSrpdLongWakeupF0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long with memory and controller clock gating state. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_long_mcclk_gate_wakeup_f0(
        &mut self,
    ) -> LpiSrpdLongMcclkGateWakeupF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl160Spec> {
        LpiSrpdLongMcclkGateWakeupF0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when the LPI timer expires. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_timer_wakeup_f0(
        &mut self,
    ) -> LpiTimerWakeupF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl160Spec> {
        LpiTimerWakeupF0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tLP_CTRL_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when controller is idle. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_idle_wakeup_f1(
        &mut self,
    ) -> LpiIdleWakeupF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl160Spec> {
        LpiIdleWakeupF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_160\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_160::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_160::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl160Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl160Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_160::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl160Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_160::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl160Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_160 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl160Spec {
    const RESET_VALUE: u32 = 0;
}
