#[doc = "Register `CPTS_VBUSP_TS_NUDGE_VAL_REG` reader"]
pub type R = crate::R<CptsVbuspTsNudgeValRegSpec>;
#[doc = "Register `CPTS_VBUSP_TS_NUDGE_VAL_REG` writer"]
pub type W = crate::W<CptsVbuspTsNudgeValRegSpec>;
#[doc = "Field `TS_NUDGE_VAL` reader - 7:0\\]
Time stamp Nudge value"]
pub type TsNudgeValR = crate::FieldReader;
#[doc = "Field `TS_NUDGE_VAL` writer - 7:0\\]
Time stamp Nudge value"]
pub type TsNudgeValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Time stamp Nudge value"]
    #[inline(always)]
    pub fn ts_nudge_val(&self) -> TsNudgeValR {
        TsNudgeValR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Time stamp Nudge value"]
    #[inline(always)]
    #[must_use]
    pub fn ts_nudge_val(&mut self) -> TsNudgeValW<CptsVbuspTsNudgeValRegSpec> {
        TsNudgeValW::new(self, 0)
    }
}
#[doc = "Time Stamp Nudge Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_nudge_val_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_nudge_val_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspTsNudgeValRegSpec;
impl crate::RegisterSpec for CptsVbuspTsNudgeValRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_ts_nudge_val_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspTsNudgeValRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_ts_nudge_val_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspTsNudgeValRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_TS_NUDGE_VAL_REG to value 0"]
impl crate::Resettable for CptsVbuspTsNudgeValRegSpec {
    const RESET_VALUE: u32 = 0;
}
