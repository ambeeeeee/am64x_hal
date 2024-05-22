#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_296` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi296Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_296` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi296Spec>;
#[doc = "Field `PI_DQS_OSC_BASE_VALUE_0_0` reader - 15:0\\]
Base value for comparison of oscillator measurement for device 0 of rank 0"]
pub type PiDqsOscBaseValue0_0R = crate::FieldReader<u16>;
#[doc = "Field `PI_DQS_OSC_BASE_VALUE_0_0` writer - 15:0\\]
Base value for comparison of oscillator measurement for device 0 of rank 0"]
pub type PiDqsOscBaseValue0_0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_DQS_OSC_BASE_VALUE_0_1` reader - 31:16\\]
Base value for comparison of oscillator measurement for device 0 of rank 1"]
pub type PiDqsOscBaseValue0_1R = crate::FieldReader<u16>;
#[doc = "Field `PI_DQS_OSC_BASE_VALUE_0_1` writer - 31:16\\]
Base value for comparison of oscillator measurement for device 0 of rank 1"]
pub type PiDqsOscBaseValue0_1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Base value for comparison of oscillator measurement for device 0 of rank 0"]
    #[inline(always)]
    pub fn pi_dqs_osc_base_value_0_0(&self) -> PiDqsOscBaseValue0_0R {
        PiDqsOscBaseValue0_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Base value for comparison of oscillator measurement for device 0 of rank 1"]
    #[inline(always)]
    pub fn pi_dqs_osc_base_value_0_1(&self) -> PiDqsOscBaseValue0_1R {
        PiDqsOscBaseValue0_1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Base value for comparison of oscillator measurement for device 0 of rank 0"]
    #[inline(always)]
    #[must_use]
    pub fn pi_dqs_osc_base_value_0_0(
        &mut self,
    ) -> PiDqsOscBaseValue0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi296Spec> {
        PiDqsOscBaseValue0_0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Base value for comparison of oscillator measurement for device 0 of rank 1"]
    #[inline(always)]
    #[must_use]
    pub fn pi_dqs_osc_base_value_0_1(
        &mut self,
    ) -> PiDqsOscBaseValue0_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi296Spec> {
        PiDqsOscBaseValue0_1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_296\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_296::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_296::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi296Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi296Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_296::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi296Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_296::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi296Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_296 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi296Spec {
    const RESET_VALUE: u32 = 0;
}
