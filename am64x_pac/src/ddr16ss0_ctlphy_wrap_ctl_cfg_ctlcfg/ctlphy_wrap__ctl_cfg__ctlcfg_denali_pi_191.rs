#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_191` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi191Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_191` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi191Spec>;
#[doc = "Field `PI_RDLVL_RXCAL_EN_F0` reader - 1:0\\]
Enable RX Offset calibration \\[PATTERN 14,15\\]
for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlRxcalEnF0R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_RXCAL_EN_F0` writer - 1:0\\]
Enable RX Offset calibration \\[PATTERN 14,15\\]
for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlRxcalEnF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_DFE_EN_F0` reader - 9:8\\]
Enable DFE \\[PATTERN 8,9\\]
for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlDfeEnF0R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_DFE_EN_F0` writer - 9:8\\]
Enable DFE \\[PATTERN 8,9\\]
for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlDfeEnF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_MULTI_EN_F0` reader - 17:16\\]
Enable Multi-pattern \\[from PI_RDLVL_PATTERN_START, total PI_RDLVL_PATTERN_NUM\\]
for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlMultiEnF0R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_MULTI_EN_F0` writer - 17:16\\]
Enable Multi-pattern \\[from PI_RDLVL_PATTERN_START, total PI_RDLVL_PATTERN_NUM\\]
for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlMultiEnF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_PAT0_EN_F1` reader - 25:24\\]
Enable PATTERN-0 for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlPat0EnF1R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_PAT0_EN_F1` writer - 25:24\\]
Enable PATTERN-0 for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlPat0EnF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Enable RX Offset calibration \\[PATTERN 14,15\\]
for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    pub fn pi_rdlvl_rxcal_en_f0(&self) -> PiRdlvlRxcalEnF0R {
        PiRdlvlRxcalEnF0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Enable DFE \\[PATTERN 8,9\\]
for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    pub fn pi_rdlvl_dfe_en_f0(&self) -> PiRdlvlDfeEnF0R {
        PiRdlvlDfeEnF0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable Multi-pattern \\[from PI_RDLVL_PATTERN_START, total PI_RDLVL_PATTERN_NUM\\]
for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    pub fn pi_rdlvl_multi_en_f0(&self) -> PiRdlvlMultiEnF0R {
        PiRdlvlMultiEnF0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Enable PATTERN-0 for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    pub fn pi_rdlvl_pat0_en_f1(&self) -> PiRdlvlPat0EnF1R {
        PiRdlvlPat0EnF1R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Enable RX Offset calibration \\[PATTERN 14,15\\]
for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_rxcal_en_f0(
        &mut self,
    ) -> PiRdlvlRxcalEnF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi191Spec> {
        PiRdlvlRxcalEnF0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Enable DFE \\[PATTERN 8,9\\]
for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_dfe_en_f0(
        &mut self,
    ) -> PiRdlvlDfeEnF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi191Spec> {
        PiRdlvlDfeEnF0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable Multi-pattern \\[from PI_RDLVL_PATTERN_START, total PI_RDLVL_PATTERN_NUM\\]
for read training for frequency set 0. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_multi_en_f0(
        &mut self,
    ) -> PiRdlvlMultiEnF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi191Spec> {
        PiRdlvlMultiEnF0W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Enable PATTERN-0 for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_pat0_en_f1(
        &mut self,
    ) -> PiRdlvlPat0EnF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi191Spec> {
        PiRdlvlPat0EnF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_191\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_191::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_191::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi191Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi191Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_191::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi191Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_191::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi191Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_191 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi191Spec {
    const RESET_VALUE: u32 = 0;
}
