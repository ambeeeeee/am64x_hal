#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_276` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi276Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_276` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi276Spec>;
#[doc = "Field `PI_TZQCAL_F1` reader - 27:16\\]
Holds the DRAM ZQCAL value for frequency set 1 in cycles."]
pub type PiTzqcalF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TZQCAL_F1` writer - 27:16\\]
Holds the DRAM ZQCAL value for frequency set 1 in cycles."]
pub type PiTzqcalF1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 16:27 - 27:16\\]
Holds the DRAM ZQCAL value for frequency set 1 in cycles."]
    #[inline(always)]
    pub fn pi_tzqcal_f1(&self) -> PiTzqcalF1R {
        PiTzqcalF1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - 27:16\\]
Holds the DRAM ZQCAL value for frequency set 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tzqcal_f1(&mut self) -> PiTzqcalF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi276Spec> {
        PiTzqcalF1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_276\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_276::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_276::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi276Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi276Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_276::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi276Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_276::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi276Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_276 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi276Spec {
    const RESET_VALUE: u32 = 0;
}
