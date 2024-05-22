#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_321` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy321Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_321` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy321Spec>;
#[doc = "Field `PHY_DDL_TRACK_UPD_THRESHOLD_1` reader - 7:0\\]
Specify threshold value for PHY init update tracking for slice 1."]
pub type PhyDdlTrackUpdThreshold1R = crate::FieldReader;
#[doc = "Field `PHY_DDL_TRACK_UPD_THRESHOLD_1` writer - 7:0\\]
Specify threshold value for PHY init update tracking for slice 1."]
pub type PhyDdlTrackUpdThreshold1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_LP4_WDQS_OE_EXTEND_1` reader - 8:8\\]
LPDDR4 write preamble extension enable for slice 1."]
pub type PhyLp4WdqsOeExtend1R = crate::BitReader;
#[doc = "Field `PHY_LP4_WDQS_OE_EXTEND_1` writer - 8:8\\]
LPDDR4 write preamble extension enable for slice 1."]
pub type PhyLp4WdqsOeExtend1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RX_CAL_DQ0_1` reader - 24:16\\]
RX Calibration codes for DQ0 for slice 1. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
pub type PhyRxCalDq0_1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ0_1` writer - 24:16\\]
RX Calibration codes for DQ0 for slice 1. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
pub type PhyRxCalDq0_1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Specify threshold value for PHY init update tracking for slice 1."]
    #[inline(always)]
    pub fn phy_ddl_track_upd_threshold_1(&self) -> PhyDdlTrackUpdThreshold1R {
        PhyDdlTrackUpdThreshold1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
LPDDR4 write preamble extension enable for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_wdqs_oe_extend_1(&self) -> PhyLp4WdqsOeExtend1R {
        PhyLp4WdqsOeExtend1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:24 - 24:16\\]
RX Calibration codes for DQ0 for slice 1. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq0_1(&self) -> PhyRxCalDq0_1R {
        PhyRxCalDq0_1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Specify threshold value for PHY init update tracking for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ddl_track_upd_threshold_1(
        &mut self,
    ) -> PhyDdlTrackUpdThreshold1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy321Spec> {
        PhyDdlTrackUpdThreshold1W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
LPDDR4 write preamble extension enable for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_wdqs_oe_extend_1(
        &mut self,
    ) -> PhyLp4WdqsOeExtend1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy321Spec> {
        PhyLp4WdqsOeExtend1W::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
RX Calibration codes for DQ0 for slice 1. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq0_1(&mut self) -> PhyRxCalDq0_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy321Spec> {
        PhyRxCalDq0_1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_321\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_321::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_321::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy321Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy321Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_321::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy321Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_321::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy321Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_321 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy321Spec {
    const RESET_VALUE: u32 = 0;
}
