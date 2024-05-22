#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_345` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy345Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_345` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy345Spec>;
#[doc = "Field `PHY_IO_PAD_DELAY_TIMING_1` reader - 3:0\\]
Feedback pad's OPAD and IPAD delay timing for slice 1."]
pub type PhyIoPadDelayTiming1R = crate::FieldReader;
#[doc = "Field `PHY_IO_PAD_DELAY_TIMING_1` writer - 3:0\\]
Feedback pad's OPAD and IPAD delay timing for slice 1."]
pub type PhyIoPadDelayTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_DQS_TSEL_RD_TIMING_1` reader - 15:8\\]
Start/end timing values for DQS read based termination enable and select signals for slice 1."]
pub type PhyDqsTselRdTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQS_TSEL_RD_TIMING_1` writer - 15:8\\]
Start/end timing values for DQS read based termination enable and select signals for slice 1."]
pub type PhyDqsTselRdTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_OE_RD_TIMING_1` reader - 23:16\\]
Start/end timing values for DQS read based OE extension for slice 1."]
pub type PhyDqsOeRdTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQS_OE_RD_TIMING_1` writer - 23:16\\]
Start/end timing values for DQS read based OE extension for slice 1."]
pub type PhyDqsOeRdTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_DQS_TSEL_WR_TIMING_1` reader - 31:24\\]
Start/end timing values for DQS write based termination enable and select signals for slice 1."]
pub type PhyDqsTselWrTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQS_TSEL_WR_TIMING_1` writer - 31:24\\]
Start/end timing values for DQS write based termination enable and select signals for slice 1."]
pub type PhyDqsTselWrTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Feedback pad's OPAD and IPAD delay timing for slice 1."]
    #[inline(always)]
    pub fn phy_io_pad_delay_timing_1(&self) -> PhyIoPadDelayTiming1R {
        PhyIoPadDelayTiming1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Start/end timing values for DQS read based termination enable and select signals for slice 1."]
    #[inline(always)]
    pub fn phy_dqs_tsel_rd_timing_1(&self) -> PhyDqsTselRdTiming1R {
        PhyDqsTselRdTiming1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Start/end timing values for DQS read based OE extension for slice 1."]
    #[inline(always)]
    pub fn phy_dqs_oe_rd_timing_1(&self) -> PhyDqsOeRdTiming1R {
        PhyDqsOeRdTiming1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Start/end timing values for DQS write based termination enable and select signals for slice 1."]
    #[inline(always)]
    pub fn phy_dqs_tsel_wr_timing_1(&self) -> PhyDqsTselWrTiming1R {
        PhyDqsTselWrTiming1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Feedback pad's OPAD and IPAD delay timing for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_io_pad_delay_timing_1(
        &mut self,
    ) -> PhyIoPadDelayTiming1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy345Spec> {
        PhyIoPadDelayTiming1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Start/end timing values for DQS read based termination enable and select signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_rd_timing_1(
        &mut self,
    ) -> PhyDqsTselRdTiming1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy345Spec> {
        PhyDqsTselRdTiming1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Start/end timing values for DQS read based OE extension for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_oe_rd_timing_1(
        &mut self,
    ) -> PhyDqsOeRdTiming1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy345Spec> {
        PhyDqsOeRdTiming1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Start/end timing values for DQS write based termination enable and select signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_tsel_wr_timing_1(
        &mut self,
    ) -> PhyDqsTselWrTiming1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy345Spec> {
        PhyDqsTselWrTiming1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_345\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_345::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_345::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy345Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy345Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_345::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy345Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_345::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy345Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_345 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy345Spec {
    const RESET_VALUE: u32 = 0;
}
