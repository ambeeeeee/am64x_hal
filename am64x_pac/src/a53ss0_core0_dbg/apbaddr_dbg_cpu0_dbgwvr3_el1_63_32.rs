#[doc = "Register `APBADDR_DBG_CPU0_DBGWVR3_EL1_63_32` reader"]
pub type R = crate::R<ApbaddrDbgCpu0Dbgwvr3El1_63_32Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_DBGWVR3_EL1_63_32` writer"]
pub type W = crate::W<ApbaddrDbgCpu0Dbgwvr3El1_63_32Spec>;
#[doc = "Field `VA` reader - 16:0\\]
Bits\\[48:2\\]
of the address value for comparison.ARM deprecates setting DBGWVR&lt;n>_EL1\\[2\\]
== 1."]
pub type VaR = crate::FieldReader<u32>;
#[doc = "Field `VA` writer - 16:0\\]
Bits\\[48:2\\]
of the address value for comparison.ARM deprecates setting DBGWVR&lt;n>_EL1\\[2\\]
== 1."]
pub type VaW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `RESS` reader - 31:17\\]
Reserved, Sign extended. Hardwired to the value of the sign bit, bit \\[48\\]. Hardware and software must treat this field as RES0 if bit\\[48\\]
is 0, and as RES1 if bit\\[48\\]
is 1."]
pub type RessR = crate::FieldReader<u16>;
#[doc = "Field `RESS` writer - 31:17\\]
Reserved, Sign extended. Hardwired to the value of the sign bit, bit \\[48\\]. Hardware and software must treat this field as RES0 if bit\\[48\\]
is 0, and as RES1 if bit\\[48\\]
is 1."]
pub type RessW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Bits\\[48:2\\]
of the address value for comparison.ARM deprecates setting DBGWVR&lt;n>_EL1\\[2\\]
== 1."]
    #[inline(always)]
    pub fn va(&self) -> VaR {
        VaR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Reserved, Sign extended. Hardwired to the value of the sign bit, bit \\[48\\]. Hardware and software must treat this field as RES0 if bit\\[48\\]
is 0, and as RES1 if bit\\[48\\]
is 1."]
    #[inline(always)]
    pub fn ress(&self) -> RessR {
        RessR::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Bits\\[48:2\\]
of the address value for comparison.ARM deprecates setting DBGWVR&lt;n>_EL1\\[2\\]
== 1."]
    #[inline(always)]
    #[must_use]
    pub fn va(&mut self) -> VaW<ApbaddrDbgCpu0Dbgwvr3El1_63_32Spec> {
        VaW::new(self, 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Reserved, Sign extended. Hardwired to the value of the sign bit, bit \\[48\\]. Hardware and software must treat this field as RES0 if bit\\[48\\]
is 0, and as RES1 if bit\\[48\\]
is 1."]
    #[inline(always)]
    #[must_use]
    pub fn ress(&mut self) -> RessW<ApbaddrDbgCpu0Dbgwvr3El1_63_32Spec> {
        RessW::new(self, 17)
    }
}
#[doc = "Debug Watchpoint Extended Value Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwvr3_el1_63_32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwvr3_el1_63_32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0Dbgwvr3El1_63_32Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0Dbgwvr3El1_63_32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_dbgwvr3_el1_63_32::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0Dbgwvr3El1_63_32Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_dbgwvr3_el1_63_32::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0Dbgwvr3El1_63_32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_DBGWVR3_EL1_63_32 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu0Dbgwvr3El1_63_32Spec {
    const RESET_VALUE: u32 = 0;
}
