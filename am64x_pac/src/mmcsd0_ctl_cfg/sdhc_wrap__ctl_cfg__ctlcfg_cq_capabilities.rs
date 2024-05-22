#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_capabilities` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqCapabilitiesSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_capabilities` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqCapabilitiesSpec>;
#[doc = "Field `CF_VAL` reader - 9:0\\]
Internal Timer Clock Frequency Value \\[ITCFVAL\\]
TCFMUL and ITCFVAL indicate the frequency of the clock used for interrupt coalescing timer and for deter-mining the polling period when using periodic SEND_QUEUE_ STATUS \\[CMD13\\]
polling. The clock frequency is calculated as ITCFVAL*ITCFMUL. For example, to encode 19.2 MHz ITCFVAL shall be C0h \\[= 192 decimal\\]
and ITCFMUL shall be 2h \\[0.1 MHz 192 * 0.1 MHz 19.2 MHz\\]"]
pub type CfValR = crate::FieldReader<u16>;
#[doc = "Field `CF_VAL` writer - 9:0\\]
Internal Timer Clock Frequency Value \\[ITCFVAL\\]
TCFMUL and ITCFVAL indicate the frequency of the clock used for interrupt coalescing timer and for deter-mining the polling period when using periodic SEND_QUEUE_ STATUS \\[CMD13\\]
polling. The clock frequency is calculated as ITCFVAL*ITCFMUL. For example, to encode 19.2 MHz ITCFVAL shall be C0h \\[= 192 decimal\\]
and ITCFMUL shall be 2h \\[0.1 MHz 192 * 0.1 MHz 19.2 MHz\\]"]
pub type CfValW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CF_MUL` reader - 15:12\\]
Internal Timer Clock Frequency Multiplier \\[ITCFMUL\\]
ITCFMUL and ITCFVAL indicate the frequency of the clock used for interrupt coalescing timer and for deter-mining the SQS polling period. See ITCFVAL definition for details. Field Value Description: 0h = 0.001 MHz 1h = 0.01 MHz 2h = 0.1 MHz 3h = 1 MHz 4h = 10 MHz Other values are reserved"]
pub type CfMulR = crate::FieldReader;
#[doc = "Field `CF_MUL` writer - 15:12\\]
Internal Timer Clock Frequency Multiplier \\[ITCFMUL\\]
ITCFMUL and ITCFVAL indicate the frequency of the clock used for interrupt coalescing timer and for deter-mining the SQS polling period. See ITCFVAL definition for details. Field Value Description: 0h = 0.001 MHz 1h = 0.01 MHz 2h = 0.1 MHz 3h = 1 MHz 4h = 10 MHz Other values are reserved"]
pub type CfMulW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Internal Timer Clock Frequency Value \\[ITCFVAL\\]
TCFMUL and ITCFVAL indicate the frequency of the clock used for interrupt coalescing timer and for deter-mining the polling period when using periodic SEND_QUEUE_ STATUS \\[CMD13\\]
polling. The clock frequency is calculated as ITCFVAL*ITCFMUL. For example, to encode 19.2 MHz ITCFVAL shall be C0h \\[= 192 decimal\\]
and ITCFMUL shall be 2h \\[0.1 MHz 192 * 0.1 MHz 19.2 MHz\\]"]
    #[inline(always)]
    pub fn cf_val(&self) -> CfValR {
        CfValR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal Timer Clock Frequency Multiplier \\[ITCFMUL\\]
ITCFMUL and ITCFVAL indicate the frequency of the clock used for interrupt coalescing timer and for deter-mining the SQS polling period. See ITCFVAL definition for details. Field Value Description: 0h = 0.001 MHz 1h = 0.01 MHz 2h = 0.1 MHz 3h = 1 MHz 4h = 10 MHz Other values are reserved"]
    #[inline(always)]
    pub fn cf_mul(&self) -> CfMulR {
        CfMulR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Internal Timer Clock Frequency Value \\[ITCFVAL\\]
TCFMUL and ITCFVAL indicate the frequency of the clock used for interrupt coalescing timer and for deter-mining the polling period when using periodic SEND_QUEUE_ STATUS \\[CMD13\\]
polling. The clock frequency is calculated as ITCFVAL*ITCFMUL. For example, to encode 19.2 MHz ITCFVAL shall be C0h \\[= 192 decimal\\]
and ITCFMUL shall be 2h \\[0.1 MHz 192 * 0.1 MHz 19.2 MHz\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cf_val(&mut self) -> CfValW<SdhcWrap_CtlCfg_CtlcfgCqCapabilitiesSpec> {
        CfValW::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal Timer Clock Frequency Multiplier \\[ITCFMUL\\]
ITCFMUL and ITCFVAL indicate the frequency of the clock used for interrupt coalescing timer and for deter-mining the SQS polling period. See ITCFVAL definition for details. Field Value Description: 0h = 0.001 MHz 1h = 0.01 MHz 2h = 0.1 MHz 3h = 1 MHz 4h = 10 MHz Other values are reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cf_mul(&mut self) -> CfMulW<SdhcWrap_CtlCfg_CtlcfgCqCapabilitiesSpec> {
        CfMulW::new(self, 12)
    }
}
#[doc = "This register is reserved for capability indication.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_capabilities::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_capabilities::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqCapabilitiesSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqCapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_capabilities::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqCapabilitiesSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_capabilities::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqCapabilitiesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_capabilities to value 0x3200"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqCapabilitiesSpec {
    const RESET_VALUE: u32 = 0x3200;
}
