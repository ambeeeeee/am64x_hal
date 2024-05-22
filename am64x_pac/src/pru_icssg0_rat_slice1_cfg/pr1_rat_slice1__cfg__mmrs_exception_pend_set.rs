#[doc = "Register `PR1_RAT_SLICE1__CFG__MMRS_exception_pend_set` reader"]
pub type R = crate::R<Pr1RatSlice1_Cfg_MmrsExceptionPendSetSpec>;
#[doc = "Register `PR1_RAT_SLICE1__CFG__MMRS_exception_pend_set` writer"]
pub type W = crate::W<Pr1RatSlice1_Cfg_MmrsExceptionPendSetSpec>;
#[doc = "Field `PEND_SET` reader - 0:0\\]
Write a 1 to set the exception pend signal."]
pub type PendSetR = crate::BitReader;
#[doc = "Field `PEND_SET` writer - 0:0\\]
Write a 1 to set the exception pend signal."]
pub type PendSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write a 1 to set the exception pend signal."]
    #[inline(always)]
    pub fn pend_set(&self) -> PendSetR {
        PendSetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write a 1 to set the exception pend signal."]
    #[inline(always)]
    #[must_use]
    pub fn pend_set(&mut self) -> PendSetW<Pr1RatSlice1_Cfg_MmrsExceptionPendSetSpec> {
        PendSetW::new(self, 0)
    }
}
#[doc = "The Exception Logging Interrupt Pending Set Register allows to set the pend signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_exception_pend_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_exception_pend_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1RatSlice1_Cfg_MmrsExceptionPendSetSpec;
impl crate::RegisterSpec for Pr1RatSlice1_Cfg_MmrsExceptionPendSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_rat_slice1__cfg__mmrs_exception_pend_set::R`](R) reader structure"]
impl crate::Readable for Pr1RatSlice1_Cfg_MmrsExceptionPendSetSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_rat_slice1__cfg__mmrs_exception_pend_set::W`](W) writer structure"]
impl crate::Writable for Pr1RatSlice1_Cfg_MmrsExceptionPendSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_RAT_SLICE1__CFG__MMRS_exception_pend_set to value 0"]
impl crate::Resettable for Pr1RatSlice1_Cfg_MmrsExceptionPendSetSpec {
    const RESET_VALUE: u32 = 0;
}
