#[doc = "Register `CPTS_VBUSP_TS_PPM_HIGH_VAL_REG` reader"]
pub type R = crate::R<CptsVbuspTsPpmHighValRegSpec>;
#[doc = "Register `CPTS_VBUSP_TS_PPM_HIGH_VAL_REG` writer"]
pub type W = crate::W<CptsVbuspTsPpmHighValRegSpec>;
#[doc = "Field `TS_PPM_HIGH_VAL` reader - 9:0\\]
Time stamp PPM High value"]
pub type TsPpmHighValR = crate::FieldReader<u16>;
#[doc = "Field `TS_PPM_HIGH_VAL` writer - 9:0\\]
Time stamp PPM High value"]
pub type TsPpmHighValW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Time stamp PPM High value"]
    #[inline(always)]
    pub fn ts_ppm_high_val(&self) -> TsPpmHighValR {
        TsPpmHighValR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Time stamp PPM High value"]
    #[inline(always)]
    #[must_use]
    pub fn ts_ppm_high_val(&mut self) -> TsPpmHighValW<CptsVbuspTsPpmHighValRegSpec> {
        TsPpmHighValW::new(self, 0)
    }
}
#[doc = "Time Stamp PPM High Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_ppm_high_val_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_ppm_high_val_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspTsPpmHighValRegSpec;
impl crate::RegisterSpec for CptsVbuspTsPpmHighValRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_ts_ppm_high_val_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspTsPpmHighValRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_ts_ppm_high_val_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspTsPpmHighValRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_TS_PPM_HIGH_VAL_REG to value 0"]
impl crate::Resettable for CptsVbuspTsPpmHighValRegSpec {
    const RESET_VALUE: u32 = 0;
}
