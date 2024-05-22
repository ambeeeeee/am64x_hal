#[doc = "Register `CPTS_VBUSP_TS_LOAD_HIGH_VAL_REG` reader"]
pub type R = crate::R<CptsVbuspTsLoadHighValRegSpec>;
#[doc = "Register `CPTS_VBUSP_TS_LOAD_HIGH_VAL_REG` writer"]
pub type W = crate::W<CptsVbuspTsLoadHighValRegSpec>;
#[doc = "Field `TS_LOAD_VAL` reader - 31:0\\]
Time stamp load high value"]
pub type TsLoadValR = crate::FieldReader<u32>;
#[doc = "Field `TS_LOAD_VAL` writer - 31:0\\]
Time stamp load high value"]
pub type TsLoadValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp load high value"]
    #[inline(always)]
    pub fn ts_load_val(&self) -> TsLoadValR {
        TsLoadValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp load high value"]
    #[inline(always)]
    #[must_use]
    pub fn ts_load_val(&mut self) -> TsLoadValW<CptsVbuspTsLoadHighValRegSpec> {
        TsLoadValW::new(self, 0)
    }
}
#[doc = "Time Stamp Load High Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_load_high_val_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_load_high_val_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspTsLoadHighValRegSpec;
impl crate::RegisterSpec for CptsVbuspTsLoadHighValRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_ts_load_high_val_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspTsLoadHighValRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_ts_load_high_val_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspTsLoadHighValRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_TS_LOAD_HIGH_VAL_REG to value 0"]
impl crate::Resettable for CptsVbuspTsLoadHighValRegSpec {
    const RESET_VALUE: u32 = 0;
}
