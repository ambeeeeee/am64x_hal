#[doc = "Register `APBADDR_DBG_CPU1_EDWAR_63_32` reader"]
pub type R = crate::R<ApbaddrDbgCpu1Edwar63_32Spec>;
#[doc = "Register `APBADDR_DBG_CPU1_EDWAR_63_32` writer"]
pub type W = crate::W<ApbaddrDbgCpu1Edwar63_32Spec>;
#[doc = "Field `EDWAR_63_32` reader - 31:0\\]
Watchpoint address. The virtual data address being accessed when a watchpoint debug event was triggered and caused entry to Debug state.UNKNOWN if the processor is not in Debug state, or if Debug state was entered other than for a watchpoint debug event.The address must be within a naturally-aligned block of memory of power-of-two size no larger than the DC ZVA block size."]
pub type Edwar63_32R = crate::FieldReader<u32>;
#[doc = "Field `EDWAR_63_32` writer - 31:0\\]
Watchpoint address. The virtual data address being accessed when a watchpoint debug event was triggered and caused entry to Debug state.UNKNOWN if the processor is not in Debug state, or if Debug state was entered other than for a watchpoint debug event.The address must be within a naturally-aligned block of memory of power-of-two size no larger than the DC ZVA block size."]
pub type Edwar63_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Watchpoint address. The virtual data address being accessed when a watchpoint debug event was triggered and caused entry to Debug state.UNKNOWN if the processor is not in Debug state, or if Debug state was entered other than for a watchpoint debug event.The address must be within a naturally-aligned block of memory of power-of-two size no larger than the DC ZVA block size."]
    #[inline(always)]
    pub fn edwar_63_32(&self) -> Edwar63_32R {
        Edwar63_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Watchpoint address. The virtual data address being accessed when a watchpoint debug event was triggered and caused entry to Debug state.UNKNOWN if the processor is not in Debug state, or if Debug state was entered other than for a watchpoint debug event.The address must be within a naturally-aligned block of memory of power-of-two size no larger than the DC ZVA block size."]
    #[inline(always)]
    #[must_use]
    pub fn edwar_63_32(&mut self) -> Edwar63_32W<ApbaddrDbgCpu1Edwar63_32Spec> {
        Edwar63_32W::new(self, 0)
    }
}
#[doc = "External Debug Watchpoint Address Register (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edwar_63_32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edwar_63_32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1Edwar63_32Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu1Edwar63_32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_edwar_63_32::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1Edwar63_32Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_edwar_63_32::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1Edwar63_32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_EDWAR_63_32 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu1Edwar63_32Spec {
    const RESET_VALUE: u32 = 0;
}
