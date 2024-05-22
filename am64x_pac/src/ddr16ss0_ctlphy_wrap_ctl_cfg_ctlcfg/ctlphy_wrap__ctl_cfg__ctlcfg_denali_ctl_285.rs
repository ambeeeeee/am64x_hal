#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_285` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl285Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_285` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl285Spec>;
#[doc = "Field `BIST_TEST_MODE` reader - 2:0\\]
Sets the BIST test mode. Value of 0 specifies standard BIST operation, value of 1 specifies a reduced BIST operation, value of 2 specifies a self-refresh retention test, value of 3 specifies an idle retention test, and value of 4 specifies memory initalization function. All other values are reserved."]
pub type BistTestModeR = crate::FieldReader;
#[doc = "Field `BIST_TEST_MODE` writer - 2:0\\]
Sets the BIST test mode. Value of 0 specifies standard BIST operation, value of 1 specifies a reduced BIST operation, value of 2 specifies a self-refresh retention test, value of 3 specifies an idle retention test, and value of 4 specifies memory initalization function. All other values are reserved."]
pub type BistTestModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Sets the BIST test mode. Value of 0 specifies standard BIST operation, value of 1 specifies a reduced BIST operation, value of 2 specifies a self-refresh retention test, value of 3 specifies an idle retention test, and value of 4 specifies memory initalization function. All other values are reserved."]
    #[inline(always)]
    pub fn bist_test_mode(&self) -> BistTestModeR {
        BistTestModeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Sets the BIST test mode. Value of 0 specifies standard BIST operation, value of 1 specifies a reduced BIST operation, value of 2 specifies a self-refresh retention test, value of 3 specifies an idle retention test, and value of 4 specifies memory initalization function. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn bist_test_mode(&mut self) -> BistTestModeW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl285Spec> {
        BistTestModeW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_285\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_285::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_285::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl285Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl285Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_285::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl285Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_285::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl285Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_285 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl285Spec {
    const RESET_VALUE: u32 = 0;
}
