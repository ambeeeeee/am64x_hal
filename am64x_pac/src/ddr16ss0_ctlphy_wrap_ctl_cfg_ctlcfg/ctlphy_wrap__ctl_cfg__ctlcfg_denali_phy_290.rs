#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_290` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy290Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_290` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy290Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_JUMP_OFFSET_1` reader - 10:0\\]
Defines the slave delay jump value when the TE window is found and begin to serch TE window for slice 1."]
pub type PhyWdqlvlDqdmSlvDlyJumpOffset1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_JUMP_OFFSET_1` writer - 10:0\\]
Defines the slave delay jump value when the TE window is found and begin to serch TE window for slice 1."]
pub type PhyWdqlvlDqdmSlvDlyJumpOffset1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_WDQLVL_UPDT_WAIT_CNT_1` reader - 19:16\\]
Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 1."]
pub type PhyWdqlvlUpdtWaitCnt1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_UPDT_WAIT_CNT_1` writer - 19:16\\]
Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 1."]
pub type PhyWdqlvlUpdtWaitCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WDQLVL_DQDM_OBS_SELECT_1` reader - 27:24\\]
Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 1."]
pub type PhyWdqlvlDqdmObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DQDM_OBS_SELECT_1` writer - 27:24\\]
Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 1."]
pub type PhyWdqlvlDqdmObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Defines the slave delay jump value when the TE window is found and begin to serch TE window for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_slv_dly_jump_offset_1(&self) -> PhyWdqlvlDqdmSlvDlyJumpOffset1R {
        PhyWdqlvlDqdmSlvDlyJumpOffset1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_updt_wait_cnt_1(&self) -> PhyWdqlvlUpdtWaitCnt1R {
        PhyWdqlvlUpdtWaitCnt1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_obs_select_1(&self) -> PhyWdqlvlDqdmObsSelect1R {
        PhyWdqlvlDqdmObsSelect1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Defines the slave delay jump value when the TE window is found and begin to serch TE window for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_slv_dly_jump_offset_1(
        &mut self,
    ) -> PhyWdqlvlDqdmSlvDlyJumpOffset1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy290Spec> {
        PhyWdqlvlDqdmSlvDlyJumpOffset1W::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_updt_wait_cnt_1(
        &mut self,
    ) -> PhyWdqlvlUpdtWaitCnt1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy290Spec> {
        PhyWdqlvlUpdtWaitCnt1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_obs_select_1(
        &mut self,
    ) -> PhyWdqlvlDqdmObsSelect1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy290Spec> {
        PhyWdqlvlDqdmObsSelect1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_290\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_290::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_290::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy290Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy290Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_290::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy290Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_290::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy290Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_290 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy290Spec {
    const RESET_VALUE: u32 = 0;
}
