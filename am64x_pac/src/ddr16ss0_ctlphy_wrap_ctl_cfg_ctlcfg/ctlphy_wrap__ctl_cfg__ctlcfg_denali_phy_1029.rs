#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1029` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1029Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1029` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1029Spec>;
#[doc = "Field `PHY_ADR_LPBK_CONTROL_2` reader - 6:0\\]
Loopback control bits for address slice 2."]
pub type PhyAdrLpbkControl2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_LPBK_CONTROL_2` writer - 6:0\\]
Loopback control bits for address slice 2."]
pub type PhyAdrLpbkControl2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_ADR_PRBS_PATTERN_START_2` reader - 14:8\\]
PRBS7 start pattern for address slice 2."]
pub type PhyAdrPrbsPatternStart2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_PRBS_PATTERN_START_2` writer - 14:8\\]
PRBS7 start pattern for address slice 2."]
pub type PhyAdrPrbsPatternStart2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_ADR_PRBS_PATTERN_MASK_2` reader - 20:16\\]
PRBS7 mask signal for address slice 2."]
pub type PhyAdrPrbsPatternMask2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_PRBS_PATTERN_MASK_2` writer - 20:16\\]
PRBS7 mask signal for address slice 2."]
pub type PhyAdrPrbsPatternMask2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_ADR_PWR_RDC_DISABLE_2` reader - 24:24\\]
Power reduction disable for address slice 2."]
pub type PhyAdrPwrRdcDisable2R = crate::BitReader;
#[doc = "Field `PHY_ADR_PWR_RDC_DISABLE_2` writer - 24:24\\]
Power reduction disable for address slice 2."]
pub type PhyAdrPwrRdcDisable2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Loopback control bits for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_lpbk_control_2(&self) -> PhyAdrLpbkControl2R {
        PhyAdrLpbkControl2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
PRBS7 start pattern for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_prbs_pattern_start_2(&self) -> PhyAdrPrbsPatternStart2R {
        PhyAdrPrbsPatternStart2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
PRBS7 mask signal for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_prbs_pattern_mask_2(&self) -> PhyAdrPrbsPatternMask2R {
        PhyAdrPrbsPatternMask2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Power reduction disable for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_pwr_rdc_disable_2(&self) -> PhyAdrPwrRdcDisable2R {
        PhyAdrPwrRdcDisable2R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Loopback control bits for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_lpbk_control_2(
        &mut self,
    ) -> PhyAdrLpbkControl2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1029Spec> {
        PhyAdrLpbkControl2W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
PRBS7 start pattern for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_prbs_pattern_start_2(
        &mut self,
    ) -> PhyAdrPrbsPatternStart2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1029Spec> {
        PhyAdrPrbsPatternStart2W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
PRBS7 mask signal for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_prbs_pattern_mask_2(
        &mut self,
    ) -> PhyAdrPrbsPatternMask2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1029Spec> {
        PhyAdrPrbsPatternMask2W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Power reduction disable for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_pwr_rdc_disable_2(
        &mut self,
    ) -> PhyAdrPwrRdcDisable2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1029Spec> {
        PhyAdrPwrRdcDisable2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1029\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1029::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1029::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1029Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1029Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1029::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1029Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1029::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1029Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1029 to value 0x0100"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1029Spec {
    const RESET_VALUE: u32 = 0x0100;
}
