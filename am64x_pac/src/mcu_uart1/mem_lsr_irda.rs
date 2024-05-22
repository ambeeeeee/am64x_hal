#[doc = "Register `MEM_LSR_IRDA` reader"]
pub type R = crate::R<MemLsrIrdaSpec>;
#[doc = "Register `MEM_LSR_IRDA` writer"]
pub type W = crate::W<MemLsrIrdaSpec>;
#[doc = "Field `RX_FIFO_E` reader - "]
pub type RxFifoER = crate::BitReader;
#[doc = "Field `RX_FIFO_E` writer - "]
pub type RxFifoEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STS_FIFO_E` reader - "]
pub type StsFifoER = crate::BitReader;
#[doc = "Field `STS_FIFO_E` writer - "]
pub type StsFifoEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - "]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - "]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` reader - "]
pub type AbortR = crate::BitReader;
#[doc = "Field `ABORT` writer - "]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_TOO_LONG` reader - "]
pub type FrameTooLongR = crate::BitReader;
#[doc = "Field `FRAME_TOO_LONG` writer - "]
pub type FrameTooLongW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_LAST_BYTE` reader - "]
pub type RxLastByteR = crate::BitReader;
#[doc = "Field `RX_LAST_BYTE` writer - "]
pub type RxLastByteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STS_FIFO_FULL` reader - "]
pub type StsFifoFullR = crate::BitReader;
#[doc = "Field `STS_FIFO_FULL` writer - "]
pub type StsFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_EMPTY` reader - "]
pub type ThrEmptyR = crate::BitReader;
#[doc = "Field `THR_EMPTY` writer - "]
pub type ThrEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_fifo_e(&self) -> RxFifoER {
        RxFifoER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sts_fifo_e(&self) -> StsFifoER {
        StsFifoER::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn frame_too_long(&self) -> FrameTooLongR {
        FrameTooLongR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_last_byte(&self) -> RxLastByteR {
        RxLastByteR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sts_fifo_full(&self) -> StsFifoFullR {
        StsFifoFullR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn thr_empty(&self) -> ThrEmptyR {
        ThrEmptyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_e(&mut self) -> RxFifoEW<MemLsrIrdaSpec> {
        RxFifoEW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sts_fifo_e(&mut self) -> StsFifoEW<MemLsrIrdaSpec> {
        StsFifoEW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CrcW<MemLsrIrdaSpec> {
        CrcW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> AbortW<MemLsrIrdaSpec> {
        AbortW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn frame_too_long(&mut self) -> FrameTooLongW<MemLsrIrdaSpec> {
        FrameTooLongW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rx_last_byte(&mut self) -> RxLastByteW<MemLsrIrdaSpec> {
        RxLastByteW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn sts_fifo_full(&mut self) -> StsFifoFullW<MemLsrIrdaSpec> {
        StsFifoFullW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn thr_empty(&mut self) -> ThrEmptyW<MemLsrIrdaSpec> {
        ThrEmptyW::new(self, 7)
    }
}
#[doc = "MEM_LSR_IRDA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_lsr_irda::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_lsr_irda::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemLsrIrdaSpec;
impl crate::RegisterSpec for MemLsrIrdaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_lsr_irda::R`](R) reader structure"]
impl crate::Readable for MemLsrIrdaSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_lsr_irda::W`](W) writer structure"]
impl crate::Writable for MemLsrIrdaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_LSR_IRDA to value 0x83"]
impl crate::Resettable for MemLsrIrdaSpec {
    const RESET_VALUE: u32 = 0x83;
}
