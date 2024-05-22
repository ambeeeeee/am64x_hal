#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_171` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi171Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_171` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi171Spec>;
#[doc = "Field `PI_WRLAT_F0` reader - 6:0\\]
DRAM WRLAT value in cycles for frequency set 0."]
pub type PiWrlatF0R = crate::FieldReader;
#[doc = "Field `PI_WRLAT_F0` writer - 6:0\\]
DRAM WRLAT value in cycles for frequency set 0."]
pub type PiWrlatF0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_ADDITIVE_LAT_F0` reader - 13:8\\]
DRAM additive latency value in cycles for frequency set 0."]
pub type PiAdditiveLatF0R = crate::FieldReader;
#[doc = "Field `PI_ADDITIVE_LAT_F0` writer - 13:8\\]
DRAM additive latency value in cycles for frequency set 0."]
pub type PiAdditiveLatF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_CA_PARITY_LAT_F0` reader - 19:16\\]
DRAM CA parity latency value in cycles for frequency set 0."]
pub type PiCaParityLatF0R = crate::FieldReader;
#[doc = "Field `PI_CA_PARITY_LAT_F0` writer - 19:16\\]
DRAM CA parity latency value in cycles for frequency set 0."]
pub type PiCaParityLatF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TPARITY_ERROR_CMD_INHIBIT_F0` reader - 31:24\\]
Defines the window after the PI receives a parity error during which DRAM commands will not execute for frequency set 0."]
pub type PiTparityErrorCmdInhibitF0R = crate::FieldReader;
#[doc = "Field `PI_TPARITY_ERROR_CMD_INHIBIT_F0` writer - 31:24\\]
Defines the window after the PI receives a parity error during which DRAM commands will not execute for frequency set 0."]
pub type PiTparityErrorCmdInhibitF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
DRAM WRLAT value in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_wrlat_f0(&self) -> PiWrlatF0R {
        PiWrlatF0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
DRAM additive latency value in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_additive_lat_f0(&self) -> PiAdditiveLatF0R {
        PiAdditiveLatF0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
DRAM CA parity latency value in cycles for frequency set 0."]
    #[inline(always)]
    pub fn pi_ca_parity_lat_f0(&self) -> PiCaParityLatF0R {
        PiCaParityLatF0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Defines the window after the PI receives a parity error during which DRAM commands will not execute for frequency set 0."]
    #[inline(always)]
    pub fn pi_tparity_error_cmd_inhibit_f0(&self) -> PiTparityErrorCmdInhibitF0R {
        PiTparityErrorCmdInhibitF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
DRAM WRLAT value in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlat_f0(&mut self) -> PiWrlatF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi171Spec> {
        PiWrlatF0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
DRAM additive latency value in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_additive_lat_f0(
        &mut self,
    ) -> PiAdditiveLatF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi171Spec> {
        PiAdditiveLatF0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
DRAM CA parity latency value in cycles for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_ca_parity_lat_f0(
        &mut self,
    ) -> PiCaParityLatF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi171Spec> {
        PiCaParityLatF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Defines the window after the PI receives a parity error during which DRAM commands will not execute for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tparity_error_cmd_inhibit_f0(
        &mut self,
    ) -> PiTparityErrorCmdInhibitF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi171Spec> {
        PiTparityErrorCmdInhibitF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_171\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_171::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_171::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi171Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi171Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_171::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi171Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_171::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi171Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_171 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi171Spec {
    const RESET_VALUE: u32 = 0;
}
