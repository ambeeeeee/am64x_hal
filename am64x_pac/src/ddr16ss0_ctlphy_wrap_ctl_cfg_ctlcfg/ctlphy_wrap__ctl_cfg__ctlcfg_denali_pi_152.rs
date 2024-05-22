#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_152` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi152Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_152` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi152Spec>;
#[doc = "Field `PI_MONITOR_STROBE` reader - 7:0\\]
Strobe the pi_monitor once. Every bit corresponds respectively with a pi_monitor. WRITE-ONLY"]
pub type PiMonitorStrobeR = crate::FieldReader;
#[doc = "Field `PI_MONITOR_STROBE` writer - 7:0\\]
Strobe the pi_monitor once. Every bit corresponds respectively with a pi_monitor. WRITE-ONLY"]
pub type PiMonitorStrobeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Strobe the pi_monitor once. Every bit corresponds respectively with a pi_monitor. WRITE-ONLY"]
    #[inline(always)]
    pub fn pi_monitor_strobe(&self) -> PiMonitorStrobeR {
        PiMonitorStrobeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Strobe the pi_monitor once. Every bit corresponds respectively with a pi_monitor. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_strobe(
        &mut self,
    ) -> PiMonitorStrobeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi152Spec> {
        PiMonitorStrobeW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_152\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_152::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_152::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi152Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi152Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_152::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi152Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_152::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi152Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_152 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi152Spec {
    const RESET_VALUE: u32 = 0;
}
