#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_172` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi172Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_172` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi172Spec>;
#[doc = "Field `PI_CASLAT_LIN_F0` reader - 6:0\\]
Sets latency from read command sent to data received from/to controller for frequency set 0. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller."]
pub type PiCaslatLinF0R = crate::FieldReader;
#[doc = "Field `PI_CASLAT_LIN_F0` writer - 6:0\\]
Sets latency from read command sent to data received from/to controller for frequency set 0. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller."]
pub type PiCaslatLinF0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_WRLAT_F1` reader - 14:8\\]
DRAM WRLAT value in cycles for frequency set 1."]
pub type PiWrlatF1R = crate::FieldReader;
#[doc = "Field `PI_WRLAT_F1` writer - 14:8\\]
DRAM WRLAT value in cycles for frequency set 1."]
pub type PiWrlatF1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_ADDITIVE_LAT_F1` reader - 21:16\\]
DRAM additive latency value in cycles for frequency set 1."]
pub type PiAdditiveLatF1R = crate::FieldReader;
#[doc = "Field `PI_ADDITIVE_LAT_F1` writer - 21:16\\]
DRAM additive latency value in cycles for frequency set 1."]
pub type PiAdditiveLatF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_CA_PARITY_LAT_F1` reader - 27:24\\]
DRAM CA parity latency value in cycles for frequency set 1."]
pub type PiCaParityLatF1R = crate::FieldReader;
#[doc = "Field `PI_CA_PARITY_LAT_F1` writer - 27:24\\]
DRAM CA parity latency value in cycles for frequency set 1."]
pub type PiCaParityLatF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Sets latency from read command sent to data received from/to controller for frequency set 0. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller."]
    #[inline(always)]
    pub fn pi_caslat_lin_f0(&self) -> PiCaslatLinF0R {
        PiCaslatLinF0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
DRAM WRLAT value in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_wrlat_f1(&self) -> PiWrlatF1R {
        PiWrlatF1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM additive latency value in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_additive_lat_f1(&self) -> PiAdditiveLatF1R {
        PiAdditiveLatF1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
DRAM CA parity latency value in cycles for frequency set 1."]
    #[inline(always)]
    pub fn pi_ca_parity_lat_f1(&self) -> PiCaParityLatF1R {
        PiCaParityLatF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Sets latency from read command sent to data received from/to controller for frequency set 0. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller."]
    #[inline(always)]
    #[must_use]
    pub fn pi_caslat_lin_f0(&mut self) -> PiCaslatLinF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi172Spec> {
        PiCaslatLinF0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
DRAM WRLAT value in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlat_f1(&mut self) -> PiWrlatF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi172Spec> {
        PiWrlatF1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
DRAM additive latency value in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_additive_lat_f1(
        &mut self,
    ) -> PiAdditiveLatF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi172Spec> {
        PiAdditiveLatF1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
DRAM CA parity latency value in cycles for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_ca_parity_lat_f1(
        &mut self,
    ) -> PiCaParityLatF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi172Spec> {
        PiCaParityLatF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_172\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_172::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_172::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi172Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi172Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_172::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi172Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_172::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi172Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_172 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi172Spec {
    const RESET_VALUE: u32 = 0;
}
