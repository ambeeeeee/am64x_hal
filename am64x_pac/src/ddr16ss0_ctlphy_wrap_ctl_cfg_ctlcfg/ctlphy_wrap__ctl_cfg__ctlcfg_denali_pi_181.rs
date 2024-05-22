#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_181` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi181Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_181` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi181Spec>;
#[doc = "Field `PI_TDFI_CTRL_DELAY_F1` reader - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\]
for frequency set 1, the delay between a DFI command change and a memory command."]
pub type PiTdfiCtrlDelayF1R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CTRL_DELAY_F1` writer - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\]
for frequency set 1, the delay between a DFI command change and a memory command."]
pub type PiTdfiCtrlDelayF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TDFI_CTRL_DELAY_F2` reader - 11:8\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\]
for frequency set 2, the delay between a DFI command change and a memory command."]
pub type PiTdfiCtrlDelayF2R = crate::FieldReader;
#[doc = "Field `PI_TDFI_CTRL_DELAY_F2` writer - 11:8\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\]
for frequency set 2, the delay between a DFI command change and a memory command."]
pub type PiTdfiCtrlDelayF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_WRLVL_EN_F0` reader - 17:16\\]
Enable the PI write leveling module for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiWrlvlEnF0R = crate::FieldReader;
#[doc = "Field `PI_WRLVL_EN_F0` writer - 17:16\\]
Enable the PI write leveling module for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiWrlvlEnF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WRLVL_EN_F1` reader - 25:24\\]
Enable the PI write leveling module for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiWrlvlEnF1R = crate::FieldReader;
#[doc = "Field `PI_WRLVL_EN_F1` writer - 25:24\\]
Enable the PI write leveling module for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiWrlvlEnF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\]
for frequency set 1, the delay between a DFI command change and a memory command."]
    #[inline(always)]
    pub fn pi_tdfi_ctrl_delay_f1(&self) -> PiTdfiCtrlDelayF1R {
        PiTdfiCtrlDelayF1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\]
for frequency set 2, the delay between a DFI command change and a memory command."]
    #[inline(always)]
    pub fn pi_tdfi_ctrl_delay_f2(&self) -> PiTdfiCtrlDelayF2R {
        PiTdfiCtrlDelayF2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable the PI write leveling module for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_wrlvl_en_f0(&self) -> PiWrlvlEnF0R {
        PiWrlvlEnF0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Enable the PI write leveling module for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_wrlvl_en_f1(&self) -> PiWrlvlEnF1R {
        PiWrlvlEnF1R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\]
for frequency set 1, the delay between a DFI command change and a memory command."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrl_delay_f1(
        &mut self,
    ) -> PiTdfiCtrlDelayF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi181Spec> {
        PiTdfiCtrlDelayF1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\]
for frequency set 2, the delay between a DFI command change and a memory command."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_ctrl_delay_f2(
        &mut self,
    ) -> PiTdfiCtrlDelayF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi181Spec> {
        PiTdfiCtrlDelayF2W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable the PI write leveling module for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_en_f0(&mut self) -> PiWrlvlEnF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi181Spec> {
        PiWrlvlEnF0W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Enable the PI write leveling module for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_en_f1(&mut self) -> PiWrlvlEnF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi181Spec> {
        PiWrlvlEnF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_181\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_181::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_181::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi181Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi181Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_181::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi181Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_181::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi181Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_181 to value 0x0202"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi181Spec {
    const RESET_VALUE: u32 = 0x0202;
}
