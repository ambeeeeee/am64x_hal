#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_123` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi123Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_123` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi123Spec>;
#[doc = "Field `PI_BIST_USER_PAT_1` reader - 31:0\\]
Sets the user-specified pattern of BIST."]
pub type PiBistUserPat1R = crate::FieldReader<u32>;
#[doc = "Field `PI_BIST_USER_PAT_1` writer - 31:0\\]
Sets the user-specified pattern of BIST."]
pub type PiBistUserPat1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Sets the user-specified pattern of BIST."]
    #[inline(always)]
    pub fn pi_bist_user_pat_1(&self) -> PiBistUserPat1R {
        PiBistUserPat1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Sets the user-specified pattern of BIST."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_user_pat_1(
        &mut self,
    ) -> PiBistUserPat1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi123Spec> {
        PiBistUserPat1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_123\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_123::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_123::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi123Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi123Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_123::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi123Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_123::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi123Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_123 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi123Spec {
    const RESET_VALUE: u32 = 0;
}
