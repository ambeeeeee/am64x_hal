#[doc = "Register `CFG_ERR` reader"]
pub type R = crate::R<CfgErrSpec>;
#[doc = "Register `CFG_ERR` writer"]
pub type W = crate::W<CfgErrSpec>;
#[doc = "Field `TIMEOUT` reader - 0:0\\]
Masked Timeout Error Interrupt"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - 0:0\\]
Masked Timeout Error Interrupt"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNEXP` reader - 1:1\\]
Masked Unexpected Error Interrupt"]
pub type UnexpR = crate::BitReader;
#[doc = "Field `UNEXP` writer - 1:1\\]
Masked Unexpected Error Interrupt"]
pub type UnexpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD` reader - 2:2\\]
Masked Command Error Interrupt"]
pub type CmdR = crate::BitReader;
#[doc = "Field `CMD` writer - 2:2\\]
Masked Command Error Interrupt"]
pub type CmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Masked Timeout Error Interrupt"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Masked Unexpected Error Interrupt"]
    #[inline(always)]
    pub fn unexp(&self) -> UnexpR {
        UnexpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Masked Command Error Interrupt"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Masked Timeout Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<CfgErrSpec> {
        TimeoutW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Masked Unexpected Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn unexp(&mut self) -> UnexpW<CfgErrSpec> {
        UnexpW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Masked Command Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<CfgErrSpec> {
        CmdW::new(self, 2)
    }
}
#[doc = "This register contains the masked interrupt bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgErrSpec;
impl crate::RegisterSpec for CfgErrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_err::R`](R) reader structure"]
impl crate::Readable for CfgErrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_err::W`](W) writer structure"]
impl crate::Writable for CfgErrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_ERR to value 0"]
impl crate::Resettable for CfgErrSpec {
    const RESET_VALUE: u32 = 0;
}
