#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_193` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi193Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_193` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi193Spec>;
#[doc = "Field `PI_RDLVL_RXCAL_EN_F2` reader - 1:0\\]
Enable RX Offset calibration \\[PATTERN 14,15\\]
for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlRxcalEnF2R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_RXCAL_EN_F2` writer - 1:0\\]
Enable RX Offset calibration \\[PATTERN 14,15\\]
for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlRxcalEnF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_DFE_EN_F2` reader - 9:8\\]
Enable DFE \\[PATTERN 8,9\\]
for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlDfeEnF2R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_DFE_EN_F2` writer - 9:8\\]
Enable DFE \\[PATTERN 8,9\\]
for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlDfeEnF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_MULTI_EN_F2` reader - 17:16\\]
Enable Multi-pattern \\[from PI_RDLVL_PATTERN_START, total PI_RDLVL_PATTERN_NUM\\]
for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlMultiEnF2R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_MULTI_EN_F2` writer - 17:16\\]
Enable Multi-pattern \\[from PI_RDLVL_PATTERN_START, total PI_RDLVL_PATTERN_NUM\\]
for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
pub type PiRdlvlMultiEnF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLAT_ADJ_F0` reader - 31:24\\]
Adjusts the relative timing between DFI read commands and the dfi_rddata_en signal for frequency set 0."]
pub type PiRdlatAdjF0R = crate::FieldReader;
#[doc = "Field `PI_RDLAT_ADJ_F0` writer - 31:24\\]
Adjusts the relative timing between DFI read commands and the dfi_rddata_en signal for frequency set 0."]
pub type PiRdlatAdjF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Enable RX Offset calibration \\[PATTERN 14,15\\]
for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    pub fn pi_rdlvl_rxcal_en_f2(&self) -> PiRdlvlRxcalEnF2R {
        PiRdlvlRxcalEnF2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Enable DFE \\[PATTERN 8,9\\]
for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    pub fn pi_rdlvl_dfe_en_f2(&self) -> PiRdlvlDfeEnF2R {
        PiRdlvlDfeEnF2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable Multi-pattern \\[from PI_RDLVL_PATTERN_START, total PI_RDLVL_PATTERN_NUM\\]
for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    pub fn pi_rdlvl_multi_en_f2(&self) -> PiRdlvlMultiEnF2R {
        PiRdlvlMultiEnF2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Adjusts the relative timing between DFI read commands and the dfi_rddata_en signal for frequency set 0."]
    #[inline(always)]
    pub fn pi_rdlat_adj_f0(&self) -> PiRdlatAdjF0R {
        PiRdlatAdjF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Enable RX Offset calibration \\[PATTERN 14,15\\]
for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_rxcal_en_f2(
        &mut self,
    ) -> PiRdlvlRxcalEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi193Spec> {
        PiRdlvlRxcalEnF2W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Enable DFE \\[PATTERN 8,9\\]
for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_dfe_en_f2(
        &mut self,
    ) -> PiRdlvlDfeEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi193Spec> {
        PiRdlvlDfeEnF2W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable Multi-pattern \\[from PI_RDLVL_PATTERN_START, total PI_RDLVL_PATTERN_NUM\\]
for read training for frequency set 2. bit1 for normal; bit0 for initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_multi_en_f2(
        &mut self,
    ) -> PiRdlvlMultiEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi193Spec> {
        PiRdlvlMultiEnF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Adjusts the relative timing between DFI read commands and the dfi_rddata_en signal for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlat_adj_f0(&mut self) -> PiRdlatAdjF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi193Spec> {
        PiRdlatAdjF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_193\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_193::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_193::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi193Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi193Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_193::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi193Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_193::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi193Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_193 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi193Spec {
    const RESET_VALUE: u32 = 0;
}
