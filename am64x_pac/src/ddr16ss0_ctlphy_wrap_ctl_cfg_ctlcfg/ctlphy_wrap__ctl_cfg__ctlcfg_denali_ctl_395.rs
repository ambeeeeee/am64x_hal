#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_395` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl395Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_395` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl395Spec>;
#[doc = "Field `TDFI_PHYUPD_TYPE0_F1` reader - 31:0\\]
Defines the DFI tPHYUPD_TYPE0 timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
set in the UPDATE_ERROR_STATUS parameter. FC=1"]
pub type TdfiPhyupdType0F1R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_PHYUPD_TYPE0_F1` writer - 31:0\\]
Defines the DFI tPHYUPD_TYPE0 timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
set in the UPDATE_ERROR_STATUS parameter. FC=1"]
pub type TdfiPhyupdType0F1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tPHYUPD_TYPE0 timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
set in the UPDATE_ERROR_STATUS parameter. FC=1"]
    #[inline(always)]
    pub fn tdfi_phyupd_type0_f1(&self) -> TdfiPhyupdType0F1R {
        TdfiPhyupdType0F1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tPHYUPD_TYPE0 timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 0. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
set in the UPDATE_ERROR_STATUS parameter. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phyupd_type0_f1(
        &mut self,
    ) -> TdfiPhyupdType0F1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl395Spec> {
        TdfiPhyupdType0F1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_395\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_395::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_395::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl395Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl395Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_395::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl395Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_395::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl395Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_395 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl395Spec {
    const RESET_VALUE: u32 = 0;
}
