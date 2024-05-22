#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_274` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi274Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_274` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi274Spec>;
#[doc = "Field `PI_TZQCAL_F0` reader - 27:16\\]
Holds the DRAM ZQCAL value for frequency set 0 in cycles."]
pub type PiTzqcalF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TZQCAL_F0` writer - 27:16\\]
Holds the DRAM ZQCAL value for frequency set 0 in cycles."]
pub type PiTzqcalF0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 16:27 - 27:16\\]
Holds the DRAM ZQCAL value for frequency set 0 in cycles."]
    #[inline(always)]
    pub fn pi_tzqcal_f0(&self) -> PiTzqcalF0R {
        PiTzqcalF0R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - 27:16\\]
Holds the DRAM ZQCAL value for frequency set 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tzqcal_f0(&mut self) -> PiTzqcalF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi274Spec> {
        PiTzqcalF0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_274\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_274::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_274::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi274Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi274Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_274::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi274Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_274::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi274Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_274 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi274Spec {
    const RESET_VALUE: u32 = 0;
}
