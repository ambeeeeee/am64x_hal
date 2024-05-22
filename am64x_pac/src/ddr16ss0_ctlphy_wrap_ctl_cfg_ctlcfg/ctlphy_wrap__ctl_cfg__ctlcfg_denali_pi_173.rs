#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_173` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi173Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_173` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi173Spec>;
#[doc = "Field `PI_TPARITY_ERROR_CMD_INHIBIT_F1` reader - 7:0\\]
Defines the window after the PI receives a parity error during which DRAM commands will not execute for frequency set 1."]
pub type PiTparityErrorCmdInhibitF1R = crate::FieldReader;
#[doc = "Field `PI_TPARITY_ERROR_CMD_INHIBIT_F1` writer - 7:0\\]
Defines the window after the PI receives a parity error during which DRAM commands will not execute for frequency set 1."]
pub type PiTparityErrorCmdInhibitF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_CASLAT_LIN_F1` reader - 14:8\\]
Sets latency from read command sent to data received from/to controller for frequency set 1. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller."]
pub type PiCaslatLinF1R = crate::FieldReader;
#[doc = "Field `PI_CASLAT_LIN_F1` writer - 14:8\\]
Sets latency from read command sent to data received from/to controller for frequency set 1. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller."]
pub type PiCaslatLinF1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_WRLAT_F2` reader - 22:16\\]
DRAM WRLAT value in cycles for frequency set 2."]
pub type PiWrlatF2R = crate::FieldReader;
#[doc = "Field `PI_WRLAT_F2` writer - 22:16\\]
DRAM WRLAT value in cycles for frequency set 2."]
pub type PiWrlatF2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_ADDITIVE_LAT_F2` reader - 29:24\\]
DRAM additive latency value in cycles for frequency set 2."]
pub type PiAdditiveLatF2R = crate::FieldReader;
#[doc = "Field `PI_ADDITIVE_LAT_F2` writer - 29:24\\]
DRAM additive latency value in cycles for frequency set 2."]
pub type PiAdditiveLatF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the window after the PI receives a parity error during which DRAM commands will not execute for frequency set 1."]
    #[inline(always)]
    pub fn pi_tparity_error_cmd_inhibit_f1(&self) -> PiTparityErrorCmdInhibitF1R {
        PiTparityErrorCmdInhibitF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Sets latency from read command sent to data received from/to controller for frequency set 1. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller."]
    #[inline(always)]
    pub fn pi_caslat_lin_f1(&self) -> PiCaslatLinF1R {
        PiCaslatLinF1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
DRAM WRLAT value in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_wrlat_f2(&self) -> PiWrlatF2R {
        PiWrlatF2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
DRAM additive latency value in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_additive_lat_f2(&self) -> PiAdditiveLatF2R {
        PiAdditiveLatF2R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the window after the PI receives a parity error during which DRAM commands will not execute for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tparity_error_cmd_inhibit_f1(
        &mut self,
    ) -> PiTparityErrorCmdInhibitF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi173Spec> {
        PiTparityErrorCmdInhibitF1W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Sets latency from read command sent to data received from/to controller for frequency set 1. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller."]
    #[inline(always)]
    #[must_use]
    pub fn pi_caslat_lin_f1(&mut self) -> PiCaslatLinF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi173Spec> {
        PiCaslatLinF1W::new(self, 8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
DRAM WRLAT value in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlat_f2(&mut self) -> PiWrlatF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi173Spec> {
        PiWrlatF2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
DRAM additive latency value in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_additive_lat_f2(
        &mut self,
    ) -> PiAdditiveLatF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi173Spec> {
        PiAdditiveLatF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_173\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_173::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_173::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi173Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi173Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_173::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi173Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_173::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi173Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_173 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi173Spec {
    const RESET_VALUE: u32 = 0;
}
