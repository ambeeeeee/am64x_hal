#[doc = "Register `ERR_REGS_exception_logging_data2` reader"]
pub type R = crate::R<ErrRegsExceptionLoggingData2Spec>;
#[doc = "Register `ERR_REGS_exception_logging_data2` writer"]
pub type W = crate::W<ErrRegsExceptionLoggingData2Spec>;
#[doc = "Field `PRIV_ID` reader - 7:0\\]
Priv ID."]
pub type PrivIdR = crate::FieldReader;
#[doc = "Field `PRIV_ID` writer - 7:0\\]
Priv ID."]
pub type PrivIdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SECURE` reader - 8:8\\]
Secure."]
pub type SecureR = crate::BitReader;
#[doc = "Field `SECURE` writer - 8:8\\]
Secure."]
pub type SecureW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIV` reader - 9:9\\]
Priv."]
pub type PrivR = crate::BitReader;
#[doc = "Field `PRIV` writer - 9:9\\]
Priv."]
pub type PrivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHEABLE` reader - 10:10\\]
Cacheable."]
pub type CacheableR = crate::BitReader;
#[doc = "Field `CACHEABLE` writer - 10:10\\]
Cacheable."]
pub type CacheableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUG` reader - 11:11\\]
Debug."]
pub type DebugR = crate::BitReader;
#[doc = "Field `DEBUG` writer - 11:11\\]
Debug."]
pub type DebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ` reader - 12:12\\]
Read."]
pub type ReadR = crate::BitReader;
#[doc = "Field `READ` writer - 12:12\\]
Read."]
pub type ReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - 13:13\\]
Write."]
pub type WriteR = crate::BitReader;
#[doc = "Field `WRITE` writer - 13:13\\]
Write."]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROUTEID` reader - 27:16\\]
Route ID."]
pub type RouteidR = crate::FieldReader<u16>;
#[doc = "Field `ROUTEID` writer - 27:16\\]
Route ID."]
pub type RouteidW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priv ID."]
    #[inline(always)]
    pub fn priv_id(&self) -> PrivIdR {
        PrivIdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Secure."]
    #[inline(always)]
    pub fn secure(&self) -> SecureR {
        SecureR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Priv."]
    #[inline(always)]
    pub fn priv_(&self) -> PrivR {
        PrivR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Cacheable."]
    #[inline(always)]
    pub fn cacheable(&self) -> CacheableR {
        CacheableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Debug."]
    #[inline(always)]
    pub fn debug(&self) -> DebugR {
        DebugR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Read."]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Write."]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Route ID."]
    #[inline(always)]
    pub fn routeid(&self) -> RouteidR {
        RouteidR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priv ID."]
    #[inline(always)]
    #[must_use]
    pub fn priv_id(&mut self) -> PrivIdW<ErrRegsExceptionLoggingData2Spec> {
        PrivIdW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Secure."]
    #[inline(always)]
    #[must_use]
    pub fn secure(&mut self) -> SecureW<ErrRegsExceptionLoggingData2Spec> {
        SecureW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Priv."]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PrivW<ErrRegsExceptionLoggingData2Spec> {
        PrivW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Cacheable."]
    #[inline(always)]
    #[must_use]
    pub fn cacheable(&mut self) -> CacheableW<ErrRegsExceptionLoggingData2Spec> {
        CacheableW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Debug."]
    #[inline(always)]
    #[must_use]
    pub fn debug(&mut self) -> DebugW<ErrRegsExceptionLoggingData2Spec> {
        DebugW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Read."]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> ReadW<ErrRegsExceptionLoggingData2Spec> {
        ReadW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Write."]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WriteW<ErrRegsExceptionLoggingData2Spec> {
        WriteW::new(self, 13)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Route ID."]
    #[inline(always)]
    #[must_use]
    pub fn routeid(&mut self) -> RouteidW<ErrRegsExceptionLoggingData2Spec> {
        RouteidW::new(self, 16)
    }
}
#[doc = "The Exception Logging Data 2 Register contains the third word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_exception_logging_data2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_exception_logging_data2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrRegsExceptionLoggingData2Spec;
impl crate::RegisterSpec for ErrRegsExceptionLoggingData2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_regs_exception_logging_data2::R`](R) reader structure"]
impl crate::Readable for ErrRegsExceptionLoggingData2Spec {}
#[doc = "`write(|w| ..)` method takes [`err_regs_exception_logging_data2::W`](W) writer structure"]
impl crate::Writable for ErrRegsExceptionLoggingData2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_REGS_exception_logging_data2 to value 0"]
impl crate::Resettable for ErrRegsExceptionLoggingData2Spec {
    const RESET_VALUE: u32 = 0;
}
