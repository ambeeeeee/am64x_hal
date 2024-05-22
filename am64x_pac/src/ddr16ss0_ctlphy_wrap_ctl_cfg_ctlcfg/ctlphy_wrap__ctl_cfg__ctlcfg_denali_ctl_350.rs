#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_350` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl350Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_350` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl350Spec>;
#[doc = "Field `INT_ACK_DFI` reader - 7:0\\]
Clear status of the INT_STATUS_DFI parameter. WRITE-ONLY"]
pub type IntAckDfiR = crate::FieldReader;
#[doc = "Field `INT_ACK_DFI` writer - 7:0\\]
Clear status of the INT_STATUS_DFI parameter. WRITE-ONLY"]
pub type IntAckDfiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT_ACK_FREQ` reader - 23:16\\]
Clear status of the INT_STATUS_FREQ parameter. WRITE-ONLY"]
pub type IntAckFreqR = crate::FieldReader;
#[doc = "Field `INT_ACK_FREQ` writer - 23:16\\]
Clear status of the INT_STATUS_FREQ parameter. WRITE-ONLY"]
pub type IntAckFreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT_ACK_INIT` reader - 31:24\\]
Clear status of the INT_STATUS_INIT parameter. WRITE-ONLY"]
pub type IntAckInitR = crate::FieldReader;
#[doc = "Field `INT_ACK_INIT` writer - 31:24\\]
Clear status of the INT_STATUS_INIT parameter. WRITE-ONLY"]
pub type IntAckInitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Clear status of the INT_STATUS_DFI parameter. WRITE-ONLY"]
    #[inline(always)]
    pub fn int_ack_dfi(&self) -> IntAckDfiR {
        IntAckDfiR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Clear status of the INT_STATUS_FREQ parameter. WRITE-ONLY"]
    #[inline(always)]
    pub fn int_ack_freq(&self) -> IntAckFreqR {
        IntAckFreqR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Clear status of the INT_STATUS_INIT parameter. WRITE-ONLY"]
    #[inline(always)]
    pub fn int_ack_init(&self) -> IntAckInitR {
        IntAckInitR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Clear status of the INT_STATUS_DFI parameter. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_ack_dfi(&mut self) -> IntAckDfiW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl350Spec> {
        IntAckDfiW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Clear status of the INT_STATUS_FREQ parameter. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_ack_freq(&mut self) -> IntAckFreqW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl350Spec> {
        IntAckFreqW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Clear status of the INT_STATUS_INIT parameter. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_ack_init(&mut self) -> IntAckInitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl350Spec> {
        IntAckInitW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_350\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_350::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_350::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl350Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl350Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_350::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl350Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_350::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl350Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_350 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl350Spec {
    const RESET_VALUE: u32 = 0;
}
