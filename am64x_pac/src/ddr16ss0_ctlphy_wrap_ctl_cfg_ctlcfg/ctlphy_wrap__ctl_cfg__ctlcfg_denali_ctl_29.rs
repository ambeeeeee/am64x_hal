#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_29` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl29Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_29` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl29Spec>;
#[doc = "Field `DQS_OSC_PERIOD` reader - 14:0\\]
Number of cycles to run the oscillator measurement. Must reflect cycles programmed into mode register."]
pub type DqsOscPeriodR = crate::FieldReader<u16>;
#[doc = "Field `DQS_OSC_PERIOD` writer - 14:0\\]
Number of cycles to run the oscillator measurement. Must reflect cycles programmed into mode register."]
pub type DqsOscPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `FUNC_VALID_CYCLES` reader - 19:16\\]
Number of cycles to hold dfi_function_valid asserted."]
pub type FuncValidCyclesR = crate::FieldReader;
#[doc = "Field `FUNC_VALID_CYCLES` writer - 19:16\\]
Number of cycles to hold dfi_function_valid asserted."]
pub type FuncValidCyclesW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - 14:0\\]
Number of cycles to run the oscillator measurement. Must reflect cycles programmed into mode register."]
    #[inline(always)]
    pub fn dqs_osc_period(&self) -> DqsOscPeriodR {
        DqsOscPeriodR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Number of cycles to hold dfi_function_valid asserted."]
    #[inline(always)]
    pub fn func_valid_cycles(&self) -> FuncValidCyclesR {
        FuncValidCyclesR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - 14:0\\]
Number of cycles to run the oscillator measurement. Must reflect cycles programmed into mode register."]
    #[inline(always)]
    #[must_use]
    pub fn dqs_osc_period(&mut self) -> DqsOscPeriodW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl29Spec> {
        DqsOscPeriodW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Number of cycles to hold dfi_function_valid asserted."]
    #[inline(always)]
    #[must_use]
    pub fn func_valid_cycles(
        &mut self,
    ) -> FuncValidCyclesW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl29Spec> {
        FuncValidCyclesW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl29Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_29::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl29Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_29::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_29 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl29Spec {
    const RESET_VALUE: u32 = 0;
}
