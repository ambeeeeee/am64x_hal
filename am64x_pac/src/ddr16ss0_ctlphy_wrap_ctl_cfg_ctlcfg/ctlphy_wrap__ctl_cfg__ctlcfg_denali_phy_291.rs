#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_291` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy291Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_291` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy291Spec>;
#[doc = "Field `PHY_WDQLVL_PERIODIC_OBS_SELECT_1` reader - 7:0\\]
Select value to map specific information during or post periodic write data leveling for slice 1."]
pub type PhyWdqlvlPeriodicObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_PERIODIC_OBS_SELECT_1` writer - 7:0\\]
Select value to map specific information during or post periodic write data leveling for slice 1."]
pub type PhyWdqlvlPeriodicObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WDQLVL_DQ_SLV_DELTA_1` reader - 15:8\\]
The margin for DQ0-7's LE and TE dealy to make sure the DQ bits can work during DM training for slice 1."]
pub type PhyWdqlvlDqSlvDelta1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DQ_SLV_DELTA_1` writer - 15:8\\]
The margin for DQ0-7's LE and TE dealy to make sure the DQ bits can work during DM training for slice 1."]
pub type PhyWdqlvlDqSlvDelta1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WDQLVL_DM_DLY_STEP_1` reader - 19:16\\]
The slave delay line step for DM training for slice 1."]
pub type PhyWdqlvlDmDlyStep1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DM_DLY_STEP_1` writer - 19:16\\]
The slave delay line step for DM training for slice 1."]
pub type PhyWdqlvlDmDlyStep1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SC_PHY_WDQLVL_CLR_PREV_RESULTS_1` reader - 24:24\\]
Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 1. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyWdqlvlClrPrevResults1R = crate::BitReader;
#[doc = "Field `SC_PHY_WDQLVL_CLR_PREV_RESULTS_1` writer - 24:24\\]
Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 1. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyWdqlvlClrPrevResults1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Select value to map specific information during or post periodic write data leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_periodic_obs_select_1(&self) -> PhyWdqlvlPeriodicObsSelect1R {
        PhyWdqlvlPeriodicObsSelect1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The margin for DQ0-7's LE and TE dealy to make sure the DQ bits can work during DM training for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_dq_slv_delta_1(&self) -> PhyWdqlvlDqSlvDelta1R {
        PhyWdqlvlDqSlvDelta1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The slave delay line step for DM training for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_dm_dly_step_1(&self) -> PhyWdqlvlDmDlyStep1R {
        PhyWdqlvlDmDlyStep1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 1. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_wdqlvl_clr_prev_results_1(&self) -> ScPhyWdqlvlClrPrevResults1R {
        ScPhyWdqlvlClrPrevResults1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Select value to map specific information during or post periodic write data leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_periodic_obs_select_1(
        &mut self,
    ) -> PhyWdqlvlPeriodicObsSelect1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy291Spec> {
        PhyWdqlvlPeriodicObsSelect1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The margin for DQ0-7's LE and TE dealy to make sure the DQ bits can work during DM training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dq_slv_delta_1(
        &mut self,
    ) -> PhyWdqlvlDqSlvDelta1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy291Spec> {
        PhyWdqlvlDqSlvDelta1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The slave delay line step for DM training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dm_dly_step_1(
        &mut self,
    ) -> PhyWdqlvlDmDlyStep1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy291Spec> {
        PhyWdqlvlDmDlyStep1W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 1. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_wdqlvl_clr_prev_results_1(
        &mut self,
    ) -> ScPhyWdqlvlClrPrevResults1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy291Spec> {
        ScPhyWdqlvlClrPrevResults1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_291\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_291::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_291::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy291Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy291Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_291::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy291Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_291::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy291Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_291 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy291Spec {
    const RESET_VALUE: u32 = 0;
}
