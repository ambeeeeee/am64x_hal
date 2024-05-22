#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_55` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi55Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_55` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi55Spec>;
#[doc = "Field `PI_CALVL_PERIODIC` reader - 0:0\\]
Enables the use of the dfi_lvl_periodic signal during CA training. Set to 1 to enable."]
pub type PiCalvlPeriodicR = crate::BitReader;
#[doc = "Field `PI_CALVL_PERIODIC` writer - 0:0\\]
Enables the use of the dfi_lvl_periodic signal during CA training. Set to 1 to enable."]
pub type PiCalvlPeriodicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CALVL_ON_SREF_EXIT` reader - 8:8\\]
Enables automatic CA training on a self-refresh exit. Set to 1 to enable."]
pub type PiCalvlOnSrefExitR = crate::BitReader;
#[doc = "Field `PI_CALVL_ON_SREF_EXIT` writer - 8:8\\]
Enables automatic CA training on a self-refresh exit. Set to 1 to enable."]
pub type PiCalvlOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CALVL_DISABLE_DFS` reader - 16:16\\]
Disables automatic CA training on freq change. Set to 1 to disable CA training on dfs, Set to 0 to enable CA training ."]
pub type PiCalvlDisableDfsR = crate::BitReader;
#[doc = "Field `PI_CALVL_DISABLE_DFS` writer - 16:16\\]
Disables automatic CA training on freq change. Set to 1 to disable CA training on dfs, Set to 0 to enable CA training ."]
pub type PiCalvlDisableDfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CALVL_ROTATE` reader - 24:24\\]
Enables rotational CS for interval CA training. Set to 1 for rotating CS."]
pub type PiCalvlRotateR = crate::BitReader;
#[doc = "Field `PI_CALVL_ROTATE` writer - 24:24\\]
Enables rotational CS for interval CA training. Set to 1 for rotating CS."]
pub type PiCalvlRotateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the use of the dfi_lvl_periodic signal during CA training. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_calvl_periodic(&self) -> PiCalvlPeriodicR {
        PiCalvlPeriodicR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables automatic CA training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_calvl_on_sref_exit(&self) -> PiCalvlOnSrefExitR {
        PiCalvlOnSrefExitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Disables automatic CA training on freq change. Set to 1 to disable CA training on dfs, Set to 0 to enable CA training ."]
    #[inline(always)]
    pub fn pi_calvl_disable_dfs(&self) -> PiCalvlDisableDfsR {
        PiCalvlDisableDfsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables rotational CS for interval CA training. Set to 1 for rotating CS."]
    #[inline(always)]
    pub fn pi_calvl_rotate(&self) -> PiCalvlRotateR {
        PiCalvlRotateR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the use of the dfi_lvl_periodic signal during CA training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_periodic(
        &mut self,
    ) -> PiCalvlPeriodicW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi55Spec> {
        PiCalvlPeriodicW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables automatic CA training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_on_sref_exit(
        &mut self,
    ) -> PiCalvlOnSrefExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi55Spec> {
        PiCalvlOnSrefExitW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Disables automatic CA training on freq change. Set to 1 to disable CA training on dfs, Set to 0 to enable CA training ."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_disable_dfs(
        &mut self,
    ) -> PiCalvlDisableDfsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi55Spec> {
        PiCalvlDisableDfsW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables rotational CS for interval CA training. Set to 1 for rotating CS."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_rotate(&mut self) -> PiCalvlRotateW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi55Spec> {
        PiCalvlRotateW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_55\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_55::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_55::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi55Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi55Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_55::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi55Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_55::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi55Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_55 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi55Spec {
    const RESET_VALUE: u32 = 0;
}
