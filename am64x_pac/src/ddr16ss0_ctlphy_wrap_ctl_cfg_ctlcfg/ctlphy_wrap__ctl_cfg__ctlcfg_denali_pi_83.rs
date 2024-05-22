#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_83` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi83Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_83` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi83Spec>;
#[doc = "Field `PI_INT_STATUS` reader - 29:0\\]
Status of interrupt features in the PI. READ-ONLY"]
pub type PiIntStatusR = crate::FieldReader<u32>;
#[doc = "Field `PI_INT_STATUS` writer - 29:0\\]
Status of interrupt features in the PI. READ-ONLY"]
pub type PiIntStatusW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - 29:0\\]
Status of interrupt features in the PI. READ-ONLY"]
    #[inline(always)]
    pub fn pi_int_status(&self) -> PiIntStatusR {
        PiIntStatusR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - 29:0\\]
Status of interrupt features in the PI. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_int_status(&mut self) -> PiIntStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi83Spec> {
        PiIntStatusW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_83\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_83::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_83::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi83Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi83Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_83::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi83Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_83::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi83Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_83 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi83Spec {
    const RESET_VALUE: u32 = 0;
}
