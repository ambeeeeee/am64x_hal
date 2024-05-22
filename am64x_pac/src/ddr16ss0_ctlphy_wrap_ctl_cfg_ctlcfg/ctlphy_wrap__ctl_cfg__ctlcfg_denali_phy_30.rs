#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_30` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy30Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_30` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy30Spec>;
#[doc = "Field `PHY_WRLVL_UPDT_WAIT_CNT_0` reader - 3:0\\]
Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 0."]
pub type PhyWrlvlUpdtWaitCnt0R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_UPDT_WAIT_CNT_0` writer - 3:0\\]
Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 0."]
pub type PhyWrlvlUpdtWaitCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_DQ_MASK_0` reader - 15:8\\]
For ECC slice, should set this register to do DQ bit mask for slice 0."]
pub type PhyDqMask0R = crate::FieldReader;
#[doc = "Field `PHY_DQ_MASK_0` writer - 15:8\\]
For ECC slice, should set this register to do DQ bit mask for slice 0."]
pub type PhyDqMask0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_GTLVL_CAPTURE_CNT_0` reader - 21:16\\]
Number of samples to take at each DQS slave delay setting during gate training for slice 0."]
pub type PhyGtlvlCaptureCnt0R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_CAPTURE_CNT_0` writer - 21:16\\]
Number of samples to take at each DQS slave delay setting during gate training for slice 0."]
pub type PhyGtlvlCaptureCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_GTLVL_UPDT_WAIT_CNT_0` reader - 27:24\\]
Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 0. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlUpdtWaitCnt0R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_UPDT_WAIT_CNT_0` writer - 27:24\\]
Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 0. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlUpdtWaitCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_updt_wait_cnt_0(&self) -> PhyWrlvlUpdtWaitCnt0R {
        PhyWrlvlUpdtWaitCnt0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
For ECC slice, should set this register to do DQ bit mask for slice 0."]
    #[inline(always)]
    pub fn phy_dq_mask_0(&self) -> PhyDqMask0R {
        PhyDqMask0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Number of samples to take at each DQS slave delay setting during gate training for slice 0."]
    #[inline(always)]
    pub fn phy_gtlvl_capture_cnt_0(&self) -> PhyGtlvlCaptureCnt0R {
        PhyGtlvlCaptureCnt0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 0. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    pub fn phy_gtlvl_updt_wait_cnt_0(&self) -> PhyGtlvlUpdtWaitCnt0R {
        PhyGtlvlUpdtWaitCnt0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_updt_wait_cnt_0(
        &mut self,
    ) -> PhyWrlvlUpdtWaitCnt0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy30Spec> {
        PhyWrlvlUpdtWaitCnt0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
For ECC slice, should set this register to do DQ bit mask for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_mask_0(&mut self) -> PhyDqMask0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy30Spec> {
        PhyDqMask0W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Number of samples to take at each DQS slave delay setting during gate training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_capture_cnt_0(
        &mut self,
    ) -> PhyGtlvlCaptureCnt0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy30Spec> {
        PhyGtlvlCaptureCnt0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Number of cycles + 4 to wait after changing DQS slave delay setting during gate training for slice 0. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_updt_wait_cnt_0(
        &mut self,
    ) -> PhyGtlvlUpdtWaitCnt0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy30Spec> {
        PhyGtlvlUpdtWaitCnt0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy30Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_30::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy30Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_30::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_30 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy30Spec {
    const RESET_VALUE: u32 = 0;
}
