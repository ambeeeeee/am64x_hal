#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_139` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl139Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_139` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl139Spec>;
#[doc = "Field `TDFI_PHYMSTR_RESP_F2` reader - 19:0\\]
Defines the DFI tPHYMSTR_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
to be set to 1 in the PHYMSTR_ERROR_STATUS parameter. FC=2"]
pub type TdfiPhymstrRespF2R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_PHYMSTR_RESP_F2` writer - 19:0\\]
Defines the DFI tPHYMSTR_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
to be set to 1 in the PHYMSTR_ERROR_STATUS parameter. FC=2"]
pub type TdfiPhymstrRespF2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `PHYMSTR_NO_AREF` reader - 24:24\\]
isables refreshes during the PHY master interface sequence. Set to 1 to disable. Refreshes during reset are only supported for DFI 4.0 and this parameter may be set or cleared for DFI 4.0. For all other DFI versions, this parameter must be set to 1."]
pub type PhymstrNoArefR = crate::BitReader;
#[doc = "Field `PHYMSTR_NO_AREF` writer - 24:24\\]
isables refreshes during the PHY master interface sequence. Set to 1 to disable. Refreshes during reset are only supported for DFI 4.0 and this parameter may be set or cleared for DFI 4.0. For all other DFI versions, this parameter must be set to 1."]
pub type PhymstrNoArefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Defines the DFI tPHYMSTR_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
to be set to 1 in the PHYMSTR_ERROR_STATUS parameter. FC=2"]
    #[inline(always)]
    pub fn tdfi_phymstr_resp_f2(&self) -> TdfiPhymstrRespF2R {
        TdfiPhymstrRespF2R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
isables refreshes during the PHY master interface sequence. Set to 1 to disable. Refreshes during reset are only supported for DFI 4.0 and this parameter may be set or cleared for DFI 4.0. For all other DFI versions, this parameter must be set to 1."]
    #[inline(always)]
    pub fn phymstr_no_aref(&self) -> PhymstrNoArefR {
        PhymstrNoArefR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Defines the DFI tPHYMSTR_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_phymstr_req assertion and a dfi_phymstr_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
to be set to 1 in the PHYMSTR_ERROR_STATUS parameter. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phymstr_resp_f2(
        &mut self,
    ) -> TdfiPhymstrRespF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl139Spec> {
        TdfiPhymstrRespF2W::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
isables refreshes during the PHY master interface sequence. Set to 1 to disable. Refreshes during reset are only supported for DFI 4.0 and this parameter may be set or cleared for DFI 4.0. For all other DFI versions, this parameter must be set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn phymstr_no_aref(&mut self) -> PhymstrNoArefW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl139Spec> {
        PhymstrNoArefW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_139\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_139::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_139::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl139Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl139Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_139::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl139Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_139::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl139Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_139 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl139Spec {
    const RESET_VALUE: u32 = 0;
}
