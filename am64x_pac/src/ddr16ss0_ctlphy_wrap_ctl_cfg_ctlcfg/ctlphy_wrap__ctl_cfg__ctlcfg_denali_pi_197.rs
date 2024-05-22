#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_197` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi197Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_197` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi197Spec>;
#[doc = "Field `PI_TDFI_CALVL_CC_F1` reader - 9:0\\]
Defines the DFI tCALVL_CC timing parameter \\[in DFI clocks\\]
for frequency set 1, the minimum cycles between calibration commands."]
pub type PiTdfiCalvlCcF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_CALVL_CC_F1` writer - 9:0\\]
Defines the DFI tCALVL_CC timing parameter \\[in DFI clocks\\]
for frequency set 1, the minimum cycles between calibration commands."]
pub type PiTdfiCalvlCcF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TDFI_CALVL_CAPTURE_F1` reader - 25:16\\]
Defines the DFI tCALVL_CAPTURE timing parameter \\[in DFI clocks\\]
for frequency set 1, the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
pub type PiTdfiCalvlCaptureF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_CALVL_CAPTURE_F1` writer - 25:16\\]
Defines the DFI tCALVL_CAPTURE timing parameter \\[in DFI clocks\\]
for frequency set 1, the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
pub type PiTdfiCalvlCaptureF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Defines the DFI tCALVL_CC timing parameter \\[in DFI clocks\\]
for frequency set 1, the minimum cycles between calibration commands."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_cc_f1(&self) -> PiTdfiCalvlCcF1R {
        PiTdfiCalvlCcF1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Defines the DFI tCALVL_CAPTURE timing parameter \\[in DFI clocks\\]
for frequency set 1, the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_capture_f1(&self) -> PiTdfiCalvlCaptureF1R {
        PiTdfiCalvlCaptureF1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Defines the DFI tCALVL_CC timing parameter \\[in DFI clocks\\]
for frequency set 1, the minimum cycles between calibration commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_cc_f1(
        &mut self,
    ) -> PiTdfiCalvlCcF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi197Spec> {
        PiTdfiCalvlCcF1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Defines the DFI tCALVL_CAPTURE timing parameter \\[in DFI clocks\\]
for frequency set 1, the minimum cycles between a calibration command and a dfi_calvl_capture pulse."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_capture_f1(
        &mut self,
    ) -> PiTdfiCalvlCaptureF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi197Spec> {
        PiTdfiCalvlCaptureF1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_197\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_197::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_197::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi197Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi197Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_197::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi197Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_197::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi197Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_197 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi197Spec {
    const RESET_VALUE: u32 = 0;
}
