#[doc = "Register `APBADDR_DBG_CPU0_EDDEVAFF1` reader"]
pub type R = crate::R<ApbaddrDbgCpu0Eddevaff1Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_EDDEVAFF1` writer"]
pub type W = crate::W<ApbaddrDbgCpu0Eddevaff1Spec>;
#[doc = "Field `EDDEVAFF1` reader - 31:0\\]
MPIDR_EL1 high half. Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type Eddevaff1R = crate::FieldReader<u32>;
#[doc = "Field `EDDEVAFF1` writer - 31:0\\]
MPIDR_EL1 high half. Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type Eddevaff1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MPIDR_EL1 high half. Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    pub fn eddevaff1(&self) -> Eddevaff1R {
        Eddevaff1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MPIDR_EL1 high half. Read-only copy of the high half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    #[must_use]
    pub fn eddevaff1(&mut self) -> Eddevaff1W<ApbaddrDbgCpu0Eddevaff1Spec> {
        Eddevaff1W::new(self, 0)
    }
}
#[doc = "External Debug Device Affinity Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_eddevaff1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_eddevaff1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0Eddevaff1Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0Eddevaff1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_eddevaff1::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0Eddevaff1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_eddevaff1::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0Eddevaff1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_EDDEVAFF1 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu0Eddevaff1Spec {
    const RESET_VALUE: u32 = 0;
}
