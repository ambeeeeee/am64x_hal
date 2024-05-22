#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_351` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl351Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_351` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl351Spec>;
#[doc = "Field `INT_ACK_MODE` reader - 7:0\\]
Clear status of the INT_STATUS_MODE parameter. WRITE-ONLY"]
pub type IntAckModeR = crate::FieldReader;
#[doc = "Field `INT_ACK_MODE` writer - 7:0\\]
Clear status of the INT_STATUS_MODE parameter. WRITE-ONLY"]
pub type IntAckModeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT_ACK_PARITY` reader - 15:8\\]
Clear status of the INT_STATUS_PARITY parameter. WRITE-ONLY"]
pub type IntAckParityR = crate::FieldReader;
#[doc = "Field `INT_ACK_PARITY` writer - 15:8\\]
Clear status of the INT_STATUS_PARITY parameter. WRITE-ONLY"]
pub type IntAckParityW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Clear status of the INT_STATUS_MODE parameter. WRITE-ONLY"]
    #[inline(always)]
    pub fn int_ack_mode(&self) -> IntAckModeR {
        IntAckModeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Clear status of the INT_STATUS_PARITY parameter. WRITE-ONLY"]
    #[inline(always)]
    pub fn int_ack_parity(&self) -> IntAckParityR {
        IntAckParityR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Clear status of the INT_STATUS_MODE parameter. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_ack_mode(&mut self) -> IntAckModeW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl351Spec> {
        IntAckModeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Clear status of the INT_STATUS_PARITY parameter. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn int_ack_parity(&mut self) -> IntAckParityW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl351Spec> {
        IntAckParityW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_351\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_351::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_351::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl351Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl351Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_351::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl351Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_351::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl351Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_351 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl351Spec {
    const RESET_VALUE: u32 = 0;
}
