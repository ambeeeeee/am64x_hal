#[doc = "Register `APBADDR_DBG_CPU0_DBGWVR0_EL1_31_0` reader"]
pub type R = crate::R<ApbaddrDbgCpu0Dbgwvr0El1_31_0Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_DBGWVR0_EL1_31_0` writer"]
pub type W = crate::W<ApbaddrDbgCpu0Dbgwvr0El1_31_0Spec>;
#[doc = "Field `RES0_DBGWVR0_EL1_31_0_1_0` reader - 1:0\\]
Reserved, RES0."]
pub type Res0Dbgwvr0El1_31_0_1_0R = crate::FieldReader;
#[doc = "Field `RES0_DBGWVR0_EL1_31_0_1_0` writer - 1:0\\]
Reserved, RES0."]
pub type Res0Dbgwvr0El1_31_0_1_0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VA` reader - 31:2\\]
Bits\\[48:2\\]
of the address value for comparison.ARM deprecates setting DBGWVR&lt;n>_EL1\\[2\\]
== 1."]
pub type VaR = crate::FieldReader<u32>;
#[doc = "Field `VA` writer - 31:2\\]
Bits\\[48:2\\]
of the address value for comparison.ARM deprecates setting DBGWVR&lt;n>_EL1\\[2\\]
== 1."]
pub type VaW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_dbgwvr0_el1_31_0_1_0(&self) -> Res0Dbgwvr0El1_31_0_1_0R {
        Res0Dbgwvr0El1_31_0_1_0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Bits\\[48:2\\]
of the address value for comparison.ARM deprecates setting DBGWVR&lt;n>_EL1\\[2\\]
== 1."]
    #[inline(always)]
    pub fn va(&self) -> VaR {
        VaR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_dbgwvr0_el1_31_0_1_0(
        &mut self,
    ) -> Res0Dbgwvr0El1_31_0_1_0W<ApbaddrDbgCpu0Dbgwvr0El1_31_0Spec> {
        Res0Dbgwvr0El1_31_0_1_0W::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Bits\\[48:2\\]
of the address value for comparison.ARM deprecates setting DBGWVR&lt;n>_EL1\\[2\\]
== 1."]
    #[inline(always)]
    #[must_use]
    pub fn va(&mut self) -> VaW<ApbaddrDbgCpu0Dbgwvr0El1_31_0Spec> {
        VaW::new(self, 2)
    }
}
#[doc = "Debug Watchpoint Value Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwvr0_el1_31_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwvr0_el1_31_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0Dbgwvr0El1_31_0Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0Dbgwvr0El1_31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_dbgwvr0_el1_31_0::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0Dbgwvr0El1_31_0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_dbgwvr0_el1_31_0::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0Dbgwvr0El1_31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_DBGWVR0_EL1_31_0 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu0Dbgwvr0El1_31_0Spec {
    const RESET_VALUE: u32 = 0;
}
