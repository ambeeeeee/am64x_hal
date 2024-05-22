#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_101` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi101Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_101` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi101Spec>;
#[doc = "Field `PI_BIST_ERR_COUNT` reader - 11:0\\]
Indicates the number of BIST errors found when the BIST_TEST_MODE parameter is set to 1, 2 or 3. READ-ONLY"]
pub type PiBistErrCountR = crate::FieldReader<u16>;
#[doc = "Field `PI_BIST_ERR_COUNT` writer - 11:0\\]
Indicates the number of BIST errors found when the BIST_TEST_MODE parameter is set to 1, 2 or 3. READ-ONLY"]
pub type PiBistErrCountW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PI_BIST_ERR_STOP` reader - 27:16\\]
Defines the maximum number of error occurrences allowed prior to quitting when the BIST_TEST_MODE parameter is set to 1, 2 or 3. A value of 0 will allow the test to run to completion."]
pub type PiBistErrStopR = crate::FieldReader<u16>;
#[doc = "Field `PI_BIST_ERR_STOP` writer - 27:16\\]
Defines the maximum number of error occurrences allowed prior to quitting when the BIST_TEST_MODE parameter is set to 1, 2 or 3. A value of 0 will allow the test to run to completion."]
pub type PiBistErrStopW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Indicates the number of BIST errors found when the BIST_TEST_MODE parameter is set to 1, 2 or 3. READ-ONLY"]
    #[inline(always)]
    pub fn pi_bist_err_count(&self) -> PiBistErrCountR {
        PiBistErrCountR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Defines the maximum number of error occurrences allowed prior to quitting when the BIST_TEST_MODE parameter is set to 1, 2 or 3. A value of 0 will allow the test to run to completion."]
    #[inline(always)]
    pub fn pi_bist_err_stop(&self) -> PiBistErrStopR {
        PiBistErrStopR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Indicates the number of BIST errors found when the BIST_TEST_MODE parameter is set to 1, 2 or 3. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_err_count(
        &mut self,
    ) -> PiBistErrCountW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi101Spec> {
        PiBistErrCountW::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Defines the maximum number of error occurrences allowed prior to quitting when the BIST_TEST_MODE parameter is set to 1, 2 or 3. A value of 0 will allow the test to run to completion."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_err_stop(&mut self) -> PiBistErrStopW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi101Spec> {
        PiBistErrStopW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_101\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_101::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_101::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi101Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi101Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_101::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi101Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_101::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi101Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_101 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi101Spec {
    const RESET_VALUE: u32 = 0;
}
