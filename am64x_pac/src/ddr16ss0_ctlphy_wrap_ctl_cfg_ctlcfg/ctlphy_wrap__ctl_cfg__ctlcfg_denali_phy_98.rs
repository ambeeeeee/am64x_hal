#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_98` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy98Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_98` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy98Spec>;
#[doc = "Field `PHY_WDQLVL_DLY_STEP_0` reader - 7:0\\]
DQ slave delay step size during write data leveling for slice 0."]
pub type PhyWdqlvlDlyStep0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DLY_STEP_0` writer - 7:0\\]
DQ slave delay step size during write data leveling for slice 0."]
pub type PhyWdqlvlDlyStep0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_WDQLVL_QTR_DLY_STEP_0` reader - 11:8\\]
Defines the step granularity for the logic to use once an edge is found for slice 0. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
pub type PhyWdqlvlQtrDlyStep0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_QTR_DLY_STEP_0` writer - 11:8\\]
Defines the step granularity for the logic to use once an edge is found for slice 0. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
pub type PhyWdqlvlQtrDlyStep0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WDQLVL_DM_SEARCH_RANGE_0` reader - 24:16\\]
The dm slave delay search range for non-lpddr4 DM training for slice 0."]
pub type PhyWdqlvlDmSearchRange0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DM_SEARCH_RANGE_0` writer - 24:16\\]
The dm slave delay search range for non-lpddr4 DM training for slice 0."]
pub type PhyWdqlvlDmSearchRange0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DQ slave delay step size during write data leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_dly_step_0(&self) -> PhyWdqlvlDlyStep0R {
        PhyWdqlvlDlyStep0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the step granularity for the logic to use once an edge is found for slice 0. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
    #[inline(always)]
    pub fn phy_wdqlvl_qtr_dly_step_0(&self) -> PhyWdqlvlQtrDlyStep0R {
        PhyWdqlvlQtrDlyStep0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
The dm slave delay search range for non-lpddr4 DM training for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_dm_search_range_0(&self) -> PhyWdqlvlDmSearchRange0R {
        PhyWdqlvlDmSearchRange0R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DQ slave delay step size during write data leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dly_step_0(
        &mut self,
    ) -> PhyWdqlvlDlyStep0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy98Spec> {
        PhyWdqlvlDlyStep0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the step granularity for the logic to use once an edge is found for slice 0. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_qtr_dly_step_0(
        &mut self,
    ) -> PhyWdqlvlQtrDlyStep0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy98Spec> {
        PhyWdqlvlQtrDlyStep0W::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
The dm slave delay search range for non-lpddr4 DM training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dm_search_range_0(
        &mut self,
    ) -> PhyWdqlvlDmSearchRange0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy98Spec> {
        PhyWdqlvlDmSearchRange0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_98\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_98::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_98::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy98Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_98::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy98Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_98::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy98Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_98 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy98Spec {
    const RESET_VALUE: u32 = 0;
}
