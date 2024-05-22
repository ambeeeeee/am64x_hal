#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_252` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi252Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_252` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi252Spec>;
#[doc = "Field `PI_TDFI_CTRLUPD_INTERVAL_F2` reader - 31:0\\]
Defines the DFI tCTRLUPD_INTERVAL timing parameter \\[in DFI clocks\\]
for frequency set 2, the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[0\\]
set in the PI_UPDATE_ERROR_STATUS parameter."]
pub type PiTdfiCtrlupdIntervalF2R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_CTRLUPD_INTERVAL_F2` writer - 31:0\\]
Defines the DFI tCTRLUPD_INTERVAL timing parameter \\[in DFI clocks\\]
for frequency set 2, the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[0\\]
set in the PI_UPDATE_ERROR_STATUS parameter."]
pub type PiTdfiCtrlupdIntervalF2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tCTRLUPD_INTERVAL timing parameter \\[in DFI clocks\\]
for frequency set 2, the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[0\\]
set in the PI_UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn pi_tdfi_ctrlupd_interval_f2(&self) -> PiTdfiCtrlupdIntervalF2R {
        PiTdfiCtrlupdIntervalF2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tCTRLUPD_INTERVAL timing parameter \\[in DFI clocks\\]
for frequency set 2, the maximum cycles between dfi_ctrlupd_req assertions. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[0\\]
set in the PI_UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrlupd_interval_f2(
        &mut self,
    ) -> PiTdfiCtrlupdIntervalF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi252Spec> {
        PiTdfiCtrlupdIntervalF2W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_252\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_252::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_252::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi252Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi252Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_252::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi252Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_252::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi252Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_252 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi252Spec {
    const RESET_VALUE: u32 = 0;
}
