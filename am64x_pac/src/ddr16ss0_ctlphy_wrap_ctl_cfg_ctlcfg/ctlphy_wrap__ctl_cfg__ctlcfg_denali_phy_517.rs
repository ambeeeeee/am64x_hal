#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_517` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy517Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_517` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy517Spec>;
#[doc = "Field `PHY_ADR_LPBK_CONTROL_0` reader - 6:0\\]
Loopback control bits for address slice 0."]
pub type PhyAdrLpbkControl0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_LPBK_CONTROL_0` writer - 6:0\\]
Loopback control bits for address slice 0."]
pub type PhyAdrLpbkControl0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_ADR_PRBS_PATTERN_START_0` reader - 14:8\\]
PRBS7 start pattern for address slice 0."]
pub type PhyAdrPrbsPatternStart0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_PRBS_PATTERN_START_0` writer - 14:8\\]
PRBS7 start pattern for address slice 0."]
pub type PhyAdrPrbsPatternStart0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_ADR_PRBS_PATTERN_MASK_0` reader - 20:16\\]
PRBS7 mask signal for address slice 0."]
pub type PhyAdrPrbsPatternMask0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_PRBS_PATTERN_MASK_0` writer - 20:16\\]
PRBS7 mask signal for address slice 0."]
pub type PhyAdrPrbsPatternMask0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_ADR_PWR_RDC_DISABLE_0` reader - 24:24\\]
Power reduction disable for address slice 0."]
pub type PhyAdrPwrRdcDisable0R = crate::BitReader;
#[doc = "Field `PHY_ADR_PWR_RDC_DISABLE_0` writer - 24:24\\]
Power reduction disable for address slice 0."]
pub type PhyAdrPwrRdcDisable0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Loopback control bits for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_lpbk_control_0(&self) -> PhyAdrLpbkControl0R {
        PhyAdrLpbkControl0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
PRBS7 start pattern for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_prbs_pattern_start_0(&self) -> PhyAdrPrbsPatternStart0R {
        PhyAdrPrbsPatternStart0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
PRBS7 mask signal for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_prbs_pattern_mask_0(&self) -> PhyAdrPrbsPatternMask0R {
        PhyAdrPrbsPatternMask0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Power reduction disable for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_pwr_rdc_disable_0(&self) -> PhyAdrPwrRdcDisable0R {
        PhyAdrPwrRdcDisable0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Loopback control bits for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_lpbk_control_0(
        &mut self,
    ) -> PhyAdrLpbkControl0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy517Spec> {
        PhyAdrLpbkControl0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
PRBS7 start pattern for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_prbs_pattern_start_0(
        &mut self,
    ) -> PhyAdrPrbsPatternStart0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy517Spec> {
        PhyAdrPrbsPatternStart0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
PRBS7 mask signal for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_prbs_pattern_mask_0(
        &mut self,
    ) -> PhyAdrPrbsPatternMask0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy517Spec> {
        PhyAdrPrbsPatternMask0W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Power reduction disable for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_pwr_rdc_disable_0(
        &mut self,
    ) -> PhyAdrPwrRdcDisable0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy517Spec> {
        PhyAdrPwrRdcDisable0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_517\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_517::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_517::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy517Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy517Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_517::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy517Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_517::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy517Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_517 to value 0x0100"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy517Spec {
    const RESET_VALUE: u32 = 0x0100;
}
