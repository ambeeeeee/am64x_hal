#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_287` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy287Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_287` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy287Spec>;
#[doc = "Field `PHY_RDLVL_CAPTURE_CNT_1` reader - 5:0\\]
Number of samples to take at each DQS slave delay setting during read leveling for slice 1."]
pub type PhyRdlvlCaptureCnt1R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_CAPTURE_CNT_1` writer - 5:0\\]
Number of samples to take at each DQS slave delay setting during read leveling for slice 1."]
pub type PhyRdlvlCaptureCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_RDLVL_UPDT_WAIT_CNT_1` reader - 11:8\\]
Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 1."]
pub type PhyRdlvlUpdtWaitCnt1R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_UPDT_WAIT_CNT_1` writer - 11:8\\]
Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 1."]
pub type PhyRdlvlUpdtWaitCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_RDLVL_OP_MODE_1` reader - 17:16\\]
Read leveling algorithm select for slice 1. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
pub type PhyRdlvlOpMode1R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_OP_MODE_1` writer - 17:16\\]
Read leveling algorithm select for slice 1. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
pub type PhyRdlvlOpMode1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_OBS_SELECT_1` reader - 28:24\\]
Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 1."]
pub type PhyRdlvlRddqsDqObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_RDDQS_DQ_OBS_SELECT_1` writer - 28:24\\]
Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 1."]
pub type PhyRdlvlRddqsDqObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Number of samples to take at each DQS slave delay setting during read leveling for slice 1."]
    #[inline(always)]
    pub fn phy_rdlvl_capture_cnt_1(&self) -> PhyRdlvlCaptureCnt1R {
        PhyRdlvlCaptureCnt1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 1."]
    #[inline(always)]
    pub fn phy_rdlvl_updt_wait_cnt_1(&self) -> PhyRdlvlUpdtWaitCnt1R {
        PhyRdlvlUpdtWaitCnt1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Read leveling algorithm select for slice 1. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
    #[inline(always)]
    pub fn phy_rdlvl_op_mode_1(&self) -> PhyRdlvlOpMode1R {
        PhyRdlvlOpMode1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 1."]
    #[inline(always)]
    pub fn phy_rdlvl_rddqs_dq_obs_select_1(&self) -> PhyRdlvlRddqsDqObsSelect1R {
        PhyRdlvlRddqsDqObsSelect1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Number of samples to take at each DQS slave delay setting during read leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_capture_cnt_1(
        &mut self,
    ) -> PhyRdlvlCaptureCnt1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy287Spec> {
        PhyRdlvlCaptureCnt1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of cycles to wait after changing DQS slave delay setting during read leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_updt_wait_cnt_1(
        &mut self,
    ) -> PhyRdlvlUpdtWaitCnt1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy287Spec> {
        PhyRdlvlUpdtWaitCnt1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Read leveling algorithm select for slice 1. Clear to 0 to move linearly from left to right. Set to 1 to start inside the window, move left and then move right."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_op_mode_1(
        &mut self,
    ) -> PhyRdlvlOpMode1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy287Spec> {
        PhyRdlvlOpMode1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during read leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_rddqs_dq_obs_select_1(
        &mut self,
    ) -> PhyRdlvlRddqsDqObsSelect1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy287Spec> {
        PhyRdlvlRddqsDqObsSelect1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_287\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_287::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_287::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy287Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy287Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_287::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy287Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_287::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy287Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_287 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy287Spec {
    const RESET_VALUE: u32 = 0;
}
