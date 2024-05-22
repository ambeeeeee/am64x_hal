#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_312` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy312Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_312` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy312Spec>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_NUM_WINDOWS_OBS_1` reader - 1:0\\]
Observation register containing read leveling number of windows found for slice 1. READ-ONLY"]
pub type PhyRdlvlRddqsDqNumWindowsObs1R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_NUM_WINDOWS_OBS_1` writer - 1:0\\]
Observation register containing read leveling number of windows found for slice 1. READ-ONLY"]
pub type PhyRdlvlRddqsDqNumWindowsObs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Observation register containing read leveling number of windows found for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_num_windows_obs_1(&self) -> PhyRdlvlRddqsDqNumWindowsObs1R {
        PhyRdlvlRddqsDqNumWindowsObs1R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Observation register containing read leveling number of windows found for slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_rddqs_dq_num_windows_obs_1(
        &mut self,
    ) -> PhyRdlvlRddqsDqNumWindowsObs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy312Spec> {
        PhyRdlvlRddqsDqNumWindowsObs1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_312\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_312::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_312::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy312Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy312Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_312::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy312Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_312::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy312Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_312 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy312Spec {
    const RESET_VALUE: u32 = 0;
}
