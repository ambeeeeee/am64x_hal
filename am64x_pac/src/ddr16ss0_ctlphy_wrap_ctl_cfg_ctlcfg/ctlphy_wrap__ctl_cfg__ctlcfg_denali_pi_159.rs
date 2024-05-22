#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_159` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi159Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_159` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi159Spec>;
#[doc = "Field `PI_WRLVL_MAX_STROBE_PEND` reader - 23:16\\]
Defines the maximum number of wrlvl_strobes that be accumulated before an AREF is prevented from being generated."]
pub type PiWrlvlMaxStrobePendR = crate::FieldReader;
#[doc = "Field `PI_WRLVL_MAX_STROBE_PEND` writer - 23:16\\]
Defines the maximum number of wrlvl_strobes that be accumulated before an AREF is prevented from being generated."]
pub type PiWrlvlMaxStrobePendW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - 23:16\\]
Defines the maximum number of wrlvl_strobes that be accumulated before an AREF is prevented from being generated."]
    #[inline(always)]
    pub fn pi_wrlvl_max_strobe_pend(&self) -> PiWrlvlMaxStrobePendR {
        PiWrlvlMaxStrobePendR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - 23:16\\]
Defines the maximum number of wrlvl_strobes that be accumulated before an AREF is prevented from being generated."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_max_strobe_pend(
        &mut self,
    ) -> PiWrlvlMaxStrobePendW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi159Spec> {
        PiWrlvlMaxStrobePendW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_159\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_159::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_159::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi159Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi159Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_159::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi159Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_159::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi159Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_159 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi159Spec {
    const RESET_VALUE: u32 = 0;
}
