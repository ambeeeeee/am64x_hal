#[doc = "Register `CPTS_VBUSP_COMP_HIGH_REG_` reader"]
pub type R = crate::R<CptsVbuspCompHighReg_Spec>;
#[doc = "Register `CPTS_VBUSP_COMP_HIGH_REG_` writer"]
pub type W = crate::W<CptsVbuspCompHighReg_Spec>;
#[doc = "Field `COMP_HIGH` reader - 31:0\\]
Time Stamp ESTF Generate Function Comparison High Value"]
pub type CompHighR = crate::FieldReader<u32>;
#[doc = "Field `COMP_HIGH` writer - 31:0\\]
Time Stamp ESTF Generate Function Comparison High Value"]
pub type CompHighW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp ESTF Generate Function Comparison High Value"]
    #[inline(always)]
    pub fn comp_high(&self) -> CompHighR {
        CompHighR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp ESTF Generate Function Comparison High Value"]
    #[inline(always)]
    #[must_use]
    pub fn comp_high(&mut self) -> CompHighW<CptsVbuspCompHighReg_Spec> {
        CompHighW::new(self, 0)
    }
}
#[doc = "Time Stamp ESTF Generate Function Comparison high Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_comp_high_reg_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_comp_high_reg_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspCompHighReg_Spec;
impl crate::RegisterSpec for CptsVbuspCompHighReg_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_comp_high_reg_::R`](R) reader structure"]
impl crate::Readable for CptsVbuspCompHighReg_Spec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_comp_high_reg_::W`](W) writer structure"]
impl crate::Writable for CptsVbuspCompHighReg_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_COMP_HIGH_REG_ to value 0"]
impl crate::Resettable for CptsVbuspCompHighReg_Spec {
    const RESET_VALUE: u32 = 0;
}
