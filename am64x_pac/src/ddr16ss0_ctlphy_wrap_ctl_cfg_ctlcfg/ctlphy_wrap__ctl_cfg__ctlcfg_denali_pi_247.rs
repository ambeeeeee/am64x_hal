#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_247` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi247Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_247` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi247Spec>;
#[doc = "Field `PI_TDFI_CTRLUPD_MAX_F0` reader - 20:0\\]
Defines the DFI tCTRLUPD_MAX timing parameter \\[in DFI clocks\\]
for frequency set 0, the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
set in the PI_UPDATE_ERROR_STATUS parameter."]
pub type PiTdfiCtrlupdMaxF0R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_CTRLUPD_MAX_F0` writer - 20:0\\]
Defines the DFI tCTRLUPD_MAX timing parameter \\[in DFI clocks\\]
for frequency set 0, the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
set in the PI_UPDATE_ERROR_STATUS parameter."]
pub type PiTdfiCtrlupdMaxF0W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
Defines the DFI tCTRLUPD_MAX timing parameter \\[in DFI clocks\\]
for frequency set 0, the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
set in the PI_UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn pi_tdfi_ctrlupd_max_f0(&self) -> PiTdfiCtrlupdMaxF0R {
        PiTdfiCtrlupdMaxF0R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - 20:0\\]
Defines the DFI tCTRLUPD_MAX timing parameter \\[in DFI clocks\\]
for frequency set 0, the maximum cycles that dfi_ctrlupd_req can be asserted. If programmed to a non-zero, a timing violation will cause an interrupt and bit \\[1\\]
set in the PI_UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrlupd_max_f0(
        &mut self,
    ) -> PiTdfiCtrlupdMaxF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi247Spec> {
        PiTdfiCtrlupdMaxF0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_247\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_247::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_247::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi247Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi247Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_247::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi247Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_247::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi247Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_247 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi247Spec {
    const RESET_VALUE: u32 = 0;
}
