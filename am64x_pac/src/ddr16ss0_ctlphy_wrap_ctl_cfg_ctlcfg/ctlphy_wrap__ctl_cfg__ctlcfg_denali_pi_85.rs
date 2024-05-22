#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_85` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi85Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_85` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi85Spec>;
#[doc = "Field `PI_INT_MASK` reader - 29:0\\]
Mask for PI_int signals from the PI_INT_STATUS parameter."]
pub type PiIntMaskR = crate::FieldReader<u32>;
#[doc = "Field `PI_INT_MASK` writer - 29:0\\]
Mask for PI_int signals from the PI_INT_STATUS parameter."]
pub type PiIntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - 29:0\\]
Mask for PI_int signals from the PI_INT_STATUS parameter."]
    #[inline(always)]
    pub fn pi_int_mask(&self) -> PiIntMaskR {
        PiIntMaskR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - 29:0\\]
Mask for PI_int signals from the PI_INT_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_int_mask(&mut self) -> PiIntMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi85Spec> {
        PiIntMaskW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_85\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_85::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_85::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi85Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi85Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_85::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi85Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_85::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi85Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_85 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi85Spec {
    const RESET_VALUE: u32 = 0;
}
