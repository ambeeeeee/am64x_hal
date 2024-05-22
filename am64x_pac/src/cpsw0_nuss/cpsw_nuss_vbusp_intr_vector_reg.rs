#[doc = "Register `CPSW_NUSS_VBUSP_intr_vector_reg` reader"]
pub type R = crate::R<CpswNussVbuspIntrVectorRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_intr_vector_reg` writer"]
pub type W = crate::W<CpswNussVbuspIntrVectorRegSpec>;
#[doc = "Field `INTR_VECTOR` reader - 31:0\\]
Interrupt Vector Register"]
pub type IntrVectorR = crate::FieldReader<u32>;
#[doc = "Field `INTR_VECTOR` writer - 31:0\\]
Interrupt Vector Register"]
pub type IntrVectorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Interrupt Vector Register"]
    #[inline(always)]
    pub fn intr_vector(&self) -> IntrVectorR {
        IntrVectorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Interrupt Vector Register"]
    #[inline(always)]
    #[must_use]
    pub fn intr_vector(&mut self) -> IntrVectorW<CpswNussVbuspIntrVectorRegSpec> {
        IntrVectorW::new(self, 0)
    }
}
#[doc = "Interrupt Vector Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_intr_vector_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_intr_vector_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspIntrVectorRegSpec;
impl crate::RegisterSpec for CpswNussVbuspIntrVectorRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_intr_vector_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspIntrVectorRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_intr_vector_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspIntrVectorRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_intr_vector_reg to value 0"]
impl crate::Resettable for CpswNussVbuspIntrVectorRegSpec {
    const RESET_VALUE: u32 = 0;
}
