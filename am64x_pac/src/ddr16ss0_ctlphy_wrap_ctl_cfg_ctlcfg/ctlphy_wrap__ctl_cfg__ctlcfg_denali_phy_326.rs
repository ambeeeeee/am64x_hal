#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_326` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy326Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_326` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy326Spec>;
#[doc = "Field `PHY_RX_CAL_DM_1` reader - 17:0\\]
RX Calibration codes for DM for slice 1. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
pub type PhyRxCalDm1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_RX_CAL_DM_1` writer - 17:0\\]
RX Calibration codes for DM for slice 1. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
pub type PhyRxCalDm1W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - 17:0\\]
RX Calibration codes for DM for slice 1. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dm_1(&self) -> PhyRxCalDm1R {
        PhyRxCalDm1R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - 17:0\\]
RX Calibration codes for DM for slice 1. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dm_1(&mut self) -> PhyRxCalDm1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy326Spec> {
        PhyRxCalDm1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_326\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_326::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_326::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy326Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy326Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_326::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy326Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_326::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy326Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_326 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy326Spec {
    const RESET_VALUE: u32 = 0;
}
