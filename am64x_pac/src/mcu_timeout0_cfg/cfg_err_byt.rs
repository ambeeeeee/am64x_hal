#[doc = "Register `CFG_ERR_BYT` reader"]
pub type R = crate::R<CfgErrBytSpec>;
#[doc = "Register `CFG_ERR_BYT` writer"]
pub type W = crate::W<CfgErrBytSpec>;
#[doc = "Field `OBYTECNT` reader - 9:0\\]
Original Bytecnt"]
pub type ObytecntR = crate::FieldReader<u16>;
#[doc = "Field `OBYTECNT` writer - 9:0\\]
Original Bytecnt"]
pub type ObytecntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CBYTECNT` reader - 25:16\\]
Current Bytecnt"]
pub type CbytecntR = crate::FieldReader<u16>;
#[doc = "Field `CBYTECNT` writer - 25:16\\]
Current Bytecnt"]
pub type CbytecntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Original Bytecnt"]
    #[inline(always)]
    pub fn obytecnt(&self) -> ObytecntR {
        ObytecntR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Current Bytecnt"]
    #[inline(always)]
    pub fn cbytecnt(&self) -> CbytecntR {
        CbytecntR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Original Bytecnt"]
    #[inline(always)]
    #[must_use]
    pub fn obytecnt(&mut self) -> ObytecntW<CfgErrBytSpec> {
        ObytecntW::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Current Bytecnt"]
    #[inline(always)]
    #[must_use]
    pub fn cbytecnt(&mut self) -> CbytecntW<CfgErrBytSpec> {
        CbytecntW::new(self, 16)
    }
}
#[doc = "This register contains information about transaction that caused the interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_byt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_byt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgErrBytSpec;
impl crate::RegisterSpec for CfgErrBytSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_err_byt::R`](R) reader structure"]
impl crate::Readable for CfgErrBytSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_err_byt::W`](W) writer structure"]
impl crate::Writable for CfgErrBytSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_ERR_BYT to value 0"]
impl crate::Resettable for CfgErrBytSpec {
    const RESET_VALUE: u32 = 0;
}
