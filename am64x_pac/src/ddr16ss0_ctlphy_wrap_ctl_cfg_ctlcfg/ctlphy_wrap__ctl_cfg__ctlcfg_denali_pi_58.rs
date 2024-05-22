#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_58` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi58Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_58` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi58Spec>;
#[doc = "Field `PI_TDFI_CALVL_MAX` reader - 31:0\\]
Defines the DFI tCALVL_MAX timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_calvl_en assertion and a valid dfi_calvl_resp."]
pub type PiTdfiCalvlMaxR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_CALVL_MAX` writer - 31:0\\]
Defines the DFI tCALVL_MAX timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_calvl_en assertion and a valid dfi_calvl_resp."]
pub type PiTdfiCalvlMaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tCALVL_MAX timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_calvl_en assertion and a valid dfi_calvl_resp."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_max(&self) -> PiTdfiCalvlMaxR {
        PiTdfiCalvlMaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Defines the DFI tCALVL_MAX timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_calvl_en assertion and a valid dfi_calvl_resp."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_max(&mut self) -> PiTdfiCalvlMaxW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi58Spec> {
        PiTdfiCalvlMaxW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_58::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_58::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi58Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_58::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi58Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_58::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi58Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_58 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi58Spec {
    const RESET_VALUE: u32 = 0;
}
