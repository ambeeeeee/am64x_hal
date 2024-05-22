#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_281` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi281Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_281` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi281Spec>;
#[doc = "Field `PI_WDQ_OSC_DELTA_INDEX_F0` reader - 3:0\\]
WDQ DQS delay delta index for OSC triggered periodic training for frequency set 0. If the value is n, the delay is 2^n/512 cycle."]
pub type PiWdqOscDeltaIndexF0R = crate::FieldReader;
#[doc = "Field `PI_WDQ_OSC_DELTA_INDEX_F0` writer - 3:0\\]
WDQ DQS delay delta index for OSC triggered periodic training for frequency set 0. If the value is n, the delay is 2^n/512 cycle."]
pub type PiWdqOscDeltaIndexF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_WDQ_OSC_DELTA_INDEX_F1` reader - 11:8\\]
WDQ DQS delay delta index for OSC triggered periodic training for frequency set 1. If the value is n, the delay is 2^n/512 cycle."]
pub type PiWdqOscDeltaIndexF1R = crate::FieldReader;
#[doc = "Field `PI_WDQ_OSC_DELTA_INDEX_F1` writer - 11:8\\]
WDQ DQS delay delta index for OSC triggered periodic training for frequency set 1. If the value is n, the delay is 2^n/512 cycle."]
pub type PiWdqOscDeltaIndexF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_WDQ_OSC_DELTA_INDEX_F2` reader - 19:16\\]
WDQ DQS delay delta index for OSC triggered periodic training for frequency set 2. If the value is n, the delay is 2^n/512 cycle."]
pub type PiWdqOscDeltaIndexF2R = crate::FieldReader;
#[doc = "Field `PI_WDQ_OSC_DELTA_INDEX_F2` writer - 19:16\\]
WDQ DQS delay delta index for OSC triggered periodic training for frequency set 2. If the value is n, the delay is 2^n/512 cycle."]
pub type PiWdqOscDeltaIndexF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_PREAMBLE_SUPPORT_F0` reader - 25:24\\]
bit0: Selection of one or two cycle preamble for read burst transfers. bit1: Selection of one or two cycles write burst transfers for NON-DDR5,one or multi\\[up to four\\]
cycles write burst transfers for DDR5."]
pub type PiPreambleSupportF0R = crate::FieldReader;
#[doc = "Field `PI_PREAMBLE_SUPPORT_F0` writer - 25:24\\]
bit0: Selection of one or two cycle preamble for read burst transfers. bit1: Selection of one or two cycles write burst transfers for NON-DDR5,one or multi\\[up to four\\]
cycles write burst transfers for DDR5."]
pub type PiPreambleSupportF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
WDQ DQS delay delta index for OSC triggered periodic training for frequency set 0. If the value is n, the delay is 2^n/512 cycle."]
    #[inline(always)]
    pub fn pi_wdq_osc_delta_index_f0(&self) -> PiWdqOscDeltaIndexF0R {
        PiWdqOscDeltaIndexF0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
WDQ DQS delay delta index for OSC triggered periodic training for frequency set 1. If the value is n, the delay is 2^n/512 cycle."]
    #[inline(always)]
    pub fn pi_wdq_osc_delta_index_f1(&self) -> PiWdqOscDeltaIndexF1R {
        PiWdqOscDeltaIndexF1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
WDQ DQS delay delta index for OSC triggered periodic training for frequency set 2. If the value is n, the delay is 2^n/512 cycle."]
    #[inline(always)]
    pub fn pi_wdq_osc_delta_index_f2(&self) -> PiWdqOscDeltaIndexF2R {
        PiWdqOscDeltaIndexF2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
bit0: Selection of one or two cycle preamble for read burst transfers. bit1: Selection of one or two cycles write burst transfers for NON-DDR5,one or multi\\[up to four\\]
cycles write burst transfers for DDR5."]
    #[inline(always)]
    pub fn pi_preamble_support_f0(&self) -> PiPreambleSupportF0R {
        PiPreambleSupportF0R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
WDQ DQS delay delta index for OSC triggered periodic training for frequency set 0. If the value is n, the delay is 2^n/512 cycle."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdq_osc_delta_index_f0(
        &mut self,
    ) -> PiWdqOscDeltaIndexF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi281Spec> {
        PiWdqOscDeltaIndexF0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
WDQ DQS delay delta index for OSC triggered periodic training for frequency set 1. If the value is n, the delay is 2^n/512 cycle."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdq_osc_delta_index_f1(
        &mut self,
    ) -> PiWdqOscDeltaIndexF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi281Spec> {
        PiWdqOscDeltaIndexF1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
WDQ DQS delay delta index for OSC triggered periodic training for frequency set 2. If the value is n, the delay is 2^n/512 cycle."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdq_osc_delta_index_f2(
        &mut self,
    ) -> PiWdqOscDeltaIndexF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi281Spec> {
        PiWdqOscDeltaIndexF2W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
bit0: Selection of one or two cycle preamble for read burst transfers. bit1: Selection of one or two cycles write burst transfers for NON-DDR5,one or multi\\[up to four\\]
cycles write burst transfers for DDR5."]
    #[inline(always)]
    #[must_use]
    pub fn pi_preamble_support_f0(
        &mut self,
    ) -> PiPreambleSupportF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi281Spec> {
        PiPreambleSupportF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_281\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_281::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_281::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi281Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi281Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_281::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi281Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_281::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi281Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_281 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi281Spec {
    const RESET_VALUE: u32 = 0;
}
