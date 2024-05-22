#[doc = "Register `CPTS_VBUSP_INTSTAT_RAW_REG` reader"]
pub type R = crate::R<CptsVbuspIntstatRawRegSpec>;
#[doc = "Register `CPTS_VBUSP_INTSTAT_RAW_REG` writer"]
pub type W = crate::W<CptsVbuspIntstatRawRegSpec>;
#[doc = "Field `TS_PEND_RAW` reader - 0:0\\]
TS_PEND_RAW int read (before enable)"]
pub type TsPendRawR = crate::BitReader;
#[doc = "Field `TS_PEND_RAW` writer - 0:0\\]
TS_PEND_RAW int read (before enable)"]
pub type TsPendRawW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TS_PEND_RAW int read (before enable)"]
    #[inline(always)]
    pub fn ts_pend_raw(&self) -> TsPendRawR {
        TsPendRawR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TS_PEND_RAW int read (before enable)"]
    #[inline(always)]
    #[must_use]
    pub fn ts_pend_raw(&mut self) -> TsPendRawW<CptsVbuspIntstatRawRegSpec> {
        TsPendRawW::new(self, 0)
    }
}
#[doc = "Interrupt Status Register Raw\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_intstat_raw_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_intstat_raw_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspIntstatRawRegSpec;
impl crate::RegisterSpec for CptsVbuspIntstatRawRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_intstat_raw_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspIntstatRawRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_intstat_raw_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspIntstatRawRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_INTSTAT_RAW_REG to value 0"]
impl crate::Resettable for CptsVbuspIntstatRawRegSpec {
    const RESET_VALUE: u32 = 0;
}
