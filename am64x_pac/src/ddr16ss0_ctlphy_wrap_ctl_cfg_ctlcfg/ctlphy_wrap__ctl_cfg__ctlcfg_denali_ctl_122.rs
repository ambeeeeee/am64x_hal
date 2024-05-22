#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_122` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl122Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_122` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl122Spec>;
#[doc = "Field `TDFI_PHYMSTR_MAX_TYPE2_F0` reader - 31:0\\]
Defines the DFI 4.0v2 tPHYMSTR_MAX_TYPE2 timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_phymstr_req can be asserted following the assertion of dfi_phymstr_ack for dfi_phymstr_type=2. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[0\\]
set in the PHYMSTR_ERROR_STATUS parameter. FC=0"]
pub type TdfiPhymstrMaxType2F0R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_PHYMSTR_MAX_TYPE2_F0` writer - 31:0\\]
Defines the DFI 4.0v2 tPHYMSTR_MAX_TYPE2 timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_phymstr_req can be asserted following the assertion of dfi_phymstr_ack for dfi_phymstr_type=2. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[0\\]
set in the PHYMSTR_ERROR_STATUS parameter. FC=0"]
pub type TdfiPhymstrMaxType2F0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI 4.0v2 tPHYMSTR_MAX_TYPE2 timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_phymstr_req can be asserted following the assertion of dfi_phymstr_ack for dfi_phymstr_type=2. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[0\\]
set in the PHYMSTR_ERROR_STATUS parameter. FC=0"]
    #[inline(always)]
    pub fn tdfi_phymstr_max_type2_f0(&self) -> TdfiPhymstrMaxType2F0R {
        TdfiPhymstrMaxType2F0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI 4.0v2 tPHYMSTR_MAX_TYPE2 timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_phymstr_req can be asserted following the assertion of dfi_phymstr_ack for dfi_phymstr_type=2. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[0\\]
set in the PHYMSTR_ERROR_STATUS parameter. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phymstr_max_type2_f0(
        &mut self,
    ) -> TdfiPhymstrMaxType2F0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl122Spec> {
        TdfiPhymstrMaxType2F0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_122\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_122::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_122::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl122Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl122Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_122::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl122Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_122::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl122Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_122 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl122Spec {
    const RESET_VALUE: u32 = 0;
}
