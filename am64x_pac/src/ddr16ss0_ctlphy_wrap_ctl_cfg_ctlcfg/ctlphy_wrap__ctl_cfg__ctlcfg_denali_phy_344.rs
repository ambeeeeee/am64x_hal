#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_344` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy344Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_344` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy344Spec>;
#[doc = "Field `PHY_DQ_OE_TIMING_1` reader - 7:0\\]
Start/end timing values for DQ/DM output enable signals for slice 1."]
pub type PhyDqOeTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQ_OE_TIMING_1` writer - 7:0\\]
Start/end timing values for DQ/DM output enable signals for slice 1."]
pub type PhyDqOeTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQ_TSEL_RD_TIMING_1` reader - 15:8\\]
Start/end timing values for DQ/DM read based termination enable and select signals for slice 1."]
pub type PhyDqTselRdTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_RD_TIMING_1` writer - 15:8\\]
Start/end timing values for DQ/DM read based termination enable and select signals for slice 1."]
pub type PhyDqTselRdTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQ_TSEL_WR_TIMING_1` reader - 23:16\\]
Start/end timing values for DQ/DM write based termination enable and select signals for slice 1."]
pub type PhyDqTselWrTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQ_TSEL_WR_TIMING_1` writer - 23:16\\]
Start/end timing values for DQ/DM write based termination enable and select signals for slice 1."]
pub type PhyDqTselWrTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_OE_TIMING_1` reader - 31:24\\]
Start/end timing values for DQS output enable signals for slice 1."]
pub type PhyDqsOeTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQS_OE_TIMING_1` writer - 31:24\\]
Start/end timing values for DQS output enable signals for slice 1."]
pub type PhyDqsOeTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Start/end timing values for DQ/DM output enable signals for slice 1."]
    #[inline(always)]
    pub fn phy_dq_oe_timing_1(&self) -> PhyDqOeTiming1R {
        PhyDqOeTiming1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Start/end timing values for DQ/DM read based termination enable and select signals for slice 1."]
    #[inline(always)]
    pub fn phy_dq_tsel_rd_timing_1(&self) -> PhyDqTselRdTiming1R {
        PhyDqTselRdTiming1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Start/end timing values for DQ/DM write based termination enable and select signals for slice 1."]
    #[inline(always)]
    pub fn phy_dq_tsel_wr_timing_1(&self) -> PhyDqTselWrTiming1R {
        PhyDqTselWrTiming1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Start/end timing values for DQS output enable signals for slice 1."]
    #[inline(always)]
    pub fn phy_dqs_oe_timing_1(&self) -> PhyDqsOeTiming1R {
        PhyDqsOeTiming1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Start/end timing values for DQ/DM output enable signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_oe_timing_1(
        &mut self,
    ) -> PhyDqOeTiming1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy344Spec> {
        PhyDqOeTiming1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Start/end timing values for DQ/DM read based termination enable and select signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_rd_timing_1(
        &mut self,
    ) -> PhyDqTselRdTiming1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy344Spec> {
        PhyDqTselRdTiming1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Start/end timing values for DQ/DM write based termination enable and select signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_tsel_wr_timing_1(
        &mut self,
    ) -> PhyDqTselWrTiming1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy344Spec> {
        PhyDqTselWrTiming1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Start/end timing values for DQS output enable signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_oe_timing_1(
        &mut self,
    ) -> PhyDqsOeTiming1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy344Spec> {
        PhyDqsOeTiming1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_344\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_344::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_344::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy344Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy344Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_344::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy344Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_344::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy344Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_344 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy344Spec {
    const RESET_VALUE: u32 = 0;
}
