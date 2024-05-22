#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_65` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy65Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_65` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy65Spec>;
#[doc = "Field `PHY_DDL_TRACK_UPD_THRESHOLD_0` reader - 7:0\\]
Specify threshold value for PHY init update tracking for slice 0."]
pub type PhyDdlTrackUpdThreshold0R = crate::FieldReader;
#[doc = "Field `PHY_DDL_TRACK_UPD_THRESHOLD_0` writer - 7:0\\]
Specify threshold value for PHY init update tracking for slice 0."]
pub type PhyDdlTrackUpdThreshold0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_LP4_WDQS_OE_EXTEND_0` reader - 8:8\\]
LPDDR4 write preamble extension enable for slice 0."]
pub type PhyLp4WdqsOeExtend0R = crate::BitReader;
#[doc = "Field `PHY_LP4_WDQS_OE_EXTEND_0` writer - 8:8\\]
LPDDR4 write preamble extension enable for slice 0."]
pub type PhyLp4WdqsOeExtend0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RX_CAL_DQ0_0` reader - 24:16\\]
RX Calibration codes for DQ0 for slice 0. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
pub type PhyRxCalDq0_0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ0_0` writer - 24:16\\]
RX Calibration codes for DQ0 for slice 0. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
pub type PhyRxCalDq0_0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Specify threshold value for PHY init update tracking for slice 0."]
    #[inline(always)]
    pub fn phy_ddl_track_upd_threshold_0(&self) -> PhyDdlTrackUpdThreshold0R {
        PhyDdlTrackUpdThreshold0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
LPDDR4 write preamble extension enable for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_wdqs_oe_extend_0(&self) -> PhyLp4WdqsOeExtend0R {
        PhyLp4WdqsOeExtend0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:24 - 24:16\\]
RX Calibration codes for DQ0 for slice 0. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq0_0(&self) -> PhyRxCalDq0_0R {
        PhyRxCalDq0_0R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Specify threshold value for PHY init update tracking for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ddl_track_upd_threshold_0(
        &mut self,
    ) -> PhyDdlTrackUpdThreshold0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy65Spec> {
        PhyDdlTrackUpdThreshold0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
LPDDR4 write preamble extension enable for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_wdqs_oe_extend_0(
        &mut self,
    ) -> PhyLp4WdqsOeExtend0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy65Spec> {
        PhyLp4WdqsOeExtend0W::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
RX Calibration codes for DQ0 for slice 0. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq0_0(&mut self) -> PhyRxCalDq0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy65Spec> {
        PhyRxCalDq0_0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_65::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_65::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy65Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy65Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_65::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy65Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_65::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy65Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_65 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy65Spec {
    const RESET_VALUE: u32 = 0;
}
