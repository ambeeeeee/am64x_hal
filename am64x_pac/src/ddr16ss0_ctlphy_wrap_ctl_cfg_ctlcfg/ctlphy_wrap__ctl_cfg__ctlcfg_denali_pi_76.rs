#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_76` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi76Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_76` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi76Spec>;
#[doc = "Field `PI_WDQLVL_OSC_EN` reader - 0:0\\]
Enable for DQS oscillator triggered write DQ training, 1 = enabled."]
pub type PiWdqlvlOscEnR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_OSC_EN` writer - 0:0\\]
Enable for DQS oscillator triggered write DQ training, 1 = enabled."]
pub type PiWdqlvlOscEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_DQS_OSC_PERIOD_EN` reader - 8:8\\]
Enable for DQS oscillator triggered periodic write DQ training, 1 = enabled."]
pub type PiDqsOscPeriodEnR = crate::BitReader;
#[doc = "Field `PI_DQS_OSC_PERIOD_EN` writer - 8:8\\]
Enable for DQS oscillator triggered periodic write DQ training, 1 = enabled."]
pub type PiDqsOscPeriodEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_PDA_EN` reader - 16:16\\]
Enable for the use of write DQ training for PDA mode, 1 = enabled."]
pub type PiWdqlvlPdaEnR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_PDA_EN` writer - 16:16\\]
Enable for the use of write DQ training for PDA mode, 1 = enabled."]
pub type PiWdqlvlPdaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_PDA_VREF_TRAIN` reader - 24:24\\]
Enable for the use of PDA to set the VREF during write DQ training, 1 = enabled."]
pub type PiWdqlvlPdaVrefTrainR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_PDA_VREF_TRAIN` writer - 24:24\\]
Enable for the use of PDA to set the VREF during write DQ training, 1 = enabled."]
pub type PiWdqlvlPdaVrefTrainW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable for DQS oscillator triggered write DQ training, 1 = enabled."]
    #[inline(always)]
    pub fn pi_wdqlvl_osc_en(&self) -> PiWdqlvlOscEnR {
        PiWdqlvlOscEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable for DQS oscillator triggered periodic write DQ training, 1 = enabled."]
    #[inline(always)]
    pub fn pi_dqs_osc_period_en(&self) -> PiDqsOscPeriodEnR {
        PiDqsOscPeriodEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable for the use of write DQ training for PDA mode, 1 = enabled."]
    #[inline(always)]
    pub fn pi_wdqlvl_pda_en(&self) -> PiWdqlvlPdaEnR {
        PiWdqlvlPdaEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable for the use of PDA to set the VREF during write DQ training, 1 = enabled."]
    #[inline(always)]
    pub fn pi_wdqlvl_pda_vref_train(&self) -> PiWdqlvlPdaVrefTrainR {
        PiWdqlvlPdaVrefTrainR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable for DQS oscillator triggered write DQ training, 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_osc_en(&mut self) -> PiWdqlvlOscEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi76Spec> {
        PiWdqlvlOscEnW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable for DQS oscillator triggered periodic write DQ training, 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dqs_osc_period_en(
        &mut self,
    ) -> PiDqsOscPeriodEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi76Spec> {
        PiDqsOscPeriodEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable for the use of write DQ training for PDA mode, 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_pda_en(&mut self) -> PiWdqlvlPdaEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi76Spec> {
        PiWdqlvlPdaEnW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable for the use of PDA to set the VREF during write DQ training, 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_pda_vref_train(
        &mut self,
    ) -> PiWdqlvlPdaVrefTrainW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi76Spec> {
        PiWdqlvlPdaVrefTrainW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_76\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_76::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_76::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi76Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi76Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_76::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi76Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_76::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi76Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_76 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi76Spec {
    const RESET_VALUE: u32 = 0;
}
