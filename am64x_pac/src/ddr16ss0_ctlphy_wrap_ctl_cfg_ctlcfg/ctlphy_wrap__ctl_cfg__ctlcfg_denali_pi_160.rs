#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_160` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi160Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_160` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi160Spec>;
#[doc = "Field `PI_TREFBW_THR` reader - 8:0\\]
Threshold value to control the AREF command interval. When the number of pending AREF is over this value, the interval is expanded to be tREF/8."]
pub type PiTrefbwThrR = crate::FieldReader<u16>;
#[doc = "Field `PI_TREFBW_THR` writer - 8:0\\]
Threshold value to control the AREF command interval. When the number of pending AREF is over this value, the interval is expanded to be tREF/8."]
pub type PiTrefbwThrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Threshold value to control the AREF command interval. When the number of pending AREF is over this value, the interval is expanded to be tREF/8."]
    #[inline(always)]
    pub fn pi_trefbw_thr(&self) -> PiTrefbwThrR {
        PiTrefbwThrR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Threshold value to control the AREF command interval. When the number of pending AREF is over this value, the interval is expanded to be tREF/8."]
    #[inline(always)]
    #[must_use]
    pub fn pi_trefbw_thr(&mut self) -> PiTrefbwThrW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi160Spec> {
        PiTrefbwThrW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_160\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_160::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_160::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi160Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi160Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_160::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi160Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_160::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi160Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_160 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi160Spec {
    const RESET_VALUE: u32 = 0;
}
