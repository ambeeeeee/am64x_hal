#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_151` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi151Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_151` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi151Spec>;
#[doc = "Field `PI_MONITOR_7` reader - 7:0\\]
Monitor register 7. READ-ONLY."]
pub type PiMonitor7R = crate::FieldReader;
#[doc = "Field `PI_MONITOR_7` writer - 7:0\\]
Monitor register 7. READ-ONLY."]
pub type PiMonitor7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Monitor register 7. READ-ONLY."]
    #[inline(always)]
    pub fn pi_monitor_7(&self) -> PiMonitor7R {
        PiMonitor7R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Monitor register 7. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_monitor_7(&mut self) -> PiMonitor7W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi151Spec> {
        PiMonitor7W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_151\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_151::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_151::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi151Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi151Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_151::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi151Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_151::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi151Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_151 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi151Spec {
    const RESET_VALUE: u32 = 0;
}
