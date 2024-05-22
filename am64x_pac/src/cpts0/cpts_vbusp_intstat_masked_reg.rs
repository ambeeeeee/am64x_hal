#[doc = "Register `CPTS_VBUSP_INTSTAT_MASKED_REG` reader"]
pub type R = crate::R<CptsVbuspIntstatMaskedRegSpec>;
#[doc = "Register `CPTS_VBUSP_INTSTAT_MASKED_REG` writer"]
pub type W = crate::W<CptsVbuspIntstatMaskedRegSpec>;
#[doc = "Field `TS_PEND` reader - 0:0\\]
TS_PEND masked interrupt read (after enable)"]
pub type TsPendR = crate::BitReader;
#[doc = "Field `TS_PEND` writer - 0:0\\]
TS_PEND masked interrupt read (after enable)"]
pub type TsPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TS_PEND masked interrupt read (after enable)"]
    #[inline(always)]
    pub fn ts_pend(&self) -> TsPendR {
        TsPendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TS_PEND masked interrupt read (after enable)"]
    #[inline(always)]
    #[must_use]
    pub fn ts_pend(&mut self) -> TsPendW<CptsVbuspIntstatMaskedRegSpec> {
        TsPendW::new(self, 0)
    }
}
#[doc = "Interrupt Status Register Masked\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_intstat_masked_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_intstat_masked_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspIntstatMaskedRegSpec;
impl crate::RegisterSpec for CptsVbuspIntstatMaskedRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_intstat_masked_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspIntstatMaskedRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_intstat_masked_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspIntstatMaskedRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_INTSTAT_MASKED_REG to value 0"]
impl crate::Resettable for CptsVbuspIntstatMaskedRegSpec {
    const RESET_VALUE: u32 = 0;
}
