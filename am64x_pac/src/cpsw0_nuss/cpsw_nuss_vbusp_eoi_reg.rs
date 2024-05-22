#[doc = "Register `CPSW_NUSS_VBUSP_eoi_reg` reader"]
pub type R = crate::R<CpswNussVbuspEoiRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_eoi_reg` writer"]
pub type W = crate::W<CpswNussVbuspEoiRegSpec>;
#[doc = "Field `EOI_VECTOR` reader - 7:0\\]
End of Interrupt Vector"]
pub type EoiVectorR = crate::FieldReader;
#[doc = "Field `EOI_VECTOR` writer - 7:0\\]
End of Interrupt Vector"]
pub type EoiVectorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
End of Interrupt Vector"]
    #[inline(always)]
    pub fn eoi_vector(&self) -> EoiVectorR {
        EoiVectorR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
End of Interrupt Vector"]
    #[inline(always)]
    #[must_use]
    pub fn eoi_vector(&mut self) -> EoiVectorW<CpswNussVbuspEoiRegSpec> {
        EoiVectorW::new(self, 0)
    }
}
#[doc = "End of Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_eoi_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_eoi_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspEoiRegSpec;
impl crate::RegisterSpec for CpswNussVbuspEoiRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_eoi_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspEoiRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_eoi_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspEoiRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_eoi_reg to value 0"]
impl crate::Resettable for CpswNussVbuspEoiRegSpec {
    const RESET_VALUE: u32 = 0;
}
