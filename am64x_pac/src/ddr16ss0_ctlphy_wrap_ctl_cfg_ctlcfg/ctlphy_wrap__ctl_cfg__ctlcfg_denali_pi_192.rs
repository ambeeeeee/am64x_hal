#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_192` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi192Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_192` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi192Spec>;
#[doc = "Field `PI_RDLVL_RXCAL_EN_F1` reader - 1:0\\]
Enable RX Offset calibration \\[PATTERN 14,15\\]
for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlRxcalEnF1R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_RXCAL_EN_F1` writer - 1:0\\]
Enable RX Offset calibration \\[PATTERN 14,15\\]
for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlRxcalEnF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_DFE_EN_F1` reader - 9:8\\]
Enable DFE \\[PATTERN 8,9\\]
for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlDfeEnF1R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_DFE_EN_F1` writer - 9:8\\]
Enable DFE \\[PATTERN 8,9\\]
for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlDfeEnF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_MULTI_EN_F1` reader - 17:16\\]
Enable Multi-pattern \\[from PI_RDLVL_PATTERN_START, total PI_RDLVL_PATTERN_NUM\\]
for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlMultiEnF1R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_MULTI_EN_F1` writer - 17:16\\]
Enable Multi-pattern \\[from PI_RDLVL_PATTERN_START, total PI_RDLVL_PATTERN_NUM\\]
for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlMultiEnF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_PAT0_EN_F2` reader - 25:24\\]
Enable PATTERN-0 for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlPat0EnF2R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_PAT0_EN_F2` writer - 25:24\\]
Enable PATTERN-0 for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlPat0EnF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Enable RX Offset calibration \\[PATTERN 14,15\\]
for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    pub fn pi_rdlvl_rxcal_en_f1(&self) -> PiRdlvlRxcalEnF1R {
        PiRdlvlRxcalEnF1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Enable DFE \\[PATTERN 8,9\\]
for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    pub fn pi_rdlvl_dfe_en_f1(&self) -> PiRdlvlDfeEnF1R {
        PiRdlvlDfeEnF1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable Multi-pattern \\[from PI_RDLVL_PATTERN_START, total PI_RDLVL_PATTERN_NUM\\]
for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    pub fn pi_rdlvl_multi_en_f1(&self) -> PiRdlvlMultiEnF1R {
        PiRdlvlMultiEnF1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Enable PATTERN-0 for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    pub fn pi_rdlvl_pat0_en_f2(&self) -> PiRdlvlPat0EnF2R {
        PiRdlvlPat0EnF2R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Enable RX Offset calibration \\[PATTERN 14,15\\]
for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_rxcal_en_f1(
        &mut self,
    ) -> PiRdlvlRxcalEnF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi192Spec> {
        PiRdlvlRxcalEnF1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Enable DFE \\[PATTERN 8,9\\]
for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_dfe_en_f1(
        &mut self,
    ) -> PiRdlvlDfeEnF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi192Spec> {
        PiRdlvlDfeEnF1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable Multi-pattern \\[from PI_RDLVL_PATTERN_START, total PI_RDLVL_PATTERN_NUM\\]
for read training for frequency set 1. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_multi_en_f1(
        &mut self,
    ) -> PiRdlvlMultiEnF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi192Spec> {
        PiRdlvlMultiEnF1W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Enable PATTERN-0 for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_pat0_en_f2(
        &mut self,
    ) -> PiRdlvlPat0EnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi192Spec> {
        PiRdlvlPat0EnF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_192\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_192::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_192::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi192Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi192Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_192::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi192Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_192::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi192Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_192 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi192Spec {
    const RESET_VALUE: u32 = 0;
}
