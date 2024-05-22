#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_164` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl164Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_164` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl164Spec>;
#[doc = "Field `LPI_PD_WAKEUP_F2` reader - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in any of the power-down states \\[with or without memory clock gating\\]. FC=2"]
pub type LpiPdWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_PD_WAKEUP_F2` writer - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in any of the power-down states \\[with or without memory clock gating\\]. FC=2"]
pub type LpiPdWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SRPD_SHORT_WAKEUP_F2` reader - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down short state \\[with or without memory clock gating\\]. FC=2"]
pub type LpiSrpdShortWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_SHORT_WAKEUP_F2` writer - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down short state \\[with or without memory clock gating\\]. FC=2"]
pub type LpiSrpdShortWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SRPD_LONG_WAKEUP_F2` reader - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long state \\[with or without memory clock gating\\]. FC=2"]
pub type LpiSrpdLongWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_LONG_WAKEUP_F2` writer - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long state \\[with or without memory clock gating\\]. FC=2"]
pub type LpiSrpdLongWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPI_SRPD_LONG_MCCLK_GATE_WAKEUP_F2` reader - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long with memory and controller clock gating state. FC=2"]
pub type LpiSrpdLongMcclkGateWakeupF2R = crate::FieldReader;
#[doc = "Field `LPI_SRPD_LONG_MCCLK_GATE_WAKEUP_F2` writer - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long with memory and controller clock gating state. FC=2"]
pub type LpiSrpdLongMcclkGateWakeupF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in any of the power-down states \\[with or without memory clock gating\\]. FC=2"]
    #[inline(always)]
    pub fn lpi_pd_wakeup_f2(&self) -> LpiPdWakeupF2R {
        LpiPdWakeupF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down short state \\[with or without memory clock gating\\]. FC=2"]
    #[inline(always)]
    pub fn lpi_srpd_short_wakeup_f2(&self) -> LpiSrpdShortWakeupF2R {
        LpiSrpdShortWakeupF2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long state \\[with or without memory clock gating\\]. FC=2"]
    #[inline(always)]
    pub fn lpi_srpd_long_wakeup_f2(&self) -> LpiSrpdLongWakeupF2R {
        LpiSrpdLongWakeupF2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long with memory and controller clock gating state. FC=2"]
    #[inline(always)]
    pub fn lpi_srpd_long_mcclk_gate_wakeup_f2(&self) -> LpiSrpdLongMcclkGateWakeupF2R {
        LpiSrpdLongMcclkGateWakeupF2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in any of the power-down states \\[with or without memory clock gating\\]. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_pd_wakeup_f2(&mut self) -> LpiPdWakeupF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl164Spec> {
        LpiPdWakeupF2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down short state \\[with or without memory clock gating\\]. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_short_wakeup_f2(
        &mut self,
    ) -> LpiSrpdShortWakeupF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl164Spec> {
        LpiSrpdShortWakeupF2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long state \\[with or without memory clock gating\\]. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_long_wakeup_f2(
        &mut self,
    ) -> LpiSrpdLongWakeupF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl164Spec> {
        LpiSrpdLongWakeupF2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the DFI tLP_WAKEUP timing parameter \\[in DFI clocks\\]
to be driven when memory is in the self-refresh power-down long with memory and controller clock gating state. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn lpi_srpd_long_mcclk_gate_wakeup_f2(
        &mut self,
    ) -> LpiSrpdLongMcclkGateWakeupF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl164Spec> {
        LpiSrpdLongMcclkGateWakeupF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_164\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_164::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_164::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl164Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl164Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_164::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl164Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_164::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl164Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_164 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl164Spec {
    const RESET_VALUE: u32 = 0;
}
