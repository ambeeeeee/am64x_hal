#[doc = "Register `CPTS_VBUSP_TS_COMP_LOW_VAL_REG` reader"]
pub type R = crate::R<CptsVbuspTsCompLowValRegSpec>;
#[doc = "Register `CPTS_VBUSP_TS_COMP_LOW_VAL_REG` writer"]
pub type W = crate::W<CptsVbuspTsCompLowValRegSpec>;
#[doc = "Field `TS_COMP_VAL` reader - 31:0\\]
Time stamp comparison low value"]
pub type TsCompValR = crate::FieldReader<u32>;
#[doc = "Field `TS_COMP_VAL` writer - 31:0\\]
Time stamp comparison low value"]
pub type TsCompValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp comparison low value"]
    #[inline(always)]
    pub fn ts_comp_val(&self) -> TsCompValR {
        TsCompValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp comparison low value"]
    #[inline(always)]
    #[must_use]
    pub fn ts_comp_val(&mut self) -> TsCompValW<CptsVbuspTsCompLowValRegSpec> {
        TsCompValW::new(self, 0)
    }
}
#[doc = "Time Stamp Comparison Low Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_comp_low_val_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_comp_low_val_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspTsCompLowValRegSpec;
impl crate::RegisterSpec for CptsVbuspTsCompLowValRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_ts_comp_low_val_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspTsCompLowValRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_ts_comp_low_val_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspTsCompLowValRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_TS_COMP_LOW_VAL_REG to value 0"]
impl crate::Resettable for CptsVbuspTsCompLowValRegSpec {
    const RESET_VALUE: u32 = 0;
}
