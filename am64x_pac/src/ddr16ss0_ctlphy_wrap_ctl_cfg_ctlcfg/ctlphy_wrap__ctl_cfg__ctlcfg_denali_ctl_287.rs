#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_287` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl287Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_287` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl287Spec>;
#[doc = "Field `BIST_DATA_PATTERN_1` reader - 31:0\\]
Data pattern to be used when the BIST_TEST_MODE parameter is programmed to 1, 2, 3 or 4. Only data corresponding to active portion of core word will be used while inactive portion will be ignored."]
pub type BistDataPattern1R = crate::FieldReader<u32>;
#[doc = "Field `BIST_DATA_PATTERN_1` writer - 31:0\\]
Data pattern to be used when the BIST_TEST_MODE parameter is programmed to 1, 2, 3 or 4. Only data corresponding to active portion of core word will be used while inactive portion will be ignored."]
pub type BistDataPattern1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Data pattern to be used when the BIST_TEST_MODE parameter is programmed to 1, 2, 3 or 4. Only data corresponding to active portion of core word will be used while inactive portion will be ignored."]
    #[inline(always)]
    pub fn bist_data_pattern_1(&self) -> BistDataPattern1R {
        BistDataPattern1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Data pattern to be used when the BIST_TEST_MODE parameter is programmed to 1, 2, 3 or 4. Only data corresponding to active portion of core word will be used while inactive portion will be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn bist_data_pattern_1(
        &mut self,
    ) -> BistDataPattern1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl287Spec> {
        BistDataPattern1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_287\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_287::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_287::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl287Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl287Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_287::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl287Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_287::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl287Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_287 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl287Spec {
    const RESET_VALUE: u32 = 0;
}
