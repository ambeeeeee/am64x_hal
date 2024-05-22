#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_95` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi95Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_95` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi95Spec>;
#[doc = "Field `PI_CTRLUPD_REQ_PER_AREF_EN` reader - 0:0\\]
Enable an automatic PI initiated update \\[dfi_ctrlupd_req\\]
after every refresh. Set to 1 to enable."]
pub type PiCtrlupdReqPerArefEnR = crate::BitReader;
#[doc = "Field `PI_CTRLUPD_REQ_PER_AREF_EN` writer - 0:0\\]
Enable an automatic PI initiated update \\[dfi_ctrlupd_req\\]
after every refresh. Set to 1 to enable."]
pub type PiCtrlupdReqPerArefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_TDFI_CTRLUPD_MIN` reader - 23:8\\]
Reports the DFI tCTRLUPD_MIN timing parameter \\[in DFI clocks\\], the minimum cycles that dfi_ctrlupd_req must be asserted."]
pub type PiTdfiCtrlupdMinR = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_CTRLUPD_MIN` writer - 23:8\\]
Reports the DFI tCTRLUPD_MIN timing parameter \\[in DFI clocks\\], the minimum cycles that dfi_ctrlupd_req must be asserted."]
pub type PiTdfiCtrlupdMinW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_UPDATE_ERROR_STATUS` reader - 25:24\\]
Identifies the source of any DFI PI-initiated update errors. Value of 1 indicates a timing violation of the associated timing parameter. Bit 1-0: ctrlupd_max_error, ctrlupd_interval_error. Bit 6-2: reserved. READ-ONLY"]
pub type PiUpdateErrorStatusR = crate::FieldReader;
#[doc = "Field `PI_UPDATE_ERROR_STATUS` writer - 25:24\\]
Identifies the source of any DFI PI-initiated update errors. Value of 1 indicates a timing violation of the associated timing parameter. Bit 1-0: ctrlupd_max_error, ctrlupd_interval_error. Bit 6-2: reserved. READ-ONLY"]
pub type PiUpdateErrorStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable an automatic PI initiated update \\[dfi_ctrlupd_req\\]
after every refresh. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_ctrlupd_req_per_aref_en(&self) -> PiCtrlupdReqPerArefEnR {
        PiCtrlupdReqPerArefEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Reports the DFI tCTRLUPD_MIN timing parameter \\[in DFI clocks\\], the minimum cycles that dfi_ctrlupd_req must be asserted."]
    #[inline(always)]
    pub fn pi_tdfi_ctrlupd_min(&self) -> PiTdfiCtrlupdMinR {
        PiTdfiCtrlupdMinR::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Identifies the source of any DFI PI-initiated update errors. Value of 1 indicates a timing violation of the associated timing parameter. Bit 1-0: ctrlupd_max_error, ctrlupd_interval_error. Bit 6-2: reserved. READ-ONLY"]
    #[inline(always)]
    pub fn pi_update_error_status(&self) -> PiUpdateErrorStatusR {
        PiUpdateErrorStatusR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable an automatic PI initiated update \\[dfi_ctrlupd_req\\]
after every refresh. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_ctrlupd_req_per_aref_en(
        &mut self,
    ) -> PiCtrlupdReqPerArefEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi95Spec> {
        PiCtrlupdReqPerArefEnW::new(self, 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Reports the DFI tCTRLUPD_MIN timing parameter \\[in DFI clocks\\], the minimum cycles that dfi_ctrlupd_req must be asserted."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrlupd_min(
        &mut self,
    ) -> PiTdfiCtrlupdMinW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi95Spec> {
        PiTdfiCtrlupdMinW::new(self, 8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Identifies the source of any DFI PI-initiated update errors. Value of 1 indicates a timing violation of the associated timing parameter. Bit 1-0: ctrlupd_max_error, ctrlupd_interval_error. Bit 6-2: reserved. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_update_error_status(
        &mut self,
    ) -> PiUpdateErrorStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi95Spec> {
        PiUpdateErrorStatusW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_95\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_95::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_95::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi95Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi95Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_95::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi95Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_95::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi95Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_95 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi95Spec {
    const RESET_VALUE: u32 = 0;
}
