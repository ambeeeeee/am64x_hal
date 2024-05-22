#[doc = "Register `APBADDR_DBG_CPU1_DBGCLAIMCLR_EL1` reader"]
pub type R = crate::R<ApbaddrDbgCpu1DbgclaimclrEl1Spec>;
#[doc = "Register `APBADDR_DBG_CPU1_DBGCLAIMCLR_EL1` writer"]
pub type W = crate::W<ApbaddrDbgCpu1DbgclaimclrEl1Spec>;
#[doc = "Field `CLAIM` reader - 7:0\\]
Claim clear bits. Reading this field returns the current value of the CLAIM bits.Writing a 1 to one of these bits clears the corresponding CLAIM bit to 0. This is an indirect write to the CLAIM bits.A single write operation can clear multiple bits to 0. Writing 0 to one of these bits has no effect."]
pub type ClaimR = crate::FieldReader;
#[doc = "Field `CLAIM` writer - 7:0\\]
Claim clear bits. Reading this field returns the current value of the CLAIM bits.Writing a 1 to one of these bits clears the corresponding CLAIM bit to 0. This is an indirect write to the CLAIM bits.A single write operation can clear multiple bits to 0. Writing 0 to one of these bits has no effect."]
pub type ClaimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RES0_DBGCLAIMCLR_EL1_31_8` reader - 31:8\\]
Reserved, RAZ/SBZ. Software can rely on these bits reading as zero, and must use a should-be-zero policy on writes. Implementations must ignore writes."]
pub type Res0DbgclaimclrEl1_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_DBGCLAIMCLR_EL1_31_8` writer - 31:8\\]
Reserved, RAZ/SBZ. Software can rely on these bits reading as zero, and must use a should-be-zero policy on writes. Implementations must ignore writes."]
pub type Res0DbgclaimclrEl1_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Claim clear bits. Reading this field returns the current value of the CLAIM bits.Writing a 1 to one of these bits clears the corresponding CLAIM bit to 0. This is an indirect write to the CLAIM bits.A single write operation can clear multiple bits to 0. Writing 0 to one of these bits has no effect."]
    #[inline(always)]
    pub fn claim(&self) -> ClaimR {
        ClaimR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RAZ/SBZ. Software can rely on these bits reading as zero, and must use a should-be-zero policy on writes. Implementations must ignore writes."]
    #[inline(always)]
    pub fn res0_dbgclaimclr_el1_31_8(&self) -> Res0DbgclaimclrEl1_31_8R {
        Res0DbgclaimclrEl1_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Claim clear bits. Reading this field returns the current value of the CLAIM bits.Writing a 1 to one of these bits clears the corresponding CLAIM bit to 0. This is an indirect write to the CLAIM bits.A single write operation can clear multiple bits to 0. Writing 0 to one of these bits has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn claim(&mut self) -> ClaimW<ApbaddrDbgCpu1DbgclaimclrEl1Spec> {
        ClaimW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RAZ/SBZ. Software can rely on these bits reading as zero, and must use a should-be-zero policy on writes. Implementations must ignore writes."]
    #[inline(always)]
    #[must_use]
    pub fn res0_dbgclaimclr_el1_31_8(
        &mut self,
    ) -> Res0DbgclaimclrEl1_31_8W<ApbaddrDbgCpu1DbgclaimclrEl1Spec> {
        Res0DbgclaimclrEl1_31_8W::new(self, 8)
    }
}
#[doc = "Debug Claim Tag Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgclaimclr_el1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgclaimclr_el1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1DbgclaimclrEl1Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu1DbgclaimclrEl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_dbgclaimclr_el1::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1DbgclaimclrEl1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_dbgclaimclr_el1::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1DbgclaimclrEl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_DBGCLAIMCLR_EL1 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu1DbgclaimclrEl1Spec {
    const RESET_VALUE: u32 = 0;
}
