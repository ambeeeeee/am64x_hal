#[doc = "Register `RAT__CFG__MMRS_exception_logging_header0` reader"]
pub type R = crate::R<Rat_Cfg_MmrsExceptionLoggingHeader0Spec>;
#[doc = "Register `RAT__CFG__MMRS_exception_logging_header0` writer"]
pub type W = crate::W<Rat_Cfg_MmrsExceptionLoggingHeader0Spec>;
#[doc = "Field `DEST_ID` reader - 7:0\\]
Destination ID."]
pub type DestIdR = crate::FieldReader;
#[doc = "Field `DEST_ID` writer - 7:0\\]
Destination ID."]
pub type DestIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SRC_ID` reader - 23:8\\]
Source ID."]
pub type SrcIdR = crate::FieldReader<u16>;
#[doc = "Field `SRC_ID` writer - 23:8\\]
Source ID."]
pub type SrcIdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TYPE_F` reader - 31:24\\]
Type. 4 = RAT."]
pub type TypeFR = crate::FieldReader;
#[doc = "Field `TYPE_F` writer - 31:24\\]
Type. 4 = RAT."]
pub type TypeFW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Destination ID."]
    #[inline(always)]
    pub fn dest_id(&self) -> DestIdR {
        DestIdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Source ID."]
    #[inline(always)]
    pub fn src_id(&self) -> SrcIdR {
        SrcIdR::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Type. 4 = RAT."]
    #[inline(always)]
    pub fn type_f(&self) -> TypeFR {
        TypeFR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Destination ID."]
    #[inline(always)]
    #[must_use]
    pub fn dest_id(&mut self) -> DestIdW<Rat_Cfg_MmrsExceptionLoggingHeader0Spec> {
        DestIdW::new(self, 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Source ID."]
    #[inline(always)]
    #[must_use]
    pub fn src_id(&mut self) -> SrcIdW<Rat_Cfg_MmrsExceptionLoggingHeader0Spec> {
        SrcIdW::new(self, 8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Type. 4 = RAT."]
    #[inline(always)]
    #[must_use]
    pub fn type_f(&mut self) -> TypeFW<Rat_Cfg_MmrsExceptionLoggingHeader0Spec> {
        TypeFW::new(self, 24)
    }
}
#[doc = "The Exception Logging Header 0 Register contains the first word of the header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_logging_header0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_logging_header0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rat_Cfg_MmrsExceptionLoggingHeader0Spec;
impl crate::RegisterSpec for Rat_Cfg_MmrsExceptionLoggingHeader0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rat__cfg__mmrs_exception_logging_header0::R`](R) reader structure"]
impl crate::Readable for Rat_Cfg_MmrsExceptionLoggingHeader0Spec {}
#[doc = "`write(|w| ..)` method takes [`rat__cfg__mmrs_exception_logging_header0::W`](W) writer structure"]
impl crate::Writable for Rat_Cfg_MmrsExceptionLoggingHeader0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAT__CFG__MMRS_exception_logging_header0 to value 0"]
impl crate::Resettable for Rat_Cfg_MmrsExceptionLoggingHeader0Spec {
    const RESET_VALUE: u32 = 0;
}
