#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_352` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy352Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_352` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy352Spec>;
#[doc = "Field `PHY_WRLVL_RESP_WAIT_CNT_1` reader - 5:0\\]
Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 1."]
pub type PhyWrlvlRespWaitCnt1R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_RESP_WAIT_CNT_1` writer - 5:0\\]
Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 1."]
pub type PhyWrlvlRespWaitCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_GTLVL_DLY_STEP_1` reader - 11:8\\]
DQS slave delay step size during gate training for slice 1."]
pub type PhyGtlvlDlyStep1R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_DLY_STEP_1` writer - 11:8\\]
DQS slave delay step size during gate training for slice 1."]
pub type PhyGtlvlDlyStep1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_GTLVL_RESP_WAIT_CNT_1` reader - 20:16\\]
Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 1. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlRespWaitCnt1R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_RESP_WAIT_CNT_1` writer - 20:16\\]
Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 1. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlRespWaitCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_resp_wait_cnt_1(&self) -> PhyWrlvlRespWaitCnt1R {
        PhyWrlvlRespWaitCnt1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
DQS slave delay step size during gate training for slice 1."]
    #[inline(always)]
    pub fn phy_gtlvl_dly_step_1(&self) -> PhyGtlvlDlyStep1R {
        PhyGtlvlDlyStep1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 1. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    pub fn phy_gtlvl_resp_wait_cnt_1(&self) -> PhyGtlvlRespWaitCnt1R {
        PhyGtlvlRespWaitCnt1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_resp_wait_cnt_1(
        &mut self,
    ) -> PhyWrlvlRespWaitCnt1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy352Spec> {
        PhyWrlvlRespWaitCnt1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
DQS slave delay step size during gate training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_dly_step_1(
        &mut self,
    ) -> PhyGtlvlDlyStep1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy352Spec> {
        PhyGtlvlDlyStep1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 1. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_resp_wait_cnt_1(
        &mut self,
    ) -> PhyGtlvlRespWaitCnt1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy352Spec> {
        PhyGtlvlRespWaitCnt1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_352\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_352::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_352::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy352Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy352Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_352::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy352Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_352::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy352Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_352 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy352Spec {
    const RESET_VALUE: u32 = 0;
}
