#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_251` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi251Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_251` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi251Spec>;
#[doc = "Field `PI_TDFI_CTRLUPD_MAX_F2` reader - 20:0\\]
Defines the DFI tCTRLUPD_MAX timing parameter \\[in DFI clocks\\]
for frequency set 2, the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
set in the PI_UPDATE_ERROR_STATUS parameter."]
pub type PiTdfiCtrlupdMaxF2R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_CTRLUPD_MAX_F2` writer - 20:0\\]
Defines the DFI tCTRLUPD_MAX timing parameter \\[in DFI clocks\\]
for frequency set 2, the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
set in the PI_UPDATE_ERROR_STATUS parameter."]
pub type PiTdfiCtrlupdMaxF2W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
Defines the DFI tCTRLUPD_MAX timing parameter \\[in DFI clocks\\]
for frequency set 2, the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
set in the PI_UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn pi_tdfi_ctrlupd_max_f2(&self) -> PiTdfiCtrlupdMaxF2R {
        PiTdfiCtrlupdMaxF2R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - 20:0\\]
Defines the DFI tCTRLUPD_MAX timing parameter \\[in DFI clocks\\]
for frequency set 2, the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
set in the PI_UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrlupd_max_f2(
        &mut self,
    ) -> PiTdfiCtrlupdMaxF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi251Spec> {
        PiTdfiCtrlupdMaxF2W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_251\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_251::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_251::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi251Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi251Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_251::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi251Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_251::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi251Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_251 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi251Spec {
    const RESET_VALUE: u32 = 0;
}
