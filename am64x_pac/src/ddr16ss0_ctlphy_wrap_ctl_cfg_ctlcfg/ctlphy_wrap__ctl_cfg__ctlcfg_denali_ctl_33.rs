#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_33` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl33Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_33` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl33Spec>;
#[doc = "Field `DQS_OSC_PROMOTE_THRESHOLD` reader - 31:0\\]
Number of long counts until a software request for the DQS Oscillator is promoted to high priority."]
pub type DqsOscPromoteThresholdR = crate::FieldReader<u32>;
#[doc = "Field `DQS_OSC_PROMOTE_THRESHOLD` writer - 31:0\\]
Number of long counts until a software request for the DQS Oscillator is promoted to high priority."]
pub type DqsOscPromoteThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Number of long counts until a software request for the DQS Oscillator is promoted to high priority."]
    #[inline(always)]
    pub fn dqs_osc_promote_threshold(&self) -> DqsOscPromoteThresholdR {
        DqsOscPromoteThresholdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Number of long counts until a software request for the DQS Oscillator is promoted to high priority."]
    #[inline(always)]
    #[must_use]
    pub fn dqs_osc_promote_threshold(
        &mut self,
    ) -> DqsOscPromoteThresholdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl33Spec> {
        DqsOscPromoteThresholdW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl33Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_33::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl33Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_33::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_33 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl33Spec {
    const RESET_VALUE: u32 = 0;
}
