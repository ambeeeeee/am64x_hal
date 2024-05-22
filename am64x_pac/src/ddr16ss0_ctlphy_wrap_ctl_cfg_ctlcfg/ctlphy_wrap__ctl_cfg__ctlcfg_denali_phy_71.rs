#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_71` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy71Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_71` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy71Spec>;
#[doc = "Field `PHY_RX_CAL_DQS_0` reader - 8:0\\]
RX Calibration codes for DQS for slice 0. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
pub type PhyRxCalDqs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQS_0` writer - 8:0\\]
RX Calibration codes for DQS for slice 0. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
pub type PhyRxCalDqs0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PHY_RX_CAL_FDBK_0` reader - 24:16\\]
RX Calibration codes for FDBK for slice 0. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
pub type PhyRxCalFdbk0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_FDBK_0` writer - 24:16\\]
RX Calibration codes for FDBK for slice 0. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
pub type PhyRxCalFdbk0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
RX Calibration codes for DQS for slice 0. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dqs_0(&self) -> PhyRxCalDqs0R {
        PhyRxCalDqs0R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - 24:16\\]
RX Calibration codes for FDBK for slice 0. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_fdbk_0(&self) -> PhyRxCalFdbk0R {
        PhyRxCalFdbk0R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
RX Calibration codes for DQS for slice 0. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dqs_0(&mut self) -> PhyRxCalDqs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy71Spec> {
        PhyRxCalDqs0W::new(self, 0)
    }
    #[doc = "Bits 16:24 - 24:16\\]
RX Calibration codes for FDBK for slice 0. Bits \\[5:0\\]
contain rx_cal_code_down. Bits \\[11:6\\]
contain rx_cal_code_up. Bits \\[17:12\\]
contain rx_cal_code2_down. Bits \\[23:18\\]
contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_fdbk_0(&mut self) -> PhyRxCalFdbk0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy71Spec> {
        PhyRxCalFdbk0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_71\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_71::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_71::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy71Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy71Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_71::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy71Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_71::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy71Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_71 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy71Spec {
    const RESET_VALUE: u32 = 0;
}
