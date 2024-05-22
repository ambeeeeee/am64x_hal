#[doc = "Register `CPTS_VBUSP_LENGTH_REG` reader"]
pub type R = crate::R<CptsVbuspLengthRegSpec>;
#[doc = "Register `CPTS_VBUSP_LENGTH_REG` writer"]
pub type W = crate::W<CptsVbuspLengthRegSpec>;
#[doc = "Field `LENGTH` reader - 31:0\\]
Time Stamp Generate Function Length Value"]
pub type LengthR = crate::FieldReader<u32>;
#[doc = "Field `LENGTH` writer - 31:0\\]
Time Stamp Generate Function Length Value"]
pub type LengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp Generate Function Length Value"]
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp Generate Function Length Value"]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LengthW<CptsVbuspLengthRegSpec> {
        LengthW::new(self, 0)
    }
}
#[doc = "Time Stamp Generate Function Length Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_length_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_length_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspLengthRegSpec;
impl crate::RegisterSpec for CptsVbuspLengthRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_length_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspLengthRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_length_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspLengthRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_LENGTH_REG to value 0"]
impl crate::Resettable for CptsVbuspLengthRegSpec {
    const RESET_VALUE: u32 = 0;
}
