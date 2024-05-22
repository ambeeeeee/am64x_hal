#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_57` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi57Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_57` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi57Spec>;
#[doc = "Field `PI_TDFI_CALVL_RESP` reader - 31:0\\]
Defines the DFI tCALVL_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_calvl_req assertion and a dfi_calvl_en assertion."]
pub type PiTdfiCalvlRespR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_CALVL_RESP` writer - 31:0\\]
Defines the DFI tCALVL_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_calvl_req assertion and a dfi_calvl_en assertion."]
pub type PiTdfiCalvlRespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tCALVL_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_calvl_req assertion and a dfi_calvl_en assertion."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_resp(&self) -> PiTdfiCalvlRespR {
        PiTdfiCalvlRespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tCALVL_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_calvl_req assertion and a dfi_calvl_en assertion."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_resp(
        &mut self,
    ) -> PiTdfiCalvlRespW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi57Spec> {
        PiTdfiCalvlRespW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_57\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_57::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_57::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi57Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi57Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_57::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi57Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_57::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi57Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_57 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi57Spec {
    const RESET_VALUE: u32 = 0;
}
