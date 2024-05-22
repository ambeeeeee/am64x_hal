#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_286` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy286Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_286` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy286Spec>;
#[doc = "Field `PHY_WRLVL_UPDT_WAIT_CNT_1` reader - 3:0\\]
Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 1."]
pub type PhyWrlvlUpdtWaitCnt1R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_UPDT_WAIT_CNT_1` writer - 3:0\\]
Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 1."]
pub type PhyWrlvlUpdtWaitCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_DQ_MASK_1` reader - 15:8\\]
For ECC slice, should set this register to do DQ bit mask for slice 1."]
pub type PhyDqMask1R = crate::FieldReader;
#[doc = "Field `PHY_DQ_MASK_1` writer - 15:8\\]
For ECC slice, should set this register to do DQ bit mask for slice 1."]
pub type PhyDqMask1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_GTLVL_CAPTURE_CNT_1` reader - 21:16\\]
Number of samples to take at each DQS slave delay setting during gate training for slice 1."]
pub type PhyGtlvlCaptureCnt1R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_CAPTURE_CNT_1` writer - 21:16\\]
Number of samples to take at each DQS slave delay setting during gate training for slice 1."]
pub type PhyGtlvlCaptureCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_GTLVL_UPDT_WAIT_CNT_1` reader - 27:24\\]
Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 1. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlUpdtWaitCnt1R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_UPDT_WAIT_CNT_1` writer - 27:24\\]
Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 1. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlUpdtWaitCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_updt_wait_cnt_1(&self) -> PhyWrlvlUpdtWaitCnt1R {
        PhyWrlvlUpdtWaitCnt1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
For ECC slice, should set this register to do DQ bit mask for slice 1."]
    #[inline(always)]
    pub fn phy_dq_mask_1(&self) -> PhyDqMask1R {
        PhyDqMask1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Number of samples to take at each DQS slave delay setting during gate training for slice 1."]
    #[inline(always)]
    pub fn phy_gtlvl_capture_cnt_1(&self) -> PhyGtlvlCaptureCnt1R {
        PhyGtlvlCaptureCnt1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 1. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    pub fn phy_gtlvl_updt_wait_cnt_1(&self) -> PhyGtlvlUpdtWaitCnt1R {
        PhyGtlvlUpdtWaitCnt1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_updt_wait_cnt_1(
        &mut self,
    ) -> PhyWrlvlUpdtWaitCnt1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy286Spec> {
        PhyWrlvlUpdtWaitCnt1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
For ECC slice, should set this register to do DQ bit mask for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_mask_1(&mut self) -> PhyDqMask1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy286Spec> {
        PhyDqMask1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Number of samples to take at each DQS slave delay setting during gate training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_capture_cnt_1(
        &mut self,
    ) -> PhyGtlvlCaptureCnt1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy286Spec> {
        PhyGtlvlCaptureCnt1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 1. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_updt_wait_cnt_1(
        &mut self,
    ) -> PhyGtlvlUpdtWaitCnt1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy286Spec> {
        PhyGtlvlUpdtWaitCnt1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_286\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_286::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_286::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy286Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy286Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_286::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy286Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_286::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy286Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_286 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy286Spec {
    const RESET_VALUE: u32 = 0;
}
