#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_408` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl408Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_408` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl408Spec>;
#[doc = "Field `TDFI_PHYUPD_RESP_F2` reader - 22:0\\]
Defines the DFI tPHYUPD_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[5\\]
set in the UPDATE_ERROR_STATUS parameter. FC=2"]
pub type TdfiPhyupdRespF2R = crate::FieldReader<u32>;
#[doc = "Field `TDFI_PHYUPD_RESP_F2` writer - 22:0\\]
Defines the DFI tPHYUPD_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[5\\]
set in the UPDATE_ERROR_STATUS parameter. FC=2"]
pub type TdfiPhyupdRespF2W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - 22:0\\]
Defines the DFI tPHYUPD_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[5\\]
set in the UPDATE_ERROR_STATUS parameter. FC=2"]
    #[inline(always)]
    pub fn tdfi_phyupd_resp_f2(&self) -> TdfiPhyupdRespF2R {
        TdfiPhyupdRespF2R::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22 - 22:0\\]
Defines the DFI tPHYUPD_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[5\\]
set in the UPDATE_ERROR_STATUS parameter. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phyupd_resp_f2(
        &mut self,
    ) -> TdfiPhyupdRespF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl408Spec> {
        TdfiPhyupdRespF2W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_408\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_408::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_408::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl408Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl408Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_408::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl408Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_408::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl408Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_408 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl408Spec {
    const RESET_VALUE: u32 = 0;
}
