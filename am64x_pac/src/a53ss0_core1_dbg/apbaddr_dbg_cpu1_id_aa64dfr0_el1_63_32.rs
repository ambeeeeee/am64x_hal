#[doc = "Register `APBADDR_DBG_CPU1_ID_AA64DFR0_EL1_63_32` reader"]
pub type R = crate::R<ApbaddrDbgCpu1IdAa64dfr0El1_63_32Spec>;
#[doc = "Register `APBADDR_DBG_CPU1_ID_AA64DFR0_EL1_63_32` writer"]
pub type W = crate::W<ApbaddrDbgCpu1IdAa64dfr0El1_63_32Spec>;
#[doc = "Field `RES0_ID_AA64DFR0_EL1_63_32_31_0` reader - 31:0\\]
Reserved, RES0."]
pub type Res0IdAa64dfr0El1_63_32_31_0R = crate::FieldReader<u32>;
#[doc = "Field `RES0_ID_AA64DFR0_EL1_63_32_31_0` writer - 31:0\\]
Reserved, RES0."]
pub type Res0IdAa64dfr0El1_63_32_31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_id_aa64dfr0_el1_63_32_31_0(&self) -> Res0IdAa64dfr0El1_63_32_31_0R {
        Res0IdAa64dfr0El1_63_32_31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_id_aa64dfr0_el1_63_32_31_0(
        &mut self,
    ) -> Res0IdAa64dfr0El1_63_32_31_0W<ApbaddrDbgCpu1IdAa64dfr0El1_63_32Spec> {
        Res0IdAa64dfr0El1_63_32_31_0W::new(self, 0)
    }
}
#[doc = "Debug Feature Register 0 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64dfr0_el1_63_32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64dfr0_el1_63_32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1IdAa64dfr0El1_63_32Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu1IdAa64dfr0El1_63_32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_id_aa64dfr0_el1_63_32::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1IdAa64dfr0El1_63_32Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_id_aa64dfr0_el1_63_32::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1IdAa64dfr0El1_63_32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_ID_AA64DFR0_EL1_63_32 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu1IdAa64dfr0El1_63_32Spec {
    const RESET_VALUE: u32 = 0;
}
