#[doc = "Register `PR1_RAT_SLICE0__CFG__MMRS_exception_enable_clear` reader"]
pub type R = crate::R<Pr1RatSlice0_Cfg_MmrsExceptionEnableClearSpec>;
#[doc = "Register `PR1_RAT_SLICE0__CFG__MMRS_exception_enable_clear` writer"]
pub type W = crate::W<Pr1RatSlice0_Cfg_MmrsExceptionEnableClearSpec>;
#[doc = "Field `ENABLE_CLR` reader - 0:0\\]
Write a 1 to clear the exception interrupt enable signal."]
pub type EnableClrR = crate::BitReader;
#[doc = "Field `ENABLE_CLR` writer - 0:0\\]
Write a 1 to clear the exception interrupt enable signal."]
pub type EnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write a 1 to clear the exception interrupt enable signal."]
    #[inline(always)]
    pub fn enable_clr(&self) -> EnableClrR {
        EnableClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write a 1 to clear the exception interrupt enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn enable_clr(&mut self) -> EnableClrW<Pr1RatSlice0_Cfg_MmrsExceptionEnableClearSpec> {
        EnableClrW::new(self, 0)
    }
}
#[doc = "The Exception Logging Interrupt Enable Clear Register allows to clear the interrupt enable signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice0__cfg__mmrs_exception_enable_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice0__cfg__mmrs_exception_enable_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1RatSlice0_Cfg_MmrsExceptionEnableClearSpec;
impl crate::RegisterSpec for Pr1RatSlice0_Cfg_MmrsExceptionEnableClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_rat_slice0__cfg__mmrs_exception_enable_clear::R`](R) reader structure"]
impl crate::Readable for Pr1RatSlice0_Cfg_MmrsExceptionEnableClearSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_rat_slice0__cfg__mmrs_exception_enable_clear::W`](W) writer structure"]
impl crate::Writable for Pr1RatSlice0_Cfg_MmrsExceptionEnableClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_RAT_SLICE0__CFG__MMRS_exception_enable_clear to value 0"]
impl crate::Resettable for Pr1RatSlice0_Cfg_MmrsExceptionEnableClearSpec {
    const RESET_VALUE: u32 = 0;
}
