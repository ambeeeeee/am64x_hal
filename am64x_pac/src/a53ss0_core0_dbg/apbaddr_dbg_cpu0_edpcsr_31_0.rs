#[doc = "Register `APBADDR_DBG_CPU0_EDPCSR_31_0` reader"]
pub type R = crate::R<ApbaddrDbgCpu0Edpcsr31_0Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_EDPCSR_31_0` writer"]
pub type W = crate::W<ApbaddrDbgCpu0Edpcsr31_0Spec>;
#[doc = "Field `EDPCSR_31_0` reader - 31:0\\]
PC Sample low word, EDPCSRlo. Bits \\[31_0\\]
of the sampled instruction address value. Reading EDPCSRlo has the side-effect of updating EDCIDSR, EDVIDSR, and EDPCSRhi. However:If the processor is in Debug state, or Sample-based profiling is prohibited, EDPCSRlo reads as 0xFFFFFFFF and EDCIDSR, EDVIDSR, and EDPCSRhi become UNKNOWN.If the processor is in Reset state, the sampled value is unknown and EDCIDSR, EDVIDSR and EDPCSRhi become UNKNOWN.If no instruction has been retired since the processor left Reset state, Debug state, or a state where Non-invasive debug is not permitted, the sampled value is UNKNOWN and EDCIDSR, EDVIDSR, and EDPCSRhi become UNKNOWN.For a read of EDPCSRlo from the memory-mapped interface, if EDLSR.SLK == 1, meaning the Software Lock is locked, then the access has no side-effects. That is, EDCIDSR, EDVIDSR, and EDPCSRhi are unchanged."]
pub type Edpcsr31_0R = crate::FieldReader<u32>;
#[doc = "Field `EDPCSR_31_0` writer - 31:0\\]
PC Sample low word, EDPCSRlo. Bits \\[31_0\\]
of the sampled instruction address value. Reading EDPCSRlo has the side-effect of updating EDCIDSR, EDVIDSR, and EDPCSRhi. However:If the processor is in Debug state, or Sample-based profiling is prohibited, EDPCSRlo reads as 0xFFFFFFFF and EDCIDSR, EDVIDSR, and EDPCSRhi become UNKNOWN.If the processor is in Reset state, the sampled value is unknown and EDCIDSR, EDVIDSR and EDPCSRhi become UNKNOWN.If no instruction has been retired since the processor left Reset state, Debug state, or a state where Non-invasive debug is not permitted, the sampled value is UNKNOWN and EDCIDSR, EDVIDSR, and EDPCSRhi become UNKNOWN.For a read of EDPCSRlo from the memory-mapped interface, if EDLSR.SLK == 1, meaning the Software Lock is locked, then the access has no side-effects. That is, EDCIDSR, EDVIDSR, and EDPCSRhi are unchanged."]
pub type Edpcsr31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
PC Sample low word, EDPCSRlo. Bits \\[31_0\\]
of the sampled instruction address value. Reading EDPCSRlo has the side-effect of updating EDCIDSR, EDVIDSR, and EDPCSRhi. However:If the processor is in Debug state, or Sample-based profiling is prohibited, EDPCSRlo reads as 0xFFFFFFFF and EDCIDSR, EDVIDSR, and EDPCSRhi become UNKNOWN.If the processor is in Reset state, the sampled value is unknown and EDCIDSR, EDVIDSR and EDPCSRhi become UNKNOWN.If no instruction has been retired since the processor left Reset state, Debug state, or a state where Non-invasive debug is not permitted, the sampled value is UNKNOWN and EDCIDSR, EDVIDSR, and EDPCSRhi become UNKNOWN.For a read of EDPCSRlo from the memory-mapped interface, if EDLSR.SLK == 1, meaning the Software Lock is locked, then the access has no side-effects. That is, EDCIDSR, EDVIDSR, and EDPCSRhi are unchanged."]
    #[inline(always)]
    pub fn edpcsr_31_0(&self) -> Edpcsr31_0R {
        Edpcsr31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
PC Sample low word, EDPCSRlo. Bits \\[31_0\\]
of the sampled instruction address value. Reading EDPCSRlo has the side-effect of updating EDCIDSR, EDVIDSR, and EDPCSRhi. However:If the processor is in Debug state, or Sample-based profiling is prohibited, EDPCSRlo reads as 0xFFFFFFFF and EDCIDSR, EDVIDSR, and EDPCSRhi become UNKNOWN.If the processor is in Reset state, the sampled value is unknown and EDCIDSR, EDVIDSR and EDPCSRhi become UNKNOWN.If no instruction has been retired since the processor left Reset state, Debug state, or a state where Non-invasive debug is not permitted, the sampled value is UNKNOWN and EDCIDSR, EDVIDSR, and EDPCSRhi become UNKNOWN.For a read of EDPCSRlo from the memory-mapped interface, if EDLSR.SLK == 1, meaning the Software Lock is locked, then the access has no side-effects. That is, EDCIDSR, EDVIDSR, and EDPCSRhi are unchanged."]
    #[inline(always)]
    #[must_use]
    pub fn edpcsr_31_0(&mut self) -> Edpcsr31_0W<ApbaddrDbgCpu0Edpcsr31_0Spec> {
        Edpcsr31_0W::new(self, 0)
    }
}
#[doc = "External Debug Program Counter Sample Register (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edpcsr_31_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edpcsr_31_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0Edpcsr31_0Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0Edpcsr31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_edpcsr_31_0::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0Edpcsr31_0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_edpcsr_31_0::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0Edpcsr31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_EDPCSR_31_0 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu0Edpcsr31_0Spec {
    const RESET_VALUE: u32 = 0;
}
