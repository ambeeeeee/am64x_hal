#[doc = "Register `CPTS_VBUSP_PPM_HIGH_REG_` reader"]
pub type R = crate::R<CptsVbuspPpmHighReg_Spec>;
#[doc = "Register `CPTS_VBUSP_PPM_HIGH_REG_` writer"]
pub type W = crate::W<CptsVbuspPpmHighReg_Spec>;
#[doc = "Field `PPM_HIGH` reader - 9:0\\]
Time Stamp ESTF Generate Function PPM High Value"]
pub type PpmHighR = crate::FieldReader<u16>;
#[doc = "Field `PPM_HIGH` writer - 9:0\\]
Time Stamp ESTF Generate Function PPM High Value"]
pub type PpmHighW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Time Stamp ESTF Generate Function PPM High Value"]
    #[inline(always)]
    pub fn ppm_high(&self) -> PpmHighR {
        PpmHighR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Time Stamp ESTF Generate Function PPM High Value"]
    #[inline(always)]
    #[must_use]
    pub fn ppm_high(&mut self) -> PpmHighW<CptsVbuspPpmHighReg_Spec> {
        PpmHighW::new(self, 0)
    }
}
#[doc = "Time Stamp ESTF Generate Function PPM High Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ppm_high_reg_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ppm_high_reg_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspPpmHighReg_Spec;
impl crate::RegisterSpec for CptsVbuspPpmHighReg_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_ppm_high_reg_::R`](R) reader structure"]
impl crate::Readable for CptsVbuspPpmHighReg_Spec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_ppm_high_reg_::W`](W) writer structure"]
impl crate::Writable for CptsVbuspPpmHighReg_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_PPM_HIGH_REG_ to value 0"]
impl crate::Resettable for CptsVbuspPpmHighReg_Spec {
    const RESET_VALUE: u32 = 0;
}
