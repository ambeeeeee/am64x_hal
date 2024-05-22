#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_342` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl342Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_342` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl342Spec>;
#[doc = "Field `INT_STATUS_DFI` reader - 7:0\\]
Status of interrupts in the controller related to DFI. READ-ONLY"]
pub type IntStatusDfiR = crate::FieldReader;
#[doc = "Field `INT_STATUS_DFI` writer - 7:0\\]
Status of interrupts in the controller related to DFI. READ-ONLY"]
pub type IntStatusDfiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT_STATUS_FREQ` reader - 23:16\\]
Status of interrupts in the controller related to Frequency Scaling. READ-ONLY"]
pub type IntStatusFreqR = crate::FieldReader;
#[doc = "Field `INT_STATUS_FREQ` writer - 23:16\\]
Status of interrupts in the controller related to Frequency Scaling. READ-ONLY"]
pub type IntStatusFreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT_STATUS_INIT` reader - 31:24\\]
Status of interrupts in the controller related to Initialization. READ-ONLY"]
pub type IntStatusInitR = crate::FieldReader;
#[doc = "Field `INT_STATUS_INIT` writer - 31:24\\]
Status of interrupts in the controller related to Initialization. READ-ONLY"]
pub type IntStatusInitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Status of interrupts in the controller related to DFI. READ-ONLY"]
    #[inline(always)]
    pub fn int_status_dfi(&self) -> IntStatusDfiR {
        IntStatusDfiR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Status of interrupts in the controller related to Frequency Scaling. READ-ONLY"]
    #[inline(always)]
    pub fn int_status_freq(&self) -> IntStatusFreqR {
        IntStatusFreqR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Status of interrupts in the controller related to Initialization. READ-ONLY"]
    #[inline(always)]
    pub fn int_status_init(&self) -> IntStatusInitR {
        IntStatusInitR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Status of interrupts in the controller related to DFI. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_dfi(&mut self) -> IntStatusDfiW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl342Spec> {
        IntStatusDfiW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Status of interrupts in the controller related to Frequency Scaling. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_freq(&mut self) -> IntStatusFreqW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl342Spec> {
        IntStatusFreqW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Status of interrupts in the controller related to Initialization. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_init(&mut self) -> IntStatusInitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl342Spec> {
        IntStatusInitW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_342\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_342::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_342::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl342Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl342Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_342::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl342Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_342::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl342Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_342 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl342Spec {
    const RESET_VALUE: u32 = 0;
}
