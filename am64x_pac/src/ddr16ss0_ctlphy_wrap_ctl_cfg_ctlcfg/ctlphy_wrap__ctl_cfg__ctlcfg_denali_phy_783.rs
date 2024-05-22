#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_783` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy783Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_783` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy783Spec>;
#[doc = "Field `PHY_ADR_CALVL_NUM_PATTERNS_1` reader - 1:0\\]
Number of patterns to use during CA training for address slice 1."]
pub type PhyAdrCalvlNumPatterns1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_NUM_PATTERNS_1` writer - 1:0\\]
Number of patterns to use during CA training for address slice 1."]
pub type PhyAdrCalvlNumPatterns1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_CALVL_RESP_WAIT_CNT_1` reader - 11:8\\]
Number of samples to wait before sampling response during CA training for address slice 1."]
pub type PhyAdrCalvlRespWaitCnt1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_RESP_WAIT_CNT_1` writer - 11:8\\]
Number of samples to wait before sampling response during CA training for address slice 1."]
pub type PhyAdrCalvlRespWaitCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_ADR_CALVL_PERIODIC_START_OFFSET_1` reader - 24:16\\]
Relative offset to start periodic CALVL from previous result"]
pub type PhyAdrCalvlPeriodicStartOffset1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CALVL_PERIODIC_START_OFFSET_1` writer - 24:16\\]
Relative offset to start periodic CALVL from previous result"]
pub type PhyAdrCalvlPeriodicStartOffset1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Number of patterns to use during CA training for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_num_patterns_1(&self) -> PhyAdrCalvlNumPatterns1R {
        PhyAdrCalvlNumPatterns1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of samples to wait before sampling response during CA training for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_resp_wait_cnt_1(&self) -> PhyAdrCalvlRespWaitCnt1R {
        PhyAdrCalvlRespWaitCnt1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Relative offset to start periodic CALVL from previous result"]
    #[inline(always)]
    pub fn phy_adr_calvl_periodic_start_offset_1(&self) -> PhyAdrCalvlPeriodicStartOffset1R {
        PhyAdrCalvlPeriodicStartOffset1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Number of patterns to use during CA training for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_num_patterns_1(
        &mut self,
    ) -> PhyAdrCalvlNumPatterns1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy783Spec> {
        PhyAdrCalvlNumPatterns1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of samples to wait before sampling response during CA training for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_resp_wait_cnt_1(
        &mut self,
    ) -> PhyAdrCalvlRespWaitCnt1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy783Spec> {
        PhyAdrCalvlRespWaitCnt1W::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Relative offset to start periodic CALVL from previous result"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_periodic_start_offset_1(
        &mut self,
    ) -> PhyAdrCalvlPeriodicStartOffset1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy783Spec> {
        PhyAdrCalvlPeriodicStartOffset1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_783\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_783::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_783::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy783Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy783Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_783::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy783Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_783::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy783Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_783 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy783Spec {
    const RESET_VALUE: u32 = 0;
}
