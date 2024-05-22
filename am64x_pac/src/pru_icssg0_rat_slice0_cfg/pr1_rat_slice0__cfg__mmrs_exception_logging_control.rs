#[doc = "Register `PR1_RAT_SLICE0__CFG__MMRS_exception_logging_control` reader"]
pub type R = crate::R<Pr1RatSlice0_Cfg_MmrsExceptionLoggingControlSpec>;
#[doc = "Register `PR1_RAT_SLICE0__CFG__MMRS_exception_logging_control` writer"]
pub type W = crate::W<Pr1RatSlice0_Cfg_MmrsExceptionLoggingControlSpec>;
#[doc = "Field `DISABLE_F` reader - 0:0\\]
Disables logging when set."]
pub type DisableFR = crate::BitReader;
#[doc = "Field `DISABLE_F` writer - 0:0\\]
Disables logging when set."]
pub type DisableFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_INTR` reader - 1:1\\]
Disables logging interrupt when set."]
pub type DisableIntrR = crate::BitReader;
#[doc = "Field `DISABLE_INTR` writer - 1:1\\]
Disables logging interrupt when set."]
pub type DisableIntrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disables logging when set."]
    #[inline(always)]
    pub fn disable_f(&self) -> DisableFR {
        DisableFR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disables logging interrupt when set."]
    #[inline(always)]
    pub fn disable_intr(&self) -> DisableIntrR {
        DisableIntrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Disables logging when set."]
    #[inline(always)]
    #[must_use]
    pub fn disable_f(&mut self) -> DisableFW<Pr1RatSlice0_Cfg_MmrsExceptionLoggingControlSpec> {
        DisableFW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disables logging interrupt when set."]
    #[inline(always)]
    #[must_use]
    pub fn disable_intr(
        &mut self,
    ) -> DisableIntrW<Pr1RatSlice0_Cfg_MmrsExceptionLoggingControlSpec> {
        DisableIntrW::new(self, 1)
    }
}
#[doc = "The Exception Logging Control Register controls the exception logging.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice0__cfg__mmrs_exception_logging_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice0__cfg__mmrs_exception_logging_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1RatSlice0_Cfg_MmrsExceptionLoggingControlSpec;
impl crate::RegisterSpec for Pr1RatSlice0_Cfg_MmrsExceptionLoggingControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_rat_slice0__cfg__mmrs_exception_logging_control::R`](R) reader structure"]
impl crate::Readable for Pr1RatSlice0_Cfg_MmrsExceptionLoggingControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_rat_slice0__cfg__mmrs_exception_logging_control::W`](W) writer structure"]
impl crate::Writable for Pr1RatSlice0_Cfg_MmrsExceptionLoggingControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_RAT_SLICE0__CFG__MMRS_exception_logging_control to value 0"]
impl crate::Resettable for Pr1RatSlice0_Cfg_MmrsExceptionLoggingControlSpec {
    const RESET_VALUE: u32 = 0;
}
