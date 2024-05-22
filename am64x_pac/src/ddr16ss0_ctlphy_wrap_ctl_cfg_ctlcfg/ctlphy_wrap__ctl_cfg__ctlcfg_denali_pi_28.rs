#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_28` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi28Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_28` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi28Spec>;
#[doc = "Field `PI_TDFI_WRLVL_RESP` reader - 31:0\\]
Defines the DFI tWRLVL_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_wrlvl_req assertion and a dfi_wrlvl_en assertion."]
pub type PiTdfiWrlvlRespR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_WRLVL_RESP` writer - 31:0\\]
Defines the DFI tWRLVL_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_wrlvl_req assertion and a dfi_wrlvl_en assertion."]
pub type PiTdfiWrlvlRespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tWRLVL_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_wrlvl_req assertion and a dfi_wrlvl_en assertion."]
    #[inline(always)]
    pub fn pi_tdfi_wrlvl_resp(&self) -> PiTdfiWrlvlRespR {
        PiTdfiWrlvlRespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tWRLVL_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_wrlvl_req assertion and a dfi_wrlvl_en assertion."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wrlvl_resp(
        &mut self,
    ) -> PiTdfiWrlvlRespW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi28Spec> {
        PiTdfiWrlvlRespW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi28Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_28::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi28Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_28::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_28 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi28Spec {
    const RESET_VALUE: u32 = 0;
}
