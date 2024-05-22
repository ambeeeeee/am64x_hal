#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_134` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi134Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_134` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi134Spec>;
#[doc = "Field `PI_MC_PWRUP_SREFRESH_EXIT` reader - 0:0\\]
It indicates MC control powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
pub type PiMcPwrupSrefreshExitR = crate::BitReader;
#[doc = "Field `PI_MC_PWRUP_SREFRESH_EXIT` writer - 0:0\\]
It indicates MC control powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
pub type PiMcPwrupSrefreshExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_PWRUP_SREFRESH_EXIT` reader - 8:8\\]
PI control powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
pub type PiPwrupSrefreshExitR = crate::BitReader;
#[doc = "Field `PI_PWRUP_SREFRESH_EXIT` writer - 8:8\\]
PI control powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
pub type PiPwrupSrefreshExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SREFRESH_EXIT_NO_REFRESH` reader - 16:16\\]
Disables the automatic refresh request associated with self-refresh exit. Set to 1 to disable."]
pub type PiSrefreshExitNoRefreshR = crate::BitReader;
#[doc = "Field `PI_SREFRESH_EXIT_NO_REFRESH` writer - 16:16\\]
Disables the automatic refresh request associated with self-refresh exit. Set to 1 to disable."]
pub type PiSrefreshExitNoRefreshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SREF_ENTRY_REQ` reader - 24:24\\]
In PI power up data retention, PI can issued sref entry command. WRITE-ONLY"]
pub type PiSrefEntryReqR = crate::BitReader;
#[doc = "Field `PI_SREF_ENTRY_REQ` writer - 24:24\\]
In PI power up data retention, PI can issued sref entry command. WRITE-ONLY"]
pub type PiSrefEntryReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
It indicates MC control powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_mc_pwrup_srefresh_exit(&self) -> PiMcPwrupSrefreshExitR {
        PiMcPwrupSrefreshExitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
PI control powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_pwrup_srefresh_exit(&self) -> PiPwrupSrefreshExitR {
        PiPwrupSrefreshExitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Disables the automatic refresh request associated with self-refresh exit. Set to 1 to disable."]
    #[inline(always)]
    pub fn pi_srefresh_exit_no_refresh(&self) -> PiSrefreshExitNoRefreshR {
        PiSrefreshExitNoRefreshR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
In PI power up data retention, PI can issued sref entry command. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_sref_entry_req(&self) -> PiSrefEntryReqR {
        PiSrefEntryReqR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
It indicates MC control powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mc_pwrup_srefresh_exit(
        &mut self,
    ) -> PiMcPwrupSrefreshExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi134Spec> {
        PiMcPwrupSrefreshExitW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
PI control powerup via self-refresh instead of full memory initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_pwrup_srefresh_exit(
        &mut self,
    ) -> PiPwrupSrefreshExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi134Spec> {
        PiPwrupSrefreshExitW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Disables the automatic refresh request associated with self-refresh exit. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_srefresh_exit_no_refresh(
        &mut self,
    ) -> PiSrefreshExitNoRefreshW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi134Spec> {
        PiSrefreshExitNoRefreshW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
In PI power up data retention, PI can issued sref entry command. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_sref_entry_req(
        &mut self,
    ) -> PiSrefEntryReqW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi134Spec> {
        PiSrefEntryReqW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_134\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_134::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_134::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi134Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi134Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_134::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi134Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_134::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi134Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_134 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi134Spec {
    const RESET_VALUE: u32 = 0;
}
