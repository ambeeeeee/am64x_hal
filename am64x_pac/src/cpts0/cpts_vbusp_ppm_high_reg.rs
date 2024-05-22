#[doc = "Register `CPTS_VBUSP_PPM_HIGH_REG` reader"]
pub type R = crate::R<CptsVbuspPpmHighRegSpec>;
#[doc = "Register `CPTS_VBUSP_PPM_HIGH_REG` writer"]
pub type W = crate::W<CptsVbuspPpmHighRegSpec>;
#[doc = "Field `PPM_HIGH` reader - 9:0\\]
Time Stamp Generate Function PPM High Value"]
pub type PpmHighR = crate::FieldReader<u16>;
#[doc = "Field `PPM_HIGH` writer - 9:0\\]
Time Stamp Generate Function PPM High Value"]
pub type PpmHighW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Time Stamp Generate Function PPM High Value"]
    #[inline(always)]
    pub fn ppm_high(&self) -> PpmHighR {
        PpmHighR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Time Stamp Generate Function PPM High Value"]
    #[inline(always)]
    #[must_use]
    pub fn ppm_high(&mut self) -> PpmHighW<CptsVbuspPpmHighRegSpec> {
        PpmHighW::new(self, 0)
    }
}
#[doc = "Time Stamp Generate Function PPM High Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ppm_high_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ppm_high_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspPpmHighRegSpec;
impl crate::RegisterSpec for CptsVbuspPpmHighRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_ppm_high_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspPpmHighRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_ppm_high_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspPpmHighRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_PPM_HIGH_REG to value 0"]
impl crate::Resettable for CptsVbuspPpmHighRegSpec {
    const RESET_VALUE: u32 = 0;
}
