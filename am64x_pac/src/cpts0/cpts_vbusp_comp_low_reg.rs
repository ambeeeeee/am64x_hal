#[doc = "Register `CPTS_VBUSP_COMP_LOW_REG` reader"]
pub type R = crate::R<CptsVbuspCompLowRegSpec>;
#[doc = "Register `CPTS_VBUSP_COMP_LOW_REG` writer"]
pub type W = crate::W<CptsVbuspCompLowRegSpec>;
#[doc = "Field `COMP_LOW` reader - 31:0\\]
Time Stamp Generate Function Comparison Low Value"]
pub type CompLowR = crate::FieldReader<u32>;
#[doc = "Field `COMP_LOW` writer - 31:0\\]
Time Stamp Generate Function Comparison Low Value"]
pub type CompLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp Generate Function Comparison Low Value"]
    #[inline(always)]
    pub fn comp_low(&self) -> CompLowR {
        CompLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp Generate Function Comparison Low Value"]
    #[inline(always)]
    #[must_use]
    pub fn comp_low(&mut self) -> CompLowW<CptsVbuspCompLowRegSpec> {
        CompLowW::new(self, 0)
    }
}
#[doc = "Time Stamp Generate Function Comparison Low Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_comp_low_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_comp_low_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspCompLowRegSpec;
impl crate::RegisterSpec for CptsVbuspCompLowRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_comp_low_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspCompLowRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_comp_low_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspCompLowRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_COMP_LOW_REG to value 0"]
impl crate::Resettable for CptsVbuspCompLowRegSpec {
    const RESET_VALUE: u32 = 0;
}
