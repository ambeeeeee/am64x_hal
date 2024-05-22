#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_125` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl125Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_125` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl125Spec>;
#[doc = "Field `TDFI_PHYMSTR_RESP_F0` reader - 19:0\\]
Defines the DFI tPHYMSTR_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
to be set to 1 in the PHYMSTR_ERROR_STATUS parameter. FC=0"]
pub type TdfiPhymstrRespF0R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_PHYMSTR_RESP_F0` writer - 19:0\\]
Defines the DFI tPHYMSTR_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
to be set to 1 in the PHYMSTR_ERROR_STATUS parameter. FC=0"]
pub type TdfiPhymstrRespF0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Defines the DFI tPHYMSTR_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
to be set to 1 in the PHYMSTR_ERROR_STATUS parameter. FC=0"]
    #[inline(always)]
    pub fn tdfi_phymstr_resp_f0(&self) -> TdfiPhymstrRespF0R {
        TdfiPhymstrRespF0R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Defines the DFI tPHYMSTR_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
to be set to 1 in the PHYMSTR_ERROR_STATUS parameter. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phymstr_resp_f0(
        &mut self,
    ) -> TdfiPhymstrRespF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl125Spec> {
        TdfiPhymstrRespF0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_125\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_125::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_125::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl125Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl125Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_125::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl125Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_125::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl125Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_125 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl125Spec {
    const RESET_VALUE: u32 = 0;
}
