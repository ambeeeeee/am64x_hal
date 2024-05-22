#[doc = "Register `CPSW_NUSS_VBUSP_intr_vector_reg_out_pulse` reader"]
pub type R = crate::R<CpswNussVbuspIntrVectorRegOutPulseSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_intr_vector_reg_out_pulse` writer"]
pub type W = crate::W<CpswNussVbuspIntrVectorRegOutPulseSpec>;
#[doc = "Field `INTR_VECTOR_OUT_PULSE` reader - 31:0\\]
Interrupt Vector"]
pub type IntrVectorOutPulseR = crate::FieldReader<u32>;
#[doc = "Field `INTR_VECTOR_OUT_PULSE` writer - 31:0\\]
Interrupt Vector"]
pub type IntrVectorOutPulseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Interrupt Vector"]
    #[inline(always)]
    pub fn intr_vector_out_pulse(&self) -> IntrVectorOutPulseR {
        IntrVectorOutPulseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Interrupt Vector"]
    #[inline(always)]
    #[must_use]
    pub fn intr_vector_out_pulse(
        &mut self,
    ) -> IntrVectorOutPulseW<CpswNussVbuspIntrVectorRegOutPulseSpec> {
        IntrVectorOutPulseW::new(self, 0)
    }
}
#[doc = "Interrupt Vector for out_pulse\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_intr_vector_reg_out_pulse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_intr_vector_reg_out_pulse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspIntrVectorRegOutPulseSpec;
impl crate::RegisterSpec for CpswNussVbuspIntrVectorRegOutPulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_intr_vector_reg_out_pulse::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspIntrVectorRegOutPulseSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_intr_vector_reg_out_pulse::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspIntrVectorRegOutPulseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_intr_vector_reg_out_pulse to value 0"]
impl crate::Resettable for CpswNussVbuspIntrVectorRegOutPulseSpec {
    const RESET_VALUE: u32 = 0;
}
