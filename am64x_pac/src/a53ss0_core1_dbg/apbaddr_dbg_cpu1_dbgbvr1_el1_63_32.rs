#[doc = "Register `APBADDR_DBG_CPU1_DBGBVR1_EL1_63_32` reader"]
pub type R = crate::R<ApbaddrDbgCpu1Dbgbvr1El1_63_32Spec>;
#[doc = "Register `APBADDR_DBG_CPU1_DBGBVR1_EL1_63_32` writer"]
pub type W = crate::W<ApbaddrDbgCpu1Dbgbvr1El1_63_32Spec>;
#[doc = "Field `DBGBVR1_EL1_63_32` reader - 31:0\\]
Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching"]
pub type Dbgbvr1El1_63_32R = crate::FieldReader<u32>;
#[doc = "Field `DBGBVR1_EL1_63_32` writer - 31:0\\]
Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching"]
pub type Dbgbvr1El1_63_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching"]
    #[inline(always)]
    pub fn dbgbvr1_el1_63_32(&self) -> Dbgbvr1El1_63_32R {
        Dbgbvr1El1_63_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching"]
    #[inline(always)]
    #[must_use]
    pub fn dbgbvr1_el1_63_32(&mut self) -> Dbgbvr1El1_63_32W<ApbaddrDbgCpu1Dbgbvr1El1_63_32Spec> {
        Dbgbvr1El1_63_32W::new(self, 0)
    }
}
#[doc = "Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR1_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbvr1_el1_63_32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbvr1_el1_63_32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1Dbgbvr1El1_63_32Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu1Dbgbvr1El1_63_32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_dbgbvr1_el1_63_32::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1Dbgbvr1El1_63_32Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_dbgbvr1_el1_63_32::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1Dbgbvr1El1_63_32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_DBGBVR1_EL1_63_32 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu1Dbgbvr1El1_63_32Spec {
    const RESET_VALUE: u32 = 0;
}
