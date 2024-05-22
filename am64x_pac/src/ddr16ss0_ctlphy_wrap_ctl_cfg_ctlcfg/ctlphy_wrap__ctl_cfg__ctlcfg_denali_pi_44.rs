#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_44` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi44Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_44` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi44Spec>;
#[doc = "Field `PI_RDLVL_GATE_DISABLE_DFS` reader - 0:0\\]
Disables automatic gate training on freq change. Set to 1 to disable rdlvl_gate on dfs,Set to 0 to enable rdlvl_gate on dfs."]
pub type PiRdlvlGateDisableDfsR = crate::BitReader;
#[doc = "Field `PI_RDLVL_GATE_DISABLE_DFS` writer - 0:0\\]
Disables automatic gate training on freq change. Set to 1 to disable rdlvl_gate on dfs,Set to 0 to enable rdlvl_gate on dfs."]
pub type PiRdlvlGateDisableDfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_ON_MPD_EXIT` reader - 8:8\\]
Enables automatic data eye training on a maximum power down mode exit. Set to 1 to enable."]
pub type PiRdlvlOnMpdExitR = crate::BitReader;
#[doc = "Field `PI_RDLVL_ON_MPD_EXIT` writer - 8:8\\]
Enables automatic data eye training on a maximum power down mode exit. Set to 1 to enable."]
pub type PiRdlvlOnMpdExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_GATE_ON_MPD_EXIT` reader - 16:16\\]
Enables automatic gate training on a maximum power down mode exit. Set to 1 to enable."]
pub type PiRdlvlGateOnMpdExitR = crate::BitReader;
#[doc = "Field `PI_RDLVL_GATE_ON_MPD_EXIT` writer - 16:16\\]
Enables automatic gate training on a maximum power down mode exit. Set to 1 to enable."]
pub type PiRdlvlGateOnMpdExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_ROTATE` reader - 24:24\\]
Enables rotational CS for interval data eye training. Set to 1 for rotating CS."]
pub type PiRdlvlRotateR = crate::BitReader;
#[doc = "Field `PI_RDLVL_ROTATE` writer - 24:24\\]
Enables rotational CS for interval data eye training. Set to 1 for rotating CS."]
pub type PiRdlvlRotateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disables automatic gate training on freq change. Set to 1 to disable rdlvl_gate on dfs,Set to 0 to enable rdlvl_gate on dfs."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_disable_dfs(&self) -> PiRdlvlGateDisableDfsR {
        PiRdlvlGateDisableDfsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables automatic data eye training on a maximum power down mode exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_on_mpd_exit(&self) -> PiRdlvlOnMpdExitR {
        PiRdlvlOnMpdExitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables automatic gate training on a maximum power down mode exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_on_mpd_exit(&self) -> PiRdlvlGateOnMpdExitR {
        PiRdlvlGateOnMpdExitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables rotational CS for interval data eye training. Set to 1 for rotating CS."]
    #[inline(always)]
    pub fn pi_rdlvl_rotate(&self) -> PiRdlvlRotateR {
        PiRdlvlRotateR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Disables automatic gate training on freq change. Set to 1 to disable rdlvl_gate on dfs,Set to 0 to enable rdlvl_gate on dfs."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_disable_dfs(
        &mut self,
    ) -> PiRdlvlGateDisableDfsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi44Spec> {
        PiRdlvlGateDisableDfsW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables automatic data eye training on a maximum power down mode exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_on_mpd_exit(
        &mut self,
    ) -> PiRdlvlOnMpdExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi44Spec> {
        PiRdlvlOnMpdExitW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables automatic gate training on a maximum power down mode exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_on_mpd_exit(
        &mut self,
    ) -> PiRdlvlGateOnMpdExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi44Spec> {
        PiRdlvlGateOnMpdExitW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables rotational CS for interval data eye training. Set to 1 for rotating CS."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_rotate(&mut self) -> PiRdlvlRotateW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi44Spec> {
        PiRdlvlRotateW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi44Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_44::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi44Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_44::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_44 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi44Spec {
    const RESET_VALUE: u32 = 0;
}
