#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_71` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi71Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_71` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi71Spec>;
#[doc = "Field `PI_WDQLVL_INTERVAL` reader - 15:0\\]
Sets the maximum number of long count sequences allowed between automatic write DQ training operations."]
pub type PiWdqlvlIntervalR = crate::FieldReader<u16>;
#[doc = "Field `PI_WDQLVL_INTERVAL` writer - 15:0\\]
Sets the maximum number of long count sequences allowed between automatic write DQ training operations."]
pub type PiWdqlvlIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_WDQLVL_ON_SREF_EXIT` reader - 16:16\\]
Issue a write DQ training command on self-refresh exit."]
pub type PiWdqlvlOnSrefExitR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_ON_SREF_EXIT` writer - 16:16\\]
Issue a write DQ training command on self-refresh exit."]
pub type PiWdqlvlOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_ON_MPD_EXIT` reader - 24:24\\]
Issue a write DQ training command on maximum power saving mode exit."]
pub type PiWdqlvlOnMpdExitR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_ON_MPD_EXIT` writer - 24:24\\]
Issue a write DQ training command on maximum power saving mode exit."]
pub type PiWdqlvlOnMpdExitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Sets the maximum number of long count sequences allowed between automatic write DQ training operations."]
    #[inline(always)]
    pub fn pi_wdqlvl_interval(&self) -> PiWdqlvlIntervalR {
        PiWdqlvlIntervalR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Issue a write DQ training command on self-refresh exit."]
    #[inline(always)]
    pub fn pi_wdqlvl_on_sref_exit(&self) -> PiWdqlvlOnSrefExitR {
        PiWdqlvlOnSrefExitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Issue a write DQ training command on maximum power saving mode exit."]
    #[inline(always)]
    pub fn pi_wdqlvl_on_mpd_exit(&self) -> PiWdqlvlOnMpdExitR {
        PiWdqlvlOnMpdExitR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Sets the maximum number of long count sequences allowed between automatic write DQ training operations."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_interval(
        &mut self,
    ) -> PiWdqlvlIntervalW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi71Spec> {
        PiWdqlvlIntervalW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Issue a write DQ training command on self-refresh exit."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_on_sref_exit(
        &mut self,
    ) -> PiWdqlvlOnSrefExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi71Spec> {
        PiWdqlvlOnSrefExitW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Issue a write DQ training command on maximum power saving mode exit."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_on_mpd_exit(
        &mut self,
    ) -> PiWdqlvlOnMpdExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi71Spec> {
        PiWdqlvlOnMpdExitW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_71\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_71::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_71::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi71Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi71Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_71::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi71Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_71::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi71Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_71 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi71Spec {
    const RESET_VALUE: u32 = 0;
}
