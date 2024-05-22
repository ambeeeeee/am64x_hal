#[doc = "Register `MEM_SFLSR` reader"]
pub type R = crate::R<MemSflsrSpec>;
#[doc = "Register `MEM_SFLSR` writer"]
pub type W = crate::W<MemSflsrSpec>;
#[doc = "Field `RESERVED0` reader - "]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - "]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_ERROR` reader - "]
pub type CrcErrorR = crate::BitReader;
#[doc = "Field `CRC_ERROR` writer - "]
pub type CrcErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT_DETECT` reader - "]
pub type AbortDetectR = crate::BitReader;
#[doc = "Field `ABORT_DETECT` writer - "]
pub type AbortDetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_TOO_LONG_ERROR` reader - "]
pub type FrameTooLongErrorR = crate::BitReader;
#[doc = "Field `FRAME_TOO_LONG_ERROR` writer - "]
pub type FrameTooLongErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE_ERROR` reader - "]
pub type OeErrorR = crate::BitReader;
#[doc = "Field `OE_ERROR` writer - "]
pub type OeErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED5` reader - "]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - "]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn crc_error(&self) -> CrcErrorR {
        CrcErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn abort_detect(&self) -> AbortDetectR {
        AbortDetectR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn frame_too_long_error(&self) -> FrameTooLongErrorR {
        FrameTooLongErrorR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn oe_error(&self) -> OeErrorR {
        OeErrorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<MemSflsrSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn crc_error(&mut self) -> CrcErrorW<MemSflsrSpec> {
        CrcErrorW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn abort_detect(&mut self) -> AbortDetectW<MemSflsrSpec> {
        AbortDetectW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn frame_too_long_error(&mut self) -> FrameTooLongErrorW<MemSflsrSpec> {
        FrameTooLongErrorW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn oe_error(&mut self) -> OeErrorW<MemSflsrSpec> {
        OeErrorW::new(self, 4)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<MemSflsrSpec> {
        Reserved5W::new(self, 5)
    }
}
#[doc = "IrDA modes only. Reading this register effectively reads frame status information from the status FIFO (this register doesn't physically exist). Reading this register will increment the status FIFO read pointer (SFREGL and SFREGH must be read first).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_sflsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_sflsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSflsrSpec;
impl crate::RegisterSpec for MemSflsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_sflsr::R`](R) reader structure"]
impl crate::Readable for MemSflsrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_sflsr::W`](W) writer structure"]
impl crate::Writable for MemSflsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SFLSR to value 0"]
impl crate::Resettable for MemSflsrSpec {
    const RESET_VALUE: u32 = 0;
}
