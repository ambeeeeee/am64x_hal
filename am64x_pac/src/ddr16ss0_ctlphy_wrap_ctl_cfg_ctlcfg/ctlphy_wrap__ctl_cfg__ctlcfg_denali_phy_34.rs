#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_34` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy34Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_34` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy34Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_JUMP_OFFSET_0` reader - 10:0\\]
Defines the slave delay jump value when the TE window is found and begin to serch TE window for slice 0."]
pub type PhyWdqlvlDqdmSlvDlyJumpOffset0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_JUMP_OFFSET_0` writer - 10:0\\]
Defines the slave delay jump value when the TE window is found and begin to serch TE window for slice 0."]
pub type PhyWdqlvlDqdmSlvDlyJumpOffset0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_WDQLVL_UPDT_WAIT_CNT_0` reader - 19:16\\]
Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 0."]
pub type PhyWdqlvlUpdtWaitCnt0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_UPDT_WAIT_CNT_0` writer - 19:16\\]
Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 0."]
pub type PhyWdqlvlUpdtWaitCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WDQLVL_DQDM_OBS_SELECT_0` reader - 27:24\\]
Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 0."]
pub type PhyWdqlvlDqdmObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DQDM_OBS_SELECT_0` writer - 27:24\\]
Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 0."]
pub type PhyWdqlvlDqdmObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Defines the slave delay jump value when the TE window is found and begin to serch TE window for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_slv_dly_jump_offset_0(&self) -> PhyWdqlvlDqdmSlvDlyJumpOffset0R {
        PhyWdqlvlDqdmSlvDlyJumpOffset0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_updt_wait_cnt_0(&self) -> PhyWdqlvlUpdtWaitCnt0R {
        PhyWdqlvlUpdtWaitCnt0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_obs_select_0(&self) -> PhyWdqlvlDqdmObsSelect0R {
        PhyWdqlvlDqdmObsSelect0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Defines the slave delay jump value when the TE window is found and begin to serch TE window for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_slv_dly_jump_offset_0(
        &mut self,
    ) -> PhyWdqlvlDqdmSlvDlyJumpOffset0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy34Spec> {
        PhyWdqlvlDqdmSlvDlyJumpOffset0W::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_updt_wait_cnt_0(
        &mut self,
    ) -> PhyWdqlvlUpdtWaitCnt0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy34Spec> {
        PhyWdqlvlUpdtWaitCnt0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_obs_select_0(
        &mut self,
    ) -> PhyWdqlvlDqdmObsSelect0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy34Spec> {
        PhyWdqlvlDqdmObsSelect0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy34Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_34::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy34Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_34::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_34 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy34Spec {
    const RESET_VALUE: u32 = 0;
}
