#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_43` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi43Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_43` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi43Spec>;
#[doc = "Field `PI_RDLVL_SEQ_EN` reader - 3:0\\]
Specifies the pattern, format and MPR for data eye training."]
pub type PiRdlvlSeqEnR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_SEQ_EN` writer - 3:0\\]
Specifies the pattern, format and MPR for data eye training."]
pub type PiRdlvlSeqEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_RDLVL_ON_SREF_EXIT` reader - 8:8\\]
Enables automatic data eye training on a self-refresh exit. Set to 1 to enable."]
pub type PiRdlvlOnSrefExitR = crate::BitReader;
#[doc = "Field `PI_RDLVL_ON_SREF_EXIT` writer - 8:8\\]
Enables automatic data eye training on a self-refresh exit. Set to 1 to enable."]
pub type PiRdlvlOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_DISABLE_DFS` reader - 16:16\\]
Disables automatic data eye training on freq change. Set to 1 to disable rdlvl on dfs,Set to 0 to enable rdlvl on dfs."]
pub type PiRdlvlDisableDfsR = crate::BitReader;
#[doc = "Field `PI_RDLVL_DISABLE_DFS` writer - 16:16\\]
Disables automatic data eye training on freq change. Set to 1 to disable rdlvl on dfs,Set to 0 to enable rdlvl on dfs."]
pub type PiRdlvlDisableDfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_GATE_ON_SREF_EXIT` reader - 24:24\\]
Enables automatic gate training on a self-refresh exit. Set to 1 to enable."]
pub type PiRdlvlGateOnSrefExitR = crate::BitReader;
#[doc = "Field `PI_RDLVL_GATE_ON_SREF_EXIT` writer - 24:24\\]
Enables automatic gate training on a self-refresh exit. Set to 1 to enable."]
pub type PiRdlvlGateOnSrefExitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Specifies the pattern, format and MPR for data eye training."]
    #[inline(always)]
    pub fn pi_rdlvl_seq_en(&self) -> PiRdlvlSeqEnR {
        PiRdlvlSeqEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables automatic data eye training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_on_sref_exit(&self) -> PiRdlvlOnSrefExitR {
        PiRdlvlOnSrefExitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Disables automatic data eye training on freq change. Set to 1 to disable rdlvl on dfs,Set to 0 to enable rdlvl on dfs."]
    #[inline(always)]
    pub fn pi_rdlvl_disable_dfs(&self) -> PiRdlvlDisableDfsR {
        PiRdlvlDisableDfsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables automatic gate training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_on_sref_exit(&self) -> PiRdlvlGateOnSrefExitR {
        PiRdlvlGateOnSrefExitR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Specifies the pattern, format and MPR for data eye training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_seq_en(&mut self) -> PiRdlvlSeqEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi43Spec> {
        PiRdlvlSeqEnW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables automatic data eye training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_on_sref_exit(
        &mut self,
    ) -> PiRdlvlOnSrefExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi43Spec> {
        PiRdlvlOnSrefExitW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Disables automatic data eye training on freq change. Set to 1 to disable rdlvl on dfs,Set to 0 to enable rdlvl on dfs."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_disable_dfs(
        &mut self,
    ) -> PiRdlvlDisableDfsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi43Spec> {
        PiRdlvlDisableDfsW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables automatic gate training on a self-refresh exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_on_sref_exit(
        &mut self,
    ) -> PiRdlvlGateOnSrefExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi43Spec> {
        PiRdlvlGateOnSrefExitW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_43::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_43::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi43Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_43::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi43Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_43::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi43Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_43 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi43Spec {
    const RESET_VALUE: u32 = 0;
}
