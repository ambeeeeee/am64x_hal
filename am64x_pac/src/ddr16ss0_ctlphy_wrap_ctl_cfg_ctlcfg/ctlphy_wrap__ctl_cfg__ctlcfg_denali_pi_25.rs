#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_25` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi25Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_25` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi25Spec>;
#[doc = "Field `PI_WRLVL_INTERVAL` reader - 15:0\\]
Number of long count sequences counted between automatic write leveling commands."]
pub type PiWrlvlIntervalR = crate::FieldReader<u16>;
#[doc = "Field `PI_WRLVL_INTERVAL` writer - 15:0\\]
Number of long count sequences counted between automatic write leveling commands."]
pub type PiWrlvlIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_WRLVL_ON_SREF_EXIT` reader - 16:16\\]
Enables automatic write leveling on a self-refresh exit. Set to 1 to enable."]
pub type PiWrlvlOnSrefExitR = crate::BitReader;
#[doc = "Field `PI_WRLVL_ON_SREF_EXIT` writer - 16:16\\]
Enables automatic write leveling on a self-refresh exit. Set to 1 to enable."]
pub type PiWrlvlOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WRLVL_DISABLE_DFS` reader - 24:24\\]
Disable automatic write leveling on freq change. Set to 1 to disable wrlvl on dfs,set 0 enable wrlvl on dfs."]
pub type PiWrlvlDisableDfsR = crate::BitReader;
#[doc = "Field `PI_WRLVL_DISABLE_DFS` writer - 24:24\\]
Disable automatic write leveling on freq change. Set to 1 to disable wrlvl on dfs,set 0 enable wrlvl on dfs."]
pub type PiWrlvlDisableDfsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Number of long count sequences counted between automatic write leveling commands."]
    #[inline(always)]
    pub fn pi_wrlvl_interval(&self) -> PiWrlvlIntervalR {
        PiWrlvlIntervalR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables automatic write leveling on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_wrlvl_on_sref_exit(&self) -> PiWrlvlOnSrefExitR {
        PiWrlvlOnSrefExitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Disable automatic write leveling on freq change. Set to 1 to disable wrlvl on dfs,set 0 enable wrlvl on dfs."]
    #[inline(always)]
    pub fn pi_wrlvl_disable_dfs(&self) -> PiWrlvlDisableDfsR {
        PiWrlvlDisableDfsR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Number of long count sequences counted between automatic write leveling commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_interval(
        &mut self,
    ) -> PiWrlvlIntervalW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi25Spec> {
        PiWrlvlIntervalW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables automatic write leveling on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_on_sref_exit(
        &mut self,
    ) -> PiWrlvlOnSrefExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi25Spec> {
        PiWrlvlOnSrefExitW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Disable automatic write leveling on freq change. Set to 1 to disable wrlvl on dfs,set 0 enable wrlvl on dfs."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_disable_dfs(
        &mut self,
    ) -> PiWrlvlDisableDfsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi25Spec> {
        PiWrlvlDisableDfsW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi25Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_25::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi25Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_25::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_25 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi25Spec {
    const RESET_VALUE: u32 = 0;
}
