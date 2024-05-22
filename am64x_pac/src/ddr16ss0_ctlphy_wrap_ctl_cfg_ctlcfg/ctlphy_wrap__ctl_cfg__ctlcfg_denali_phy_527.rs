#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_527` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy527Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_527` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy527Spec>;
#[doc = "Field `PHY_ADR_CALVL_NUM_PATTERNS_0` reader - 1:0\\]
Number of patterns to use during CA training for address slice 0."]
pub type PhyAdrCalvlNumPatterns0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_NUM_PATTERNS_0` writer - 1:0\\]
Number of patterns to use during CA training for address slice 0."]
pub type PhyAdrCalvlNumPatterns0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_CALVL_RESP_WAIT_CNT_0` reader - 11:8\\]
Number of samples to wait before sampling response during CA training for address slice 0."]
pub type PhyAdrCalvlRespWaitCnt0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_RESP_WAIT_CNT_0` writer - 11:8\\]
Number of samples to wait before sampling response during CA training for address slice 0."]
pub type PhyAdrCalvlRespWaitCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_ADR_CALVL_PERIODIC_START_OFFSET_0` reader - 24:16\\]
Relative offset to start periodic CALVL from previous result"]
pub type PhyAdrCalvlPeriodicStartOffset0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CALVL_PERIODIC_START_OFFSET_0` writer - 24:16\\]
Relative offset to start periodic CALVL from previous result"]
pub type PhyAdrCalvlPeriodicStartOffset0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Number of patterns to use during CA training for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_num_patterns_0(&self) -> PhyAdrCalvlNumPatterns0R {
        PhyAdrCalvlNumPatterns0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of samples to wait before sampling response during CA training for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_resp_wait_cnt_0(&self) -> PhyAdrCalvlRespWaitCnt0R {
        PhyAdrCalvlRespWaitCnt0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Relative offset to start periodic CALVL from previous result"]
    #[inline(always)]
    pub fn phy_adr_calvl_periodic_start_offset_0(&self) -> PhyAdrCalvlPeriodicStartOffset0R {
        PhyAdrCalvlPeriodicStartOffset0R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Number of patterns to use during CA training for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_num_patterns_0(
        &mut self,
    ) -> PhyAdrCalvlNumPatterns0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy527Spec> {
        PhyAdrCalvlNumPatterns0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of samples to wait before sampling response during CA training for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_resp_wait_cnt_0(
        &mut self,
    ) -> PhyAdrCalvlRespWaitCnt0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy527Spec> {
        PhyAdrCalvlRespWaitCnt0W::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Relative offset to start periodic CALVL from previous result"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_periodic_start_offset_0(
        &mut self,
    ) -> PhyAdrCalvlPeriodicStartOffset0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy527Spec> {
        PhyAdrCalvlPeriodicStartOffset0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_527\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_527::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_527::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy527Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy527Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_527::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy527Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_527::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy527Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_527 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy527Spec {
    const RESET_VALUE: u32 = 0;
}
