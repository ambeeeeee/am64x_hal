#[doc = "Register `APBADDR_DBG_CPU1_EDCIDSR` reader"]
pub type R = crate::R<ApbaddrDbgCpu1EdcidsrSpec>;
#[doc = "Register `APBADDR_DBG_CPU1_EDCIDSR` writer"]
pub type W = crate::W<ApbaddrDbgCpu1EdcidsrSpec>;
#[doc = "Field `CONTEXTIDR` reader - 31:0\\]
The sampled value of CONTEXTIDR_EL1, captured on reading the low half of EDPCSR.If EL3 is implemented and using AArch32 then CONTEXTIDR is a Banked register, and EDCIDSR samples the current Banked copy of CONTEXTIDR."]
pub type ContextidrR = crate::FieldReader<u32>;
#[doc = "Field `CONTEXTIDR` writer - 31:0\\]
The sampled value of CONTEXTIDR_EL1, captured on reading the low half of EDPCSR.If EL3 is implemented and using AArch32 then CONTEXTIDR is a Banked register, and EDCIDSR samples the current Banked copy of CONTEXTIDR."]
pub type ContextidrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The sampled value of CONTEXTIDR_EL1, captured on reading the low half of EDPCSR.If EL3 is implemented and using AArch32 then CONTEXTIDR is a Banked register, and EDCIDSR samples the current Banked copy of CONTEXTIDR."]
    #[inline(always)]
    pub fn contextidr(&self) -> ContextidrR {
        ContextidrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The sampled value of CONTEXTIDR_EL1, captured on reading the low half of EDPCSR.If EL3 is implemented and using AArch32 then CONTEXTIDR is a Banked register, and EDCIDSR samples the current Banked copy of CONTEXTIDR."]
    #[inline(always)]
    #[must_use]
    pub fn contextidr(&mut self) -> ContextidrW<ApbaddrDbgCpu1EdcidsrSpec> {
        ContextidrW::new(self, 0)
    }
}
#[doc = "External Debug Context ID Sample Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edcidsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edcidsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1EdcidsrSpec;
impl crate::RegisterSpec for ApbaddrDbgCpu1EdcidsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_edcidsr::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1EdcidsrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_edcidsr::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1EdcidsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_EDCIDSR to value 0"]
impl crate::Resettable for ApbaddrDbgCpu1EdcidsrSpec {
    const RESET_VALUE: u32 = 0;
}
