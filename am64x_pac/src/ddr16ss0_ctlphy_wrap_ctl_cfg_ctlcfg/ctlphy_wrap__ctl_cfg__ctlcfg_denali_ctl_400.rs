#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_400` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl400Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_400` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl400Spec>;
#[doc = "Field `TDFI_CTRLUPD_INTERVAL_F1` reader - 31:0\\]
Defines the DFI tCTRLUPD_INTERVAL timing parameter \\[in DFI clocks\\], the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[6\\]
set in the UPDATE_ERROR_STATUS parameter. FC=1"]
pub type TdfiCtrlupdIntervalF1R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_CTRLUPD_INTERVAL_F1` writer - 31:0\\]
Defines the DFI tCTRLUPD_INTERVAL timing parameter \\[in DFI clocks\\], the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[6\\]
set in the UPDATE_ERROR_STATUS parameter. FC=1"]
pub type TdfiCtrlupdIntervalF1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tCTRLUPD_INTERVAL timing parameter \\[in DFI clocks\\], the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[6\\]
set in the UPDATE_ERROR_STATUS parameter. FC=1"]
    #[inline(always)]
    pub fn tdfi_ctrlupd_interval_f1(&self) -> TdfiCtrlupdIntervalF1R {
        TdfiCtrlupdIntervalF1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tCTRLUPD_INTERVAL timing parameter \\[in DFI clocks\\], the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[6\\]
set in the UPDATE_ERROR_STATUS parameter. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrlupd_interval_f1(
        &mut self,
    ) -> TdfiCtrlupdIntervalF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl400Spec> {
        TdfiCtrlupdIntervalF1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_400\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_400::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_400::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl400Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl400Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_400::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl400Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_400::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl400Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_400 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl400Spec {
    const RESET_VALUE: u32 = 0;
}
