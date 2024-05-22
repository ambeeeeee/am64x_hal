#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_tdl_base_addr` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_cq_tdl_base_addr` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrSpec>;
#[doc = "Field `CQTDLBA_LO` reader - 31:0\\]
Task Descriptor List Base Address \\[TDLBA\\]
This register stores the LSB bits \\[bits 31:0\\]
of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * \\[Task Descrip-tor size + Transfer Descriptor size\\]
as configured by Host driver. This address shall be set on Byte1 KByte boundary.: The lower 10 bits of this register shall be set to 0 by software and shall be ignored by CQE."]
pub type CqtdlbaLoR = crate::FieldReader<u32>;
#[doc = "Field `CQTDLBA_LO` writer - 31:0\\]
Task Descriptor List Base Address \\[TDLBA\\]
This register stores the LSB bits \\[bits 31:0\\]
of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * \\[Task Descrip-tor size + Transfer Descriptor size\\]
as configured by Host driver. This address shall be set on Byte1 KByte boundary.: The lower 10 bits of this register shall be set to 0 by software and shall be ignored by CQE."]
pub type CqtdlbaLoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Task Descriptor List Base Address \\[TDLBA\\]
This register stores the LSB bits \\[bits 31:0\\]
of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * \\[Task Descrip-tor size + Transfer Descriptor size\\]
as configured by Host driver. This address shall be set on Byte1 KByte boundary.: The lower 10 bits of this register shall be set to 0 by software and shall be ignored by CQE."]
    #[inline(always)]
    pub fn cqtdlba_lo(&self) -> CqtdlbaLoR {
        CqtdlbaLoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Task Descriptor List Base Address \\[TDLBA\\]
This register stores the LSB bits \\[bits 31:0\\]
of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * \\[Task Descrip-tor size + Transfer Descriptor size\\]
as configured by Host driver. This address shall be set on Byte1 KByte boundary.: The lower 10 bits of this register shall be set to 0 by software and shall be ignored by CQE."]
    #[inline(always)]
    #[must_use]
    pub fn cqtdlba_lo(&mut self) -> CqtdlbaLoW<SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrSpec> {
        CqtdlbaLoW::new(self, 0)
    }
}
#[doc = "This register is used for configuring the lower 32 bits of the byte address of the head of the Task 312 Descriptor List in the host memory.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_cq_tdl_base_addr::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_cq_tdl_base_addr to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCqTdlBaseAddrSpec {
    const RESET_VALUE: u32 = 0;
}
