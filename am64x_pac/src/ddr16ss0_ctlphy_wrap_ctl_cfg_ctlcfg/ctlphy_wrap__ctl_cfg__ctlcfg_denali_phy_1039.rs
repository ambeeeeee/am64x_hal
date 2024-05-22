#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1039` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1039Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1039` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1039Spec>;
#[doc = "Field `PHY_ADR_CALVL_NUM_PATTERNS_2` reader - 1:0\\]
Number of patterns to use during CA training for address slice 2."]
pub type PhyAdrCalvlNumPatterns2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_NUM_PATTERNS_2` writer - 1:0\\]
Number of patterns to use during CA training for address slice 2."]
pub type PhyAdrCalvlNumPatterns2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_ADR_CALVL_RESP_WAIT_CNT_2` reader - 11:8\\]
Number of samples to wait before sampling response during CA training for address slice 2."]
pub type PhyAdrCalvlRespWaitCnt2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_RESP_WAIT_CNT_2` writer - 11:8\\]
Number of samples to wait before sampling response during CA training for address slice 2."]
pub type PhyAdrCalvlRespWaitCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_ADR_CALVL_PERIODIC_START_OFFSET_2` reader - 24:16\\]
Relative offset to start periodic CALVL from previous result"]
pub type PhyAdrCalvlPeriodicStartOffset2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CALVL_PERIODIC_START_OFFSET_2` writer - 24:16\\]
Relative offset to start periodic CALVL from previous result"]
pub type PhyAdrCalvlPeriodicStartOffset2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Number of patterns to use during CA training for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_num_patterns_2(&self) -> PhyAdrCalvlNumPatterns2R {
        PhyAdrCalvlNumPatterns2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of samples to wait before sampling response during CA training for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_resp_wait_cnt_2(&self) -> PhyAdrCalvlRespWaitCnt2R {
        PhyAdrCalvlRespWaitCnt2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Relative offset to start periodic CALVL from previous result"]
    #[inline(always)]
    pub fn phy_adr_calvl_periodic_start_offset_2(&self) -> PhyAdrCalvlPeriodicStartOffset2R {
        PhyAdrCalvlPeriodicStartOffset2R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Number of patterns to use during CA training for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_num_patterns_2(
        &mut self,
    ) -> PhyAdrCalvlNumPatterns2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1039Spec> {
        PhyAdrCalvlNumPatterns2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of samples to wait before sampling response during CA training for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_resp_wait_cnt_2(
        &mut self,
    ) -> PhyAdrCalvlRespWaitCnt2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1039Spec> {
        PhyAdrCalvlRespWaitCnt2W::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Relative offset to start periodic CALVL from previous result"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_periodic_start_offset_2(
        &mut self,
    ) -> PhyAdrCalvlPeriodicStartOffset2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1039Spec> {
        PhyAdrCalvlPeriodicStartOffset2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1039\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1039::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1039::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1039Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1039Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1039::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1039Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1039::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1039Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1039 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1039Spec {
    const RESET_VALUE: u32 = 0;
}
