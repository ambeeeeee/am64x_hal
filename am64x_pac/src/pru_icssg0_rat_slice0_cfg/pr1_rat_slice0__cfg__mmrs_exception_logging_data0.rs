#[doc = "Register `PR1_RAT_SLICE0__CFG__MMRS_exception_logging_data0` reader"]
pub type R = crate::R<Pr1RatSlice0_Cfg_MmrsExceptionLoggingData0Spec>;
#[doc = "Register `PR1_RAT_SLICE0__CFG__MMRS_exception_logging_data0` writer"]
pub type W = crate::W<Pr1RatSlice0_Cfg_MmrsExceptionLoggingData0Spec>;
#[doc = "Field `ADDR_L` reader - 31:0\\]
Address lower 32 bits."]
pub type AddrLR = crate::FieldReader<u32>;
#[doc = "Field `ADDR_L` writer - 31:0\\]
Address lower 32 bits."]
pub type AddrLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Address lower 32 bits."]
    #[inline(always)]
    pub fn addr_l(&self) -> AddrLR {
        AddrLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Address lower 32 bits."]
    #[inline(always)]
    #[must_use]
    pub fn addr_l(&mut self) -> AddrLW<Pr1RatSlice0_Cfg_MmrsExceptionLoggingData0Spec> {
        AddrLW::new(self, 0)
    }
}
#[doc = "The Exception Logging Data 0 Register contains the first word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice0__cfg__mmrs_exception_logging_data0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice0__cfg__mmrs_exception_logging_data0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1RatSlice0_Cfg_MmrsExceptionLoggingData0Spec;
impl crate::RegisterSpec for Pr1RatSlice0_Cfg_MmrsExceptionLoggingData0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_rat_slice0__cfg__mmrs_exception_logging_data0::R`](R) reader structure"]
impl crate::Readable for Pr1RatSlice0_Cfg_MmrsExceptionLoggingData0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_rat_slice0__cfg__mmrs_exception_logging_data0::W`](W) writer structure"]
impl crate::Writable for Pr1RatSlice0_Cfg_MmrsExceptionLoggingData0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_RAT_SLICE0__CFG__MMRS_exception_logging_data0 to value 0"]
impl crate::Resettable for Pr1RatSlice0_Cfg_MmrsExceptionLoggingData0Spec {
    const RESET_VALUE: u32 = 0;
}
