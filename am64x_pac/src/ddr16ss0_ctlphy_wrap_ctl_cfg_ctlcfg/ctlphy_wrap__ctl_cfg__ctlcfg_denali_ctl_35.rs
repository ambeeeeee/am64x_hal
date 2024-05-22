#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_35` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl35Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_35` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl35Spec>;
#[doc = "Field `TOSCO_F1` reader - 7:0\\]
Number of cycles for tOSCO timing parameter, the time for the DQS Oscillator measurement to be available in the mode registers. FC=1"]
pub type ToscoF1R = crate::FieldReader;
#[doc = "Field `TOSCO_F1` writer - 7:0\\]
Number of cycles for tOSCO timing parameter, the time for the DQS Oscillator measurement to be available in the mode registers. FC=1"]
pub type ToscoF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TOSCO_F2` reader - 15:8\\]
Number of cycles for tOSCO timing parameter, the time for the DQS Oscillator measurement to be available in the mode registers. FC=2"]
pub type ToscoF2R = crate::FieldReader;
#[doc = "Field `TOSCO_F2` writer - 15:8\\]
Number of cycles for tOSCO timing parameter, the time for the DQS Oscillator measurement to be available in the mode registers. FC=2"]
pub type ToscoF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DQS_OSC_BASE_VALUE_0_CS0` reader - 31:16\\]
Base value for device 0 on chip 0. READ-ONLY DEV=0"]
pub type DqsOscBaseValue0Cs0R = crate::FieldReader<u16>;
#[doc = "Field `DQS_OSC_BASE_VALUE_0_CS0` writer - 31:16\\]
Base value for device 0 on chip 0. READ-ONLY DEV=0"]
pub type DqsOscBaseValue0Cs0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Number of cycles for tOSCO timing parameter, the time for the DQS Oscillator measurement to be available in the mode registers. FC=1"]
    #[inline(always)]
    pub fn tosco_f1(&self) -> ToscoF1R {
        ToscoF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of cycles for tOSCO timing parameter, the time for the DQS Oscillator measurement to be available in the mode registers. FC=2"]
    #[inline(always)]
    pub fn tosco_f2(&self) -> ToscoF2R {
        ToscoF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Base value for device 0 on chip 0. READ-ONLY DEV=0"]
    #[inline(always)]
    pub fn dqs_osc_base_value_0_cs0(&self) -> DqsOscBaseValue0Cs0R {
        DqsOscBaseValue0Cs0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Number of cycles for tOSCO timing parameter, the time for the DQS Oscillator measurement to be available in the mode registers. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tosco_f1(&mut self) -> ToscoF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl35Spec> {
        ToscoF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of cycles for tOSCO timing parameter, the time for the DQS Oscillator measurement to be available in the mode registers. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tosco_f2(&mut self) -> ToscoF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl35Spec> {
        ToscoF2W::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Base value for device 0 on chip 0. READ-ONLY DEV=0"]
    #[inline(always)]
    #[must_use]
    pub fn dqs_osc_base_value_0_cs0(
        &mut self,
    ) -> DqsOscBaseValue0Cs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl35Spec> {
        DqsOscBaseValue0Cs0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl35Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_35::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl35Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_35::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl35Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_35 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl35Spec {
    const RESET_VALUE: u32 = 0;
}
