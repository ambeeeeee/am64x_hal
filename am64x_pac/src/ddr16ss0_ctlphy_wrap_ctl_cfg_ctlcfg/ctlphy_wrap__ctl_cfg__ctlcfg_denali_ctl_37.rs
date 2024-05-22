#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_37` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl37Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_37` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl37Spec>;
#[doc = "Field `DQS_OSC_BASE_VALUE_1_CS1` reader - 15:0\\]
Base value for device 1 on chip 1. READ-ONLY DEV=1"]
pub type DqsOscBaseValue1Cs1R = crate::FieldReader<u16>;
#[doc = "Field `DQS_OSC_BASE_VALUE_1_CS1` writer - 15:0\\]
Base value for device 1 on chip 1. READ-ONLY DEV=1"]
pub type DqsOscBaseValue1Cs1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DQS_OSC_STATUS` reader - 19:16\\]
Holds the overflow and out of variance status associated with the resp. interrupts. Bit \\[0\\]
set indicates overflow of DQS oscillator,bit \\[1\\]
set indicates overflow of WCKO oscillator,bit \\[2\\]
set indicates oov of DQS oscillator,bit \\[3\\]
set indicates oov of WCKO oscillator. READ-ONLY"]
pub type DqsOscStatusR = crate::FieldReader;
#[doc = "Field `DQS_OSC_STATUS` writer - 19:16\\]
Holds the overflow and out of variance status associated with the resp. interrupts. Bit \\[0\\]
set indicates overflow of DQS oscillator,bit \\[1\\]
set indicates overflow of WCKO oscillator,bit \\[2\\]
set indicates oov of DQS oscillator,bit \\[3\\]
set indicates oov of WCKO oscillator. READ-ONLY"]
pub type DqsOscStatusW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DQS_OSC_IN_PROGRESS_STATUS` reader - 24:24\\]
DQS Oscillator is in progress.Set '1' DQS OSC is in progress READ-ONLY"]
pub type DqsOscInProgressStatusR = crate::BitReader;
#[doc = "Field `DQS_OSC_IN_PROGRESS_STATUS` writer - 24:24\\]
DQS Oscillator is in progress.Set '1' DQS OSC is in progress READ-ONLY"]
pub type DqsOscInProgressStatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Base value for device 1 on chip 1. READ-ONLY DEV=1"]
    #[inline(always)]
    pub fn dqs_osc_base_value_1_cs1(&self) -> DqsOscBaseValue1Cs1R {
        DqsOscBaseValue1Cs1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Holds the overflow and out of variance status associated with the resp. interrupts. Bit \\[0\\]
set indicates overflow of DQS oscillator,bit \\[1\\]
set indicates overflow of WCKO oscillator,bit \\[2\\]
set indicates oov of DQS oscillator,bit \\[3\\]
set indicates oov of WCKO oscillator. READ-ONLY"]
    #[inline(always)]
    pub fn dqs_osc_status(&self) -> DqsOscStatusR {
        DqsOscStatusR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
DQS Oscillator is in progress.Set '1' DQS OSC is in progress READ-ONLY"]
    #[inline(always)]
    pub fn dqs_osc_in_progress_status(&self) -> DqsOscInProgressStatusR {
        DqsOscInProgressStatusR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Base value for device 1 on chip 1. READ-ONLY DEV=1"]
    #[inline(always)]
    #[must_use]
    pub fn dqs_osc_base_value_1_cs1(
        &mut self,
    ) -> DqsOscBaseValue1Cs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl37Spec> {
        DqsOscBaseValue1Cs1W::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Holds the overflow and out of variance status associated with the resp. interrupts. Bit \\[0\\]
set indicates overflow of DQS oscillator,bit \\[1\\]
set indicates overflow of WCKO oscillator,bit \\[2\\]
set indicates oov of DQS oscillator,bit \\[3\\]
set indicates oov of WCKO oscillator. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn dqs_osc_status(&mut self) -> DqsOscStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl37Spec> {
        DqsOscStatusW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
DQS Oscillator is in progress.Set '1' DQS OSC is in progress READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn dqs_osc_in_progress_status(
        &mut self,
    ) -> DqsOscInProgressStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl37Spec> {
        DqsOscInProgressStatusW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_37::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_37::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl37Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl37Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_37::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl37Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_37::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl37Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_37 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl37Spec {
    const RESET_VALUE: u32 = 0;
}
