#[doc = "Register `RAT__CFG__MMRS_exception_logging_header1` reader"]
pub type R = crate::R<Rat_Cfg_MmrsExceptionLoggingHeader1Spec>;
#[doc = "Register `RAT__CFG__MMRS_exception_logging_header1` writer"]
pub type W = crate::W<Rat_Cfg_MmrsExceptionLoggingHeader1Spec>;
#[doc = "Field `CODE` reader - 23:16\\]
Code. 1 = Boundary crossing error."]
pub type CodeR = crate::FieldReader;
#[doc = "Field `CODE` writer - 23:16\\]
Code. 1 = Boundary crossing error."]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GROUP` reader - 31:24\\]
Group."]
pub type GroupR = crate::FieldReader;
#[doc = "Field `GROUP` writer - 31:24\\]
Group."]
pub type GroupW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - 23:16\\]
Code. 1 = Boundary crossing error."]
    #[inline(always)]
    pub fn code(&self) -> CodeR {
        CodeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Group."]
    #[inline(always)]
    pub fn group(&self) -> GroupR {
        GroupR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - 23:16\\]
Code. 1 = Boundary crossing error."]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CodeW<Rat_Cfg_MmrsExceptionLoggingHeader1Spec> {
        CodeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Group."]
    #[inline(always)]
    #[must_use]
    pub fn group(&mut self) -> GroupW<Rat_Cfg_MmrsExceptionLoggingHeader1Spec> {
        GroupW::new(self, 24)
    }
}
#[doc = "The Exception Logging Header 1 Register contains the second word of the header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_logging_header1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_logging_header1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rat_Cfg_MmrsExceptionLoggingHeader1Spec;
impl crate::RegisterSpec for Rat_Cfg_MmrsExceptionLoggingHeader1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rat__cfg__mmrs_exception_logging_header1::R`](R) reader structure"]
impl crate::Readable for Rat_Cfg_MmrsExceptionLoggingHeader1Spec {}
#[doc = "`write(|w| ..)` method takes [`rat__cfg__mmrs_exception_logging_header1::W`](W) writer structure"]
impl crate::Writable for Rat_Cfg_MmrsExceptionLoggingHeader1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAT__CFG__MMRS_exception_logging_header1 to value 0"]
impl crate::Resettable for Rat_Cfg_MmrsExceptionLoggingHeader1Spec {
    const RESET_VALUE: u32 = 0;
}
