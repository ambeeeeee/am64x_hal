#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_resp_err_mask` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqRespErrMaskSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_resp_err_mask` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqRespErrMaskSpec>;
#[doc = "Field `CQRMEM` reader - 31:0\\]
This bit is used as in interrupt mask on the device status filed which is received in R1/R1b responses.Bit Value Description \\[for any bit i\\]:1 = When a R1/R1b response is received, with bit i in the device status set, a RED interrupt is generated 0 = When a R1/R1b response is received, bit i in the device status is ignored The reset value of this register is set to trigger an inter-rupt on all Error type bits in the device status, as defined in Section 6.13.NOTE: Responses to CMD13 \\[SQS\\]
encode the QSR,so they are ignored by this logic"]
pub type CqrmemR = crate::FieldReader<u32>;
#[doc = "Field `CQRMEM` writer - 31:0\\]
This bit is used as in interrupt mask on the device status filed which is received in R1/R1b responses.Bit Value Description \\[for any bit i\\]:1 = When a R1/R1b response is received, with bit i in the device status set, a RED interrupt is generated 0 = When a R1/R1b response is received, bit i in the device status is ignored The reset value of this register is set to trigger an inter-rupt on all Error type bits in the device status, as defined in Section 6.13.NOTE: Responses to CMD13 \\[SQS\\]
encode the QSR,so they are ignored by this logic"]
pub type CqrmemW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This bit is used as in interrupt mask on the device status filed which is received in R1/R1b responses.Bit Value Description \\[for any bit i\\]:1 = When a R1/R1b response is received, with bit i in the device status set, a RED interrupt is generated 0 = When a R1/R1b response is received, bit i in the device status is ignored The reset value of this register is set to trigger an inter-rupt on all Error type bits in the device status, as defined in Section 6.13.NOTE: Responses to CMD13 \\[SQS\\]
encode the QSR,so they are ignored by this logic"]
    #[inline(always)]
    pub fn cqrmem(&self) -> CqrmemR {
        CqrmemR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This bit is used as in interrupt mask on the device status filed which is received in R1/R1b responses.Bit Value Description \\[for any bit i\\]:1 = When a R1/R1b response is received, with bit i in the device status set, a RED interrupt is generated 0 = When a R1/R1b response is received, bit i in the device status is ignored The reset value of this register is set to trigger an inter-rupt on all Error type bits in the device status, as defined in Section 6.13.NOTE: Responses to CMD13 \\[SQS\\]
encode the QSR,so they are ignored by this logic"]
    #[inline(always)]
    #[must_use]
    pub fn cqrmem(&mut self) -> CqrmemW<SdhcWrap_CtlCfg_CtlcfgCqRespErrMaskSpec> {
        CqrmemW::new(self, 0)
    }
}
#[doc = "This register controls the generation of Response Error Detection (RED) interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_resp_err_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_resp_err_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqRespErrMaskSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqRespErrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_resp_err_mask::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqRespErrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_resp_err_mask::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqRespErrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_resp_err_mask to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqRespErrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
