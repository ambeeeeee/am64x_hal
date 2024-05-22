#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_47` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi47Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_47` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi47Spec>;
#[doc = "Field `PI_TDFI_RDLVL_RESP` reader - 31:0\\]
Defines the DFI tRDLVL_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_rdlvl_req or dfi_rdlvl_gate_req assertion and a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion."]
pub type PiTdfiRdlvlRespR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_RDLVL_RESP` writer - 31:0\\]
Defines the DFI tRDLVL_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_rdlvl_req or dfi_rdlvl_gate_req assertion and a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion."]
pub type PiTdfiRdlvlRespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tRDLVL_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_rdlvl_req or dfi_rdlvl_gate_req assertion and a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion."]
    #[inline(always)]
    pub fn pi_tdfi_rdlvl_resp(&self) -> PiTdfiRdlvlRespR {
        PiTdfiRdlvlRespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tRDLVL_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_rdlvl_req or dfi_rdlvl_gate_req assertion and a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_rdlvl_resp(
        &mut self,
    ) -> PiTdfiRdlvlRespW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi47Spec> {
        PiTdfiRdlvlRespW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_47::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_47::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi47Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_47::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi47Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_47::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi47Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_47 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi47Spec {
    const RESET_VALUE: u32 = 0;
}
