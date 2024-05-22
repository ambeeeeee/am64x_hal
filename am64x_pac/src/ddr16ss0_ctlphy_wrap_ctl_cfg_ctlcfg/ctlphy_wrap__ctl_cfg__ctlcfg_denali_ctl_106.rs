#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_106` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl106Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_106` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl106Spec>;
#[doc = "Field `PWRUP_SREFRESH_EXIT` reader - 0:0\\]
Allow powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
pub type PwrupSrefreshExitR = crate::BitReader;
#[doc = "Field `PWRUP_SREFRESH_EXIT` writer - 0:0\\]
Allow powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
pub type PwrupSrefreshExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SREFRESH_EXIT_NO_REFRESH` reader - 8:8\\]
Disables the automatic refresh request associated with self-refresh exit. Set to 1 to disable."]
pub type SrefreshExitNoRefreshR = crate::BitReader;
#[doc = "Field `SREFRESH_EXIT_NO_REFRESH` writer - 8:8\\]
Disables the automatic refresh request associated with self-refresh exit. Set to 1 to disable."]
pub type SrefreshExitNoRefreshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_QUICK_SREFRESH` reader - 16:16\\]
Allow user to interrupt memory initialization to enter self-refresh mode. Set to 1 to allow interruption."]
pub type EnableQuickSrefreshR = crate::BitReader;
#[doc = "Field `ENABLE_QUICK_SREFRESH` writer - 16:16\\]
Allow user to interrupt memory initialization to enter self-refresh mode. Set to 1 to allow interruption."]
pub type EnableQuickSrefreshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKE_DELAY` reader - 26:24\\]
Additional cycles to delay CKE for status reporting."]
pub type CkeDelayR = crate::FieldReader;
#[doc = "Field `CKE_DELAY` writer - 26:24\\]
Additional cycles to delay CKE for status reporting."]
pub type CkeDelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Allow powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pwrup_srefresh_exit(&self) -> PwrupSrefreshExitR {
        PwrupSrefreshExitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Disables the automatic refresh request associated with self-refresh exit. Set to 1 to disable."]
    #[inline(always)]
    pub fn srefresh_exit_no_refresh(&self) -> SrefreshExitNoRefreshR {
        SrefreshExitNoRefreshR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Allow user to interrupt memory initialization to enter self-refresh mode. Set to 1 to allow interruption."]
    #[inline(always)]
    pub fn enable_quick_srefresh(&self) -> EnableQuickSrefreshR {
        EnableQuickSrefreshR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Additional cycles to delay CKE for status reporting."]
    #[inline(always)]
    pub fn cke_delay(&self) -> CkeDelayR {
        CkeDelayR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Allow powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pwrup_srefresh_exit(
        &mut self,
    ) -> PwrupSrefreshExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl106Spec> {
        PwrupSrefreshExitW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Disables the automatic refresh request associated with self-refresh exit. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn srefresh_exit_no_refresh(
        &mut self,
    ) -> SrefreshExitNoRefreshW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl106Spec> {
        SrefreshExitNoRefreshW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Allow user to interrupt memory initialization to enter self-refresh mode. Set to 1 to allow interruption."]
    #[inline(always)]
    #[must_use]
    pub fn enable_quick_srefresh(
        &mut self,
    ) -> EnableQuickSrefreshW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl106Spec> {
        EnableQuickSrefreshW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Additional cycles to delay CKE for status reporting."]
    #[inline(always)]
    #[must_use]
    pub fn cke_delay(&mut self) -> CkeDelayW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl106Spec> {
        CkeDelayW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_106\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_106::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_106::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl106Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl106Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_106::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl106Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_106::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl106Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_106 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl106Spec {
    const RESET_VALUE: u32 = 0;
}
