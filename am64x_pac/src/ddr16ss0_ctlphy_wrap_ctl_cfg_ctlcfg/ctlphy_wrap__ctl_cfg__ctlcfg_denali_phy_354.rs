#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_354` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy354Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_354` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy354Spec>;
#[doc = "Field `PHY_WDQLVL_DLY_STEP_1` reader - 7:0\\]
DQ slave delay step size during write data leveling for slice 1."]
pub type PhyWdqlvlDlyStep1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DLY_STEP_1` writer - 7:0\\]
DQ slave delay step size during write data leveling for slice 1."]
pub type PhyWdqlvlDlyStep1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WDQLVL_QTR_DLY_STEP_1` reader - 11:8\\]
Defines the step granularity for the logic to use once an edge is found for slice 1. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
pub type PhyWdqlvlQtrDlyStep1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_QTR_DLY_STEP_1` writer - 11:8\\]
Defines the step granularity for the logic to use once an edge is found for slice 1. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
pub type PhyWdqlvlQtrDlyStep1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WDQLVL_DM_SEARCH_RANGE_1` reader - 24:16\\]
The dm slave delay search range for non-lpddr4 DM training for slice 1."]
pub type PhyWdqlvlDmSearchRange1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DM_SEARCH_RANGE_1` writer - 24:16\\]
The dm slave delay search range for non-lpddr4 DM training for slice 1."]
pub type PhyWdqlvlDmSearchRange1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DQ slave delay step size during write data leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_dly_step_1(&self) -> PhyWdqlvlDlyStep1R {
        PhyWdqlvlDlyStep1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the step granularity for the logic to use once an edge is found for slice 1. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
    #[inline(always)]
    pub fn phy_wdqlvl_qtr_dly_step_1(&self) -> PhyWdqlvlQtrDlyStep1R {
        PhyWdqlvlQtrDlyStep1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
The dm slave delay search range for non-lpddr4 DM training for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_dm_search_range_1(&self) -> PhyWdqlvlDmSearchRange1R {
        PhyWdqlvlDmSearchRange1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DQ slave delay step size during write data leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dly_step_1(
        &mut self,
    ) -> PhyWdqlvlDlyStep1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy354Spec> {
        PhyWdqlvlDlyStep1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the step granularity for the logic to use once an edge is found for slice 1. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_qtr_dly_step_1(
        &mut self,
    ) -> PhyWdqlvlQtrDlyStep1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy354Spec> {
        PhyWdqlvlQtrDlyStep1W::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
The dm slave delay search range for non-lpddr4 DM training for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dm_search_range_1(
        &mut self,
    ) -> PhyWdqlvlDmSearchRange1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy354Spec> {
        PhyWdqlvlDmSearchRange1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_354\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_354::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_354::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy354Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy354Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_354::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy354Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_354::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy354Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_354 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy354Spec {
    const RESET_VALUE: u32 = 0;
}
