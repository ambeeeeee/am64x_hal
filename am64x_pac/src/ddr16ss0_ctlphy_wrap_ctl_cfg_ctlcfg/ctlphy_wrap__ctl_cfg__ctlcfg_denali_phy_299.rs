#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_299` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy299Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_299` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy299Spec>;
#[doc = "Field `PHY_NTP_PERIOD_THRESHOLD_MIN_1` reader - 9:0\\]
Minimum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 1."]
pub type PhyNtpPeriodThresholdMin1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_NTP_PERIOD_THRESHOLD_MIN_1` writer - 9:0\\]
Minimum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 1."]
pub type PhyNtpPeriodThresholdMin1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_NTP_PERIOD_THRESHOLD_MAX_1` reader - 25:16\\]
Maximum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 1."]
pub type PhyNtpPeriodThresholdMax1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_NTP_PERIOD_THRESHOLD_MAX_1` writer - 25:16\\]
Maximum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 1."]
pub type PhyNtpPeriodThresholdMax1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Minimum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 1."]
    #[inline(always)]
    pub fn phy_ntp_period_threshold_min_1(&self) -> PhyNtpPeriodThresholdMin1R {
        PhyNtpPeriodThresholdMin1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Maximum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 1."]
    #[inline(always)]
    pub fn phy_ntp_period_threshold_max_1(&self) -> PhyNtpPeriodThresholdMax1R {
        PhyNtpPeriodThresholdMax1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Minimum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_period_threshold_min_1(
        &mut self,
    ) -> PhyNtpPeriodThresholdMin1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy299Spec> {
        PhyNtpPeriodThresholdMin1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Maximum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_period_threshold_max_1(
        &mut self,
    ) -> PhyNtpPeriodThresholdMax1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy299Spec> {
        PhyNtpPeriodThresholdMax1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_299\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_299::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_299::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy299Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy299Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_299::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy299Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_299::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy299Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_299 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy299Spec {
    const RESET_VALUE: u32 = 0;
}
