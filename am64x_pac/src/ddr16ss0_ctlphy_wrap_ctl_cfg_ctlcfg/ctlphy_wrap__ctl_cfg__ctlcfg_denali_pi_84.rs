#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_84` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi84Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_84` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi84Spec>;
#[doc = "Field `PI_INT_ACK` reader - 28:0\\]
Clear the corresponding interrupt bit of the PI_INT_STATUS parameter. WRITE-ONLY"]
pub type PiIntAckR = crate::FieldReader<u32>;
#[doc = "Field `PI_INT_ACK` writer - 28:0\\]
Clear the corresponding interrupt bit of the PI_INT_STATUS parameter. WRITE-ONLY"]
pub type PiIntAckW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - 28:0\\]
Clear the corresponding interrupt bit of the PI_INT_STATUS parameter. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_int_ack(&self) -> PiIntAckR {
        PiIntAckR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - 28:0\\]
Clear the corresponding interrupt bit of the PI_INT_STATUS parameter. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_int_ack(&mut self) -> PiIntAckW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi84Spec> {
        PiIntAckW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_84::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_84::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi84Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi84Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_84::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi84Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_84::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi84Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_84 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi84Spec {
    const RESET_VALUE: u32 = 0;
}
