#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_88` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy88Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_88` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy88Spec>;
#[doc = "Field `PHY_DQ_OE_TIMING_0` reader - 7:0\\]
Start/end timing values for DQ/DM output enable signals for slice 0."]
pub type PhyDqOeTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQ_OE_TIMING_0` writer - 7:0\\]
Start/end timing values for DQ/DM output enable signals for slice 0."]
pub type PhyDqOeTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQ_TSEL_RD_TIMING_0` reader - 15:8\\]
Start/end timing values for DQ/DM read based termination enable and select signals for slice 0."]
pub type PhyDqTselRdTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_RD_TIMING_0` writer - 15:8\\]
Start/end timing values for DQ/DM read based termination enable and select signals for slice 0."]
pub type PhyDqTselRdTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQ_TSEL_WR_TIMING_0` reader - 23:16\\]
Start/end timing values for DQ/DM write based termination enable and select signals for slice 0."]
pub type PhyDqTselWrTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_WR_TIMING_0` writer - 23:16\\]
Start/end timing values for DQ/DM write based termination enable and select signals for slice 0."]
pub type PhyDqTselWrTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_OE_TIMING_0` reader - 31:24\\]
Start/end timing values for DQS output enable signals for slice 0."]
pub type PhyDqsOeTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQS_OE_TIMING_0` writer - 31:24\\]
Start/end timing values for DQS output enable signals for slice 0."]
pub type PhyDqsOeTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Start/end timing values for DQ/DM output enable signals for slice 0."]
    #[inline(always)]
    pub fn phy_dq_oe_timing_0(&self) -> PhyDqOeTiming0R {
        PhyDqOeTiming0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Start/end timing values for DQ/DM read based termination enable and select signals for slice 0."]
    #[inline(always)]
    pub fn phy_dq_tsel_rd_timing_0(&self) -> PhyDqTselRdTiming0R {
        PhyDqTselRdTiming0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Start/end timing values for DQ/DM write based termination enable and select signals for slice 0."]
    #[inline(always)]
    pub fn phy_dq_tsel_wr_timing_0(&self) -> PhyDqTselWrTiming0R {
        PhyDqTselWrTiming0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Start/end timing values for DQS output enable signals for slice 0."]
    #[inline(always)]
    pub fn phy_dqs_oe_timing_0(&self) -> PhyDqsOeTiming0R {
        PhyDqsOeTiming0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Start/end timing values for DQ/DM output enable signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_oe_timing_0(
        &mut self,
    ) -> PhyDqOeTiming0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy88Spec> {
        PhyDqOeTiming0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Start/end timing values for DQ/DM read based termination enable and select signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_rd_timing_0(
        &mut self,
    ) -> PhyDqTselRdTiming0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy88Spec> {
        PhyDqTselRdTiming0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Start/end timing values for DQ/DM write based termination enable and select signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_wr_timing_0(
        &mut self,
    ) -> PhyDqTselWrTiming0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy88Spec> {
        PhyDqTselWrTiming0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Start/end timing values for DQS output enable signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_oe_timing_0(
        &mut self,
    ) -> PhyDqsOeTiming0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy88Spec> {
        PhyDqsOeTiming0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_88\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_88::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_88::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy88Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy88Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_88::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy88Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_88::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy88Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_88 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy88Spec {
    const RESET_VALUE: u32 = 0;
}
