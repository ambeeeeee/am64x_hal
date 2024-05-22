#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_174` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi174Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_174` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi174Spec>;
#[doc = "Field `PI_CA_PARITY_LAT_F2` reader - 3:0\\]
DRAM CA parity latency value in cycles for frequency set 2."]
pub type PiCaParityLatF2R = crate::FieldReader;
#[doc = "Field `PI_CA_PARITY_LAT_F2` writer - 3:0\\]
DRAM CA parity latency value in cycles for frequency set 2."]
pub type PiCaParityLatF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TPARITY_ERROR_CMD_INHIBIT_F2` reader - 15:8\\]
Defines the window after the PI receives a parity error during which DRAM commands will not execute for frequency set 2."]
pub type PiTparityErrorCmdInhibitF2R = crate::FieldReader;
#[doc = "Field `PI_TPARITY_ERROR_CMD_INHIBIT_F2` writer - 15:8\\]
Defines the window after the PI receives a parity error during which DRAM commands will not execute for frequency set 2."]
pub type PiTparityErrorCmdInhibitF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_CASLAT_LIN_F2` reader - 22:16\\]
Sets latency from read command sent to data received from/to controller for frequency set 2. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller."]
pub type PiCaslatLinF2R = crate::FieldReader;
#[doc = "Field `PI_CASLAT_LIN_F2` writer - 22:16\\]
Sets latency from read command sent to data received from/to controller for frequency set 2. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller."]
pub type PiCaslatLinF2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
DRAM CA parity latency value in cycles for frequency set 2."]
    #[inline(always)]
    pub fn pi_ca_parity_lat_f2(&self) -> PiCaParityLatF2R {
        PiCaParityLatF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the window after the PI receives a parity error during which DRAM commands will not execute for frequency set 2."]
    #[inline(always)]
    pub fn pi_tparity_error_cmd_inhibit_f2(&self) -> PiTparityErrorCmdInhibitF2R {
        PiTparityErrorCmdInhibitF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Sets latency from read command sent to data received from/to controller for frequency set 2. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller."]
    #[inline(always)]
    pub fn pi_caslat_lin_f2(&self) -> PiCaslatLinF2R {
        PiCaslatLinF2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
DRAM CA parity latency value in cycles for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_ca_parity_lat_f2(
        &mut self,
    ) -> PiCaParityLatF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi174Spec> {
        PiCaParityLatF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the window after the PI receives a parity error during which DRAM commands will not execute for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tparity_error_cmd_inhibit_f2(
        &mut self,
    ) -> PiTparityErrorCmdInhibitF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi174Spec> {
        PiTparityErrorCmdInhibitF2W::new(self, 8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Sets latency from read command sent to data received from/to controller for frequency set 2. Bit \\[0\\]
is half-cycle increment and the upper bits define memory CAS latency for the controller."]
    #[inline(always)]
    #[must_use]
    pub fn pi_caslat_lin_f2(&mut self) -> PiCaslatLinF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi174Spec> {
        PiCaslatLinF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_174\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_174::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_174::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi174Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi174Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_174::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi174Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_174::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi174Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_174 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi174Spec {
    const RESET_VALUE: u32 = 0;
}
