#[doc = "Register `APBADDR_DBG_CPU0_EDDEVID2` reader"]
pub type R = crate::R<ApbaddrDbgCpu0Eddevid2Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_EDDEVID2` writer"]
pub type W = crate::W<ApbaddrDbgCpu0Eddevid2Spec>;
#[doc = "Field `RES0_EDDEVID2_31_0` reader - 31:0\\]
Reserved, RES0."]
pub type Res0Eddevid2_31_0R = crate::FieldReader<u32>;
#[doc = "Field `RES0_EDDEVID2_31_0` writer - 31:0\\]
Reserved, RES0."]
pub type Res0Eddevid2_31_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_eddevid2_31_0(&self) -> Res0Eddevid2_31_0R {
        Res0Eddevid2_31_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_eddevid2_31_0(&mut self) -> Res0Eddevid2_31_0W<ApbaddrDbgCpu0Eddevid2Spec> {
        Res0Eddevid2_31_0W::new(self, 0)
    }
}
#[doc = "External Debug Device ID Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_eddevid2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_eddevid2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0Eddevid2Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0Eddevid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_eddevid2::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0Eddevid2Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_eddevid2::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0Eddevid2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_EDDEVID2 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu0Eddevid2Spec {
    const RESET_VALUE: u32 = 0;
}
