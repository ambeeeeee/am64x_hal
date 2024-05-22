#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_166` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl166Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_166` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl166Spec>;
#[doc = "Field `LPI_TIMER_COUNT` reader - 11:0\\]
Defines the LPI timer count."]
pub type LpiTimerCountR = crate::FieldReader<u16>;
#[doc = "Field `LPI_TIMER_COUNT` writer - 11:0\\]
Defines the LPI timer count."]
pub type LpiTimerCountW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `LPI_WAKEUP_TIMEOUT` reader - 27:16\\]
Defines the LPI timeout time, the maximum cycles between a dfi_lp_req de-assertion and a dfi_lp_ack de-assertion. If this value is exceeded, an interrupt will occur."]
pub type LpiWakeupTimeoutR = crate::FieldReader<u16>;
#[doc = "Field `LPI_WAKEUP_TIMEOUT` writer - 27:16\\]
Defines the LPI timeout time, the maximum cycles between a dfi_lp_req de-assertion and a dfi_lp_ack de-assertion. If this value is exceeded, an interrupt will occur."]
pub type LpiWakeupTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Defines the LPI timer count."]
    #[inline(always)]
    pub fn lpi_timer_count(&self) -> LpiTimerCountR {
        LpiTimerCountR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Defines the LPI timeout time, the maximum cycles between a dfi_lp_req de-assertion and a dfi_lp_ack de-assertion. If this value is exceeded, an interrupt will occur."]
    #[inline(always)]
    pub fn lpi_wakeup_timeout(&self) -> LpiWakeupTimeoutR {
        LpiWakeupTimeoutR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Defines the LPI timer count."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_timer_count(&mut self) -> LpiTimerCountW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl166Spec> {
        LpiTimerCountW::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Defines the LPI timeout time, the maximum cycles between a dfi_lp_req de-assertion and a dfi_lp_ack de-assertion. If this value is exceeded, an interrupt will occur."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_wakeup_timeout(
        &mut self,
    ) -> LpiWakeupTimeoutW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl166Spec> {
        LpiWakeupTimeoutW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_166\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_166::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_166::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl166Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl166Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_166::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl166Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_166::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl166Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_166 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl166Spec {
    const RESET_VALUE: u32 = 0;
}
