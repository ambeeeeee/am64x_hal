#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1354` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1354Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1354` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1354Spec>;
#[doc = "Field `PHY_AC_PRBS_PATTERN_START` reader - 6:0\\]
PRBS7 start pattern for address/control slice."]
pub type PhyAcPrbsPatternStartR = crate::FieldReader;
#[doc = "Field `PHY_AC_PRBS_PATTERN_START` writer - 6:0\\]
PRBS7 start pattern for address/control slice."]
pub type PhyAcPrbsPatternStartW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_AC_PRBS_PATTERN_MASK` reader - 11:8\\]
PRBS7 mask signal for address/control slice."]
pub type PhyAcPrbsPatternMaskR = crate::FieldReader;
#[doc = "Field `PHY_AC_PRBS_PATTERN_MASK` writer - 11:8\\]
PRBS7 mask signal for address/control slice."]
pub type PhyAcPrbsPatternMaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
PRBS7 start pattern for address/control slice."]
    #[inline(always)]
    pub fn phy_ac_prbs_pattern_start(&self) -> PhyAcPrbsPatternStartR {
        PhyAcPrbsPatternStartR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
PRBS7 mask signal for address/control slice."]
    #[inline(always)]
    pub fn phy_ac_prbs_pattern_mask(&self) -> PhyAcPrbsPatternMaskR {
        PhyAcPrbsPatternMaskR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
PRBS7 start pattern for address/control slice."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_prbs_pattern_start(
        &mut self,
    ) -> PhyAcPrbsPatternStartW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1354Spec> {
        PhyAcPrbsPatternStartW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
PRBS7 mask signal for address/control slice."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_prbs_pattern_mask(
        &mut self,
    ) -> PhyAcPrbsPatternMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1354Spec> {
        PhyAcPrbsPatternMaskW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1354\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1354::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1354::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1354Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1354Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1354::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1354Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1354::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1354Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1354 to value 0x01"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1354Spec {
    const RESET_VALUE: u32 = 0x01;
}
