#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_43` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy43Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_43` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy43Spec>;
#[doc = "Field `PHY_NTP_PERIOD_THRESHOLD_MIN_0` reader - 9:0\\]
Minimum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 0."]
pub type PhyNtpPeriodThresholdMin0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_NTP_PERIOD_THRESHOLD_MIN_0` writer - 9:0\\]
Minimum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 0."]
pub type PhyNtpPeriodThresholdMin0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_NTP_PERIOD_THRESHOLD_MAX_0` reader - 25:16\\]
Maximum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 0."]
pub type PhyNtpPeriodThresholdMax0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_NTP_PERIOD_THRESHOLD_MAX_0` writer - 25:16\\]
Maximum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 0."]
pub type PhyNtpPeriodThresholdMax0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Minimum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 0."]
    #[inline(always)]
    pub fn phy_ntp_period_threshold_min_0(&self) -> PhyNtpPeriodThresholdMin0R {
        PhyNtpPeriodThresholdMin0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Maximum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 0."]
    #[inline(always)]
    pub fn phy_ntp_period_threshold_max_0(&self) -> PhyNtpPeriodThresholdMax0R {
        PhyNtpPeriodThresholdMax0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Minimum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_period_threshold_min_0(
        &mut self,
    ) -> PhyNtpPeriodThresholdMin0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy43Spec> {
        PhyNtpPeriodThresholdMin0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Maximum Threshold that phy_clk_wrdqs_slave_delay could cross boundary, to set period threshold/early threshold after No-Topology training is completed for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ntp_period_threshold_max_0(
        &mut self,
    ) -> PhyNtpPeriodThresholdMax0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy43Spec> {
        PhyNtpPeriodThresholdMax0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_43::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_43::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy43Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_43::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy43Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_43::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy43Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_43 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy43Spec {
    const RESET_VALUE: u32 = 0;
}
