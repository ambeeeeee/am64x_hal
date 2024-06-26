#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_35` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy35Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_35` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy35Spec>;
#[doc = "Field `PHY_WDQLVL_PERIODIC_OBS_SELECT_0` reader - 7:0\\]
Select value to map specific information during or post periodic write data leveling for slice 0."]
pub type PhyWdqlvlPeriodicObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_PERIODIC_OBS_SELECT_0` writer - 7:0\\]
Select value to map specific information during or post periodic write data leveling for slice 0."]
pub type PhyWdqlvlPeriodicObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WDQLVL_DQ_SLV_DELTA_0` reader - 15:8\\]
The margin for DQ0-7's LE and TE dealy to make sure the DQ bits can work during DM training for slice 0."]
pub type PhyWdqlvlDqSlvDelta0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DQ_SLV_DELTA_0` writer - 15:8\\]
The margin for DQ0-7's LE and TE dealy to make sure the DQ bits can work during DM training for slice 0."]
pub type PhyWdqlvlDqSlvDelta0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WDQLVL_DM_DLY_STEP_0` reader - 19:16\\]
The slave delay line step for DM training for slice 0."]
pub type PhyWdqlvlDmDlyStep0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DM_DLY_STEP_0` writer - 19:16\\]
The slave delay line step for DM training for slice 0."]
pub type PhyWdqlvlDmDlyStep0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SC_PHY_WDQLVL_CLR_PREV_RESULTS_0` reader - 24:24\\]
Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 0. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyWdqlvlClrPrevResults0R = crate::BitReader;
#[doc = "Field `SC_PHY_WDQLVL_CLR_PREV_RESULTS_0` writer - 24:24\\]
Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 0. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyWdqlvlClrPrevResults0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Select value to map specific information during or post periodic write data leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_periodic_obs_select_0(&self) -> PhyWdqlvlPeriodicObsSelect0R {
        PhyWdqlvlPeriodicObsSelect0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The margin for DQ0-7's LE and TE dealy to make sure the DQ bits can work during DM training for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_dq_slv_delta_0(&self) -> PhyWdqlvlDqSlvDelta0R {
        PhyWdqlvlDqSlvDelta0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The slave delay line step for DM training for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_dm_dly_step_0(&self) -> PhyWdqlvlDmDlyStep0R {
        PhyWdqlvlDmDlyStep0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_wdqlvl_clr_prev_results_0(&self) -> ScPhyWdqlvlClrPrevResults0R {
        ScPhyWdqlvlClrPrevResults0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Select value to map specific information during or post periodic write data leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_periodic_obs_select_0(
        &mut self,
    ) -> PhyWdqlvlPeriodicObsSelect0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy35Spec> {
        PhyWdqlvlPeriodicObsSelect0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
The margin for DQ0-7's LE and TE dealy to make sure the DQ bits can work during DM training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dq_slv_delta_0(
        &mut self,
    ) -> PhyWdqlvlDqSlvDelta0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy35Spec> {
        PhyWdqlvlDqSlvDelta0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The slave delay line step for DM training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dm_dly_step_0(
        &mut self,
    ) -> PhyWdqlvlDmDlyStep0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy35Spec> {
        PhyWdqlvlDmDlyStep0W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_wdqlvl_clr_prev_results_0(
        &mut self,
    ) -> ScPhyWdqlvlClrPrevResults0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy35Spec> {
        ScPhyWdqlvlClrPrevResults0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy35Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_35::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy35Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_35::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy35Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_35 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy35Spec {
    const RESET_VALUE: u32 = 0;
}
