#[doc = "Register `CPTS_VBUSP_PPM_LOW_REG` reader"]
pub type R = crate::R<CptsVbuspPpmLowRegSpec>;
#[doc = "Register `CPTS_VBUSP_PPM_LOW_REG` writer"]
pub type W = crate::W<CptsVbuspPpmLowRegSpec>;
#[doc = "Field `PPM_LOW` reader - 31:0\\]
Time Stamp Generate Function PPM Low Value"]
pub type PpmLowR = crate::FieldReader<u32>;
#[doc = "Field `PPM_LOW` writer - 31:0\\]
Time Stamp Generate Function PPM Low Value"]
pub type PpmLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp Generate Function PPM Low Value"]
    #[inline(always)]
    pub fn ppm_low(&self) -> PpmLowR {
        PpmLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time Stamp Generate Function PPM Low Value"]
    #[inline(always)]
    #[must_use]
    pub fn ppm_low(&mut self) -> PpmLowW<CptsVbuspPpmLowRegSpec> {
        PpmLowW::new(self, 0)
    }
}
#[doc = "Time Stamp Generate Function PPM Low Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ppm_low_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ppm_low_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspPpmLowRegSpec;
impl crate::RegisterSpec for CptsVbuspPpmLowRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_ppm_low_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspPpmLowRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_ppm_low_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspPpmLowRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_PPM_LOW_REG to value 0"]
impl crate::Resettable for CptsVbuspPpmLowRegSpec {
    const RESET_VALUE: u32 = 0;
}
