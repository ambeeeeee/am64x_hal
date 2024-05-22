#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_385` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl385Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_385` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl385Spec>;
#[doc = "Field `TDFI_CTRLUPD_MAX_F0` reader - 20:0\\]
Defines the DFI tCTRLUPD_MAX timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[0\\]
set in the UPDATE_ERROR_STATUS parameter. FC=0"]
pub type TdfiCtrlupdMaxF0R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_CTRLUPD_MAX_F0` writer - 20:0\\]
Defines the DFI tCTRLUPD_MAX timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[0\\]
set in the UPDATE_ERROR_STATUS parameter. FC=0"]
pub type TdfiCtrlupdMaxF0W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
Defines the DFI tCTRLUPD_MAX timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[0\\]
set in the UPDATE_ERROR_STATUS parameter. FC=0"]
    #[inline(always)]
    pub fn tdfi_ctrlupd_max_f0(&self) -> TdfiCtrlupdMaxF0R {
        TdfiCtrlupdMaxF0R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - 20:0\\]
Defines the DFI tCTRLUPD_MAX timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[0\\]
set in the UPDATE_ERROR_STATUS parameter. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrlupd_max_f0(
        &mut self,
    ) -> TdfiCtrlupdMaxF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl385Spec> {
        TdfiCtrlupdMaxF0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_385\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_385::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_385::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl385Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl385Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_385::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl385Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_385::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl385Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_385 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl385Spec {
    const RESET_VALUE: u32 = 0;
}
