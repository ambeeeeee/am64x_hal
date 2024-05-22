#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_tdl_base_addr_upbits` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrUpbitsSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_tdl_base_addr_upbits` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrUpbitsSpec>;
#[doc = "Field `CQTDLBA_HI` reader - 31:0\\]
Task Descriptor List Base Address \\[TDLBA\\]
This register stores the MSB bits \\[bits 63:32\\]
of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * \\[Task Descrip-tor size + Transfer Descriptor size\\]
as configured by Host driver. This register is reserved when using 32-bit addressing mode."]
pub type CqtdlbaHiR = crate::FieldReader<u32>;
#[doc = "Field `CQTDLBA_HI` writer - 31:0\\]
Task Descriptor List Base Address \\[TDLBA\\]
This register stores the MSB bits \\[bits 63:32\\]
of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * \\[Task Descrip-tor size + Transfer Descriptor size\\]
as configured by Host driver. This register is reserved when using 32-bit addressing mode."]
pub type CqtdlbaHiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Task Descriptor List Base Address \\[TDLBA\\]
This register stores the MSB bits \\[bits 63:32\\]
of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * \\[Task Descrip-tor size + Transfer Descriptor size\\]
as configured by Host driver. This register is reserved when using 32-bit addressing mode."]
    #[inline(always)]
    pub fn cqtdlba_hi(&self) -> CqtdlbaHiR {
        CqtdlbaHiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Task Descriptor List Base Address \\[TDLBA\\]
This register stores the MSB bits \\[bits 63:32\\]
of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * \\[Task Descrip-tor size + Transfer Descriptor size\\]
as configured by Host driver. This register is reserved when using 32-bit addressing mode."]
    #[inline(always)]
    #[must_use]
    pub fn cqtdlba_hi(&mut self) -> CqtdlbaHiW<SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrUpbitsSpec> {
        CqtdlbaHiW::new(self, 0)
    }
}
#[doc = "This register is used for configuring the upper 32 bits of the byte address of the head of the Task 316 Descriptor List in the host memory.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr_upbits::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr_upbits::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrUpbitsSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrUpbitsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr_upbits::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrUpbitsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr_upbits::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrUpbitsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_tdl_base_addr_upbits to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrUpbitsSpec {
    const RESET_VALUE: u32 = 0;
}
