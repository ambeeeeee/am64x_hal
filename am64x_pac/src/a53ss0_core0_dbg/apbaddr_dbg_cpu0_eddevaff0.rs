#[doc = "Register `APBADDR_DBG_CPU0_EDDEVAFF0` reader"]
pub type R = crate::R<ApbaddrDbgCpu0Eddevaff0Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_EDDEVAFF0` writer"]
pub type W = crate::W<ApbaddrDbgCpu0Eddevaff0Spec>;
#[doc = "Field `EDDEVAFF0` reader - 31:0\\]
MPIDR_EL1 low half. Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type Eddevaff0R = crate::FieldReader<u32>;
#[doc = "Field `EDDEVAFF0` writer - 31:0\\]
MPIDR_EL1 low half. Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
pub type Eddevaff0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MPIDR_EL1 low half. Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    pub fn eddevaff0(&self) -> Eddevaff0R {
        Eddevaff0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MPIDR_EL1 low half. Read-only copy of the low half of MPIDR_EL1, as seen from the highest implemented exception level."]
    #[inline(always)]
    #[must_use]
    pub fn eddevaff0(&mut self) -> Eddevaff0W<ApbaddrDbgCpu0Eddevaff0Spec> {
        Eddevaff0W::new(self, 0)
    }
}
#[doc = "External Debug Device Affinity Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_eddevaff0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_eddevaff0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0Eddevaff0Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0Eddevaff0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_eddevaff0::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0Eddevaff0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_eddevaff0::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0Eddevaff0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_EDDEVAFF0 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu0Eddevaff0Spec {
    const RESET_VALUE: u32 = 0;
}
