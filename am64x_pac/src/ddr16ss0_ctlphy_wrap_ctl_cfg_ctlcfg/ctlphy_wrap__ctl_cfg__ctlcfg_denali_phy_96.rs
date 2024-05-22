#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_96` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy96Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_96` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy96Spec>;
#[doc = "Field `PHY_WRLVL_RESP_WAIT_CNT_0` reader - 5:0\\]
Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 0."]
pub type PhyWrlvlRespWaitCnt0R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_RESP_WAIT_CNT_0` writer - 5:0\\]
Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 0."]
pub type PhyWrlvlRespWaitCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_GTLVL_DLY_STEP_0` reader - 11:8\\]
DQS slave delay step size during gate training for slice 0."]
pub type PhyGtlvlDlyStep0R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_DLY_STEP_0` writer - 11:8\\]
DQS slave delay step size during gate training for slice 0."]
pub type PhyGtlvlDlyStep0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_GTLVL_RESP_WAIT_CNT_0` reader - 20:16\\]
Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 0. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlRespWaitCnt0R = crate::FieldReader;
#[doc = "Field `PHY_GTLVL_RESP_WAIT_CNT_0` writer - 20:16\\]
Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 0. The valid range is 0x0 to 0xB."]
pub type PhyGtlvlRespWaitCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_resp_wait_cnt_0(&self) -> PhyWrlvlRespWaitCnt0R {
        PhyWrlvlRespWaitCnt0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
DQS slave delay step size during gate training for slice 0."]
    #[inline(always)]
    pub fn phy_gtlvl_dly_step_0(&self) -> PhyGtlvlDlyStep0R {
        PhyGtlvlDlyStep0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 0. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    pub fn phy_gtlvl_resp_wait_cnt_0(&self) -> PhyGtlvlRespWaitCnt0R {
        PhyGtlvlRespWaitCnt0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Number of cycles to wait between dfi_wrlvl_strobe and the sampling of the DQs during write leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_resp_wait_cnt_0(
        &mut self,
    ) -> PhyWrlvlRespWaitCnt0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy96Spec> {
        PhyWrlvlRespWaitCnt0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
DQS slave delay step size during gate training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_dly_step_0(
        &mut self,
    ) -> PhyGtlvlDlyStep0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy96Spec> {
        PhyGtlvlDlyStep0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Number of cycles + 4 to wait between dfi_rddata_en and the sampling of the DQS during gate training for slice 0. The valid range is 0x0 to 0xB."]
    #[inline(always)]
    #[must_use]
    pub fn phy_gtlvl_resp_wait_cnt_0(
        &mut self,
    ) -> PhyGtlvlRespWaitCnt0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy96Spec> {
        PhyGtlvlRespWaitCnt0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_96::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_96::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy96Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy96Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_96::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy96Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_96::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy96Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_96 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy96Spec {
    const RESET_VALUE: u32 = 0;
}
