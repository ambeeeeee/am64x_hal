#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_34` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl34Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_34` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl34Spec>;
#[doc = "Field `OSC_VARIANCE_LIMIT` reader - 15:0\\]
Allowed difference between base value and DQS Oscillator measurement."]
pub type OscVarianceLimitR = crate::FieldReader<u16>;
#[doc = "Field `OSC_VARIANCE_LIMIT` writer - 15:0\\]
Allowed difference between base value and DQS Oscillator measurement."]
pub type OscVarianceLimitW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DQS_OSC_REQUEST` reader - 16:16\\]
Software request for DQS Oscillator measurement function in DRAM. WRITE-ONLY"]
pub type DqsOscRequestR = crate::BitReader;
#[doc = "Field `DQS_OSC_REQUEST` writer - 16:16\\]
Software request for DQS Oscillator measurement function in DRAM. WRITE-ONLY"]
pub type DqsOscRequestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOSCO_F0` reader - 31:24\\]
Number of cycles for tOSCO timing parameter, the time for the DQS Oscillator measurement to be available in the mode registers. FC=0"]
pub type ToscoF0R = crate::FieldReader;
#[doc = "Field `TOSCO_F0` writer - 31:24\\]
Number of cycles for tOSCO timing parameter, the time for the DQS Oscillator measurement to be available in the mode registers. FC=0"]
pub type ToscoF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Allowed difference between base value and DQS Oscillator measurement."]
    #[inline(always)]
    pub fn osc_variance_limit(&self) -> OscVarianceLimitR {
        OscVarianceLimitR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Software request for DQS Oscillator measurement function in DRAM. WRITE-ONLY"]
    #[inline(always)]
    pub fn dqs_osc_request(&self) -> DqsOscRequestR {
        DqsOscRequestR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Number of cycles for tOSCO timing parameter, the time for the DQS Oscillator measurement to be available in the mode registers. FC=0"]
    #[inline(always)]
    pub fn tosco_f0(&self) -> ToscoF0R {
        ToscoF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Allowed difference between base value and DQS Oscillator measurement."]
    #[inline(always)]
    #[must_use]
    pub fn osc_variance_limit(
        &mut self,
    ) -> OscVarianceLimitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl34Spec> {
        OscVarianceLimitW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Software request for DQS Oscillator measurement function in DRAM. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn dqs_osc_request(&mut self) -> DqsOscRequestW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl34Spec> {
        DqsOscRequestW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Number of cycles for tOSCO timing parameter, the time for the DQS Oscillator measurement to be available in the mode registers. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tosco_f0(&mut self) -> ToscoF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl34Spec> {
        ToscoF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl34Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_34::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl34Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_34::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_34 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl34Spec {
    const RESET_VALUE: u32 = 0;
}
