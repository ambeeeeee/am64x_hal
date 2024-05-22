#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_407` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl407Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_407` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl407Spec>;
#[doc = "Field `TDFI_PHYUPD_TYPE3_F2` reader - 31:0\\]
Defines the DFI tPHYUPD_TYPE3 timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[4\\]
set in the UPDATE_ERROR_STATUS parameter. FC=2"]
pub type TdfiPhyupdType3F2R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_PHYUPD_TYPE3_F2` writer - 31:0\\]
Defines the DFI tPHYUPD_TYPE3 timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[4\\]
set in the UPDATE_ERROR_STATUS parameter. FC=2"]
pub type TdfiPhyupdType3F2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tPHYUPD_TYPE3 timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[4\\]
set in the UPDATE_ERROR_STATUS parameter. FC=2"]
    #[inline(always)]
    pub fn tdfi_phyupd_type3_f2(&self) -> TdfiPhyupdType3F2R {
        TdfiPhyupdType3F2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tPHYUPD_TYPE3 timing parameter \\[in DFI clocks\\], the maximum cycles that dfi_phyupd_req can assert after dfi_phyupd_ack for dfi_phyupd_type 3. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[4\\]
set in the UPDATE_ERROR_STATUS parameter. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phyupd_type3_f2(
        &mut self,
    ) -> TdfiPhyupdType3F2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl407Spec> {
        TdfiPhyupdType3F2W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_407\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_407::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_407::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl407Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl407Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_407::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl407Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_407::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl407Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_407 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl407Spec {
    const RESET_VALUE: u32 = 0;
}
