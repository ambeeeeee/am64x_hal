#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_send_sts_config2` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig2Spec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_send_sts_config2` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig2Spec>;
#[doc = "Field `QUEUE_RCA` reader - 15:0\\]
This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_ STATUS \\[CMD13\\]
com-mand. argument. CQE shall copy this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS \\[CMD13\\]
command."]
pub type QueueRcaR = crate::FieldReader<u16>;
#[doc = "Field `QUEUE_RCA` writer - 15:0\\]
This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_ STATUS \\[CMD13\\]
com-mand. argument. CQE shall copy this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS \\[CMD13\\]
command."]
pub type QueueRcaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_ STATUS \\[CMD13\\]
com-mand. argument. CQE shall copy this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS \\[CMD13\\]
command."]
    #[inline(always)]
    pub fn queue_rca(&self) -> QueueRcaR {
        QueueRcaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_ STATUS \\[CMD13\\]
com-mand. argument. CQE shall copy this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS \\[CMD13\\]
command."]
    #[inline(always)]
    #[must_use]
    pub fn queue_rca(&mut self) -> QueueRcaW<SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig2Spec> {
        QueueRcaW::new(self, 0)
    }
}
#[doc = "This register is used for 333 configuring RCA field in SEND_QUEUE_STATUS command argu-ment.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig2Spec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config2::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig2Spec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_send_sts_config2::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_send_sts_config2 to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqSendStsConfig2Spec {
    const RESET_VALUE: u32 = 0;
}
