#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_61` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi61Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_61` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi61Spec>;
#[doc = "Field `PI_CA_TRAIN_VREF_EN` reader - 0:0\\]
Control for VREF training during CA training post power-on initialization. Set to enable VREF training."]
pub type PiCaTrainVrefEnR = crate::BitReader;
#[doc = "Field `PI_CA_TRAIN_VREF_EN` writer - 0:0\\]
Control for VREF training during CA training post power-on initialization. Set to enable VREF training."]
pub type PiCaTrainVrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CALVL_VREF_INITIAL_STEPSIZE` reader - 11:8\\]
The adjust step for the initial Vref\\[ca\\]
training."]
pub type PiCalvlVrefInitialStepsizeR = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_INITIAL_STEPSIZE` writer - 11:8\\]
The adjust step for the initial Vref\\[ca\\]
training."]
pub type PiCalvlVrefInitialStepsizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_CALVL_VREF_NORMAL_STEPSIZE` reader - 19:16\\]
The adjust step for the post-initial Vref\\[ca\\]
training."]
pub type PiCalvlVrefNormalStepsizeR = crate::FieldReader;
#[doc = "Field `PI_CALVL_VREF_NORMAL_STEPSIZE` writer - 19:16\\]
The adjust step for the post-initial Vref\\[ca\\]
training."]
pub type PiCalvlVrefNormalStepsizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TDFI_INIT_START_MIN` reader - 31:24\\]
Minimum number of DFI clocks before dfi_init_start can be driven after a previous command/training event."]
pub type PiTdfiInitStartMinR = crate::FieldReader;
#[doc = "Field `PI_TDFI_INIT_START_MIN` writer - 31:24\\]
Minimum number of DFI clocks before dfi_init_start can be driven after a previous command/training event."]
pub type PiTdfiInitStartMinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Control for VREF training during CA training post power-on initialization. Set to enable VREF training."]
    #[inline(always)]
    pub fn pi_ca_train_vref_en(&self) -> PiCaTrainVrefEnR {
        PiCaTrainVrefEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
The adjust step for the initial Vref\\[ca\\]
training."]
    #[inline(always)]
    pub fn pi_calvl_vref_initial_stepsize(&self) -> PiCalvlVrefInitialStepsizeR {
        PiCalvlVrefInitialStepsizeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The adjust step for the post-initial Vref\\[ca\\]
training."]
    #[inline(always)]
    pub fn pi_calvl_vref_normal_stepsize(&self) -> PiCalvlVrefNormalStepsizeR {
        PiCalvlVrefNormalStepsizeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Minimum number of DFI clocks before dfi_init_start can be driven after a previous command/training event."]
    #[inline(always)]
    pub fn pi_tdfi_init_start_min(&self) -> PiTdfiInitStartMinR {
        PiTdfiInitStartMinR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Control for VREF training during CA training post power-on initialization. Set to enable VREF training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_ca_train_vref_en(
        &mut self,
    ) -> PiCaTrainVrefEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi61Spec> {
        PiCaTrainVrefEnW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
The adjust step for the initial Vref\\[ca\\]
training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_initial_stepsize(
        &mut self,
    ) -> PiCalvlVrefInitialStepsizeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi61Spec> {
        PiCalvlVrefInitialStepsizeW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The adjust step for the post-initial Vref\\[ca\\]
training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_vref_normal_stepsize(
        &mut self,
    ) -> PiCalvlVrefNormalStepsizeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi61Spec> {
        PiCalvlVrefNormalStepsizeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Minimum number of DFI clocks before dfi_init_start can be driven after a previous command/training event."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_init_start_min(
        &mut self,
    ) -> PiTdfiInitStartMinW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi61Spec> {
        PiTdfiInitStartMinW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_61::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_61::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi61Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi61Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_61::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi61Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_61::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi61Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_61 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi61Spec {
    const RESET_VALUE: u32 = 0;
}
