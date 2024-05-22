#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_32` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi32Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_32` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi32Spec>;
#[doc = "Field `PI_CA_PARITY_ERROR_INJECT` reader - 25:0\\]
Selects bit to corrupt on the CA bus for CA parity error injection."]
pub type PiCaParityErrorInjectR = crate::FieldReader<u32>;
#[doc = "Field `PI_CA_PARITY_ERROR_INJECT` writer - 25:0\\]
Selects bit to corrupt on the CA bus for CA parity error injection."]
pub type PiCaParityErrorInjectW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - 25:0\\]
Selects bit to corrupt on the CA bus for CA parity error injection."]
    #[inline(always)]
    pub fn pi_ca_parity_error_inject(&self) -> PiCaParityErrorInjectR {
        PiCaParityErrorInjectR::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - 25:0\\]
Selects bit to corrupt on the CA bus for CA parity error injection."]
    #[inline(always)]
    #[must_use]
    pub fn pi_ca_parity_error_inject(
        &mut self,
    ) -> PiCaParityErrorInjectW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi32Spec> {
        PiCaParityErrorInjectW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi32Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_32::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi32Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_32::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_32 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi32Spec {
    const RESET_VALUE: u32 = 0;
}
