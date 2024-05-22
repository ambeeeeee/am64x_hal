#[doc = "Register `CFG_GPMC_ERR_TYPE` reader"]
pub type R = crate::R<CfgGpmcErrTypeSpec>;
#[doc = "Register `CFG_GPMC_ERR_TYPE` writer"]
pub type W = crate::W<CfgGpmcErrTypeSpec>;
#[doc = "Field `ERRORVALID` reader - 0:0\\]
Error validity status - Must be explicitely cleared with a write 1 transaction"]
pub type ErrorvalidR = crate::BitReader;
#[doc = "Field `ERRORVALID` writer - 0:0\\]
Error validity status - Must be explicitely cleared with a write 1 transaction"]
pub type ErrorvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORTIMEOUT` reader - 2:2\\]
Time-out error"]
pub type ErrortimeoutR = crate::BitReader;
#[doc = "Field `ERRORTIMEOUT` writer - 2:2\\]
Time-out error"]
pub type ErrortimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORNOTSUPPMCMD` reader - 3:3\\]
Not supported Command error"]
pub type ErrornotsuppmcmdR = crate::BitReader;
#[doc = "Field `ERRORNOTSUPPMCMD` writer - 3:3\\]
Not supported Command error"]
pub type ErrornotsuppmcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRORNOTSUPPADD` reader - 4:4\\]
Not supported Address error"]
pub type ErrornotsuppaddR = crate::BitReader;
#[doc = "Field `ERRORNOTSUPPADD` writer - 4:4\\]
Not supported Address error"]
pub type ErrornotsuppaddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILLEGALMCMD` reader - 10:8\\]
System Command of the transaction that caused the error"]
pub type IllegalmcmdR = crate::FieldReader;
#[doc = "Field `ILLEGALMCMD` writer - 10:8\\]
System Command of the transaction that caused the error"]
pub type IllegalmcmdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Error validity status - Must be explicitely cleared with a write 1 transaction"]
    #[inline(always)]
    pub fn errorvalid(&self) -> ErrorvalidR {
        ErrorvalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Time-out error"]
    #[inline(always)]
    pub fn errortimeout(&self) -> ErrortimeoutR {
        ErrortimeoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Not supported Command error"]
    #[inline(always)]
    pub fn errornotsuppmcmd(&self) -> ErrornotsuppmcmdR {
        ErrornotsuppmcmdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Not supported Address error"]
    #[inline(always)]
    pub fn errornotsuppadd(&self) -> ErrornotsuppaddR {
        ErrornotsuppaddR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
System Command of the transaction that caused the error"]
    #[inline(always)]
    pub fn illegalmcmd(&self) -> IllegalmcmdR {
        IllegalmcmdR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Error validity status - Must be explicitely cleared with a write 1 transaction"]
    #[inline(always)]
    #[must_use]
    pub fn errorvalid(&mut self) -> ErrorvalidW<CfgGpmcErrTypeSpec> {
        ErrorvalidW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Time-out error"]
    #[inline(always)]
    #[must_use]
    pub fn errortimeout(&mut self) -> ErrortimeoutW<CfgGpmcErrTypeSpec> {
        ErrortimeoutW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Not supported Command error"]
    #[inline(always)]
    #[must_use]
    pub fn errornotsuppmcmd(&mut self) -> ErrornotsuppmcmdW<CfgGpmcErrTypeSpec> {
        ErrornotsuppmcmdW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Not supported Address error"]
    #[inline(always)]
    #[must_use]
    pub fn errornotsuppadd(&mut self) -> ErrornotsuppaddW<CfgGpmcErrTypeSpec> {
        ErrornotsuppaddW::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
System Command of the transaction that caused the error"]
    #[inline(always)]
    #[must_use]
    pub fn illegalmcmd(&mut self) -> IllegalmcmdW<CfgGpmcErrTypeSpec> {
        IllegalmcmdW::new(self, 8)
    }
}
#[doc = "The GPMC_ERR_TYPE register stores the type of error when an error occurs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_err_type::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_err_type::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcErrTypeSpec;
impl crate::RegisterSpec for CfgGpmcErrTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_err_type::R`](R) reader structure"]
impl crate::Readable for CfgGpmcErrTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_err_type::W`](W) writer structure"]
impl crate::Writable for CfgGpmcErrTypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_ERR_TYPE to value 0"]
impl crate::Resettable for CfgGpmcErrTypeSpec {
    const RESET_VALUE: u32 = 0;
}
