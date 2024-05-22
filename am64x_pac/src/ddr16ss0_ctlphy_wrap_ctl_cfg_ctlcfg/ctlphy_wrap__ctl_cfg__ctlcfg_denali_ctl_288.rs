#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_288` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl288Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_288` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl288Spec>;
#[doc = "Field `BIST_RET_STATE` reader - 0:0\\]
Indicates if BIST is in a retention wait state, used when the BIST_TEST_MODE parameter is programmed to 2 or 3. Value of 1 indicates BIST is waiting. READ-ONLY"]
pub type BistRetStateR = crate::BitReader;
#[doc = "Field `BIST_RET_STATE` writer - 0:0\\]
Indicates if BIST is in a retention wait state, used when the BIST_TEST_MODE parameter is programmed to 2 or 3. Value of 1 indicates BIST is waiting. READ-ONLY"]
pub type BistRetStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIST_ERR_STOP` reader - 19:8\\]
Defines the maximum number of error occurrences allowed prior to quitting when the BIST_TEST_MODE parameter is programmed to 1, 2 or 3. A value of 0 will allow the test to run to completion."]
pub type BistErrStopR = crate::FieldReader<u16>;
#[doc = "Field `BIST_ERR_STOP` writer - 19:8\\]
Defines the maximum number of error occurrences allowed prior to quitting when the BIST_TEST_MODE parameter is programmed to 1, 2 or 3. A value of 0 will allow the test to run to completion."]
pub type BistErrStopW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates if BIST is in a retention wait state, used when the BIST_TEST_MODE parameter is programmed to 2 or 3. Value of 1 indicates BIST is waiting. READ-ONLY"]
    #[inline(always)]
    pub fn bist_ret_state(&self) -> BistRetStateR {
        BistRetStateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Defines the maximum number of error occurrences allowed prior to quitting when the BIST_TEST_MODE parameter is programmed to 1, 2 or 3. A value of 0 will allow the test to run to completion."]
    #[inline(always)]
    pub fn bist_err_stop(&self) -> BistErrStopR {
        BistErrStopR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates if BIST is in a retention wait state, used when the BIST_TEST_MODE parameter is programmed to 2 or 3. Value of 1 indicates BIST is waiting. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn bist_ret_state(&mut self) -> BistRetStateW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl288Spec> {
        BistRetStateW::new(self, 0)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Defines the maximum number of error occurrences allowed prior to quitting when the BIST_TEST_MODE parameter is programmed to 1, 2 or 3. A value of 0 will allow the test to run to completion."]
    #[inline(always)]
    #[must_use]
    pub fn bist_err_stop(&mut self) -> BistErrStopW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl288Spec> {
        BistErrStopW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_288\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_288::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_288::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl288Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl288Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_288::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl288Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_288::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl288Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_288 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl288Spec {
    const RESET_VALUE: u32 = 0;
}
