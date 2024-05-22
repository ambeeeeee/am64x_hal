#[doc = "Register `MEM_IIR_IRDA` reader"]
pub type R = crate::R<MemIirIrdaSpec>;
#[doc = "Register `MEM_IIR_IRDA` writer"]
pub type W = crate::W<MemIirIrdaSpec>;
#[doc = "Field `RHR_IT` reader - "]
pub type RhrItR = crate::BitReader;
#[doc = "Field `RHR_IT` writer - "]
pub type RhrItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_IT` reader - "]
pub type ThrItR = crate::BitReader;
#[doc = "Field `THR_IT` writer - "]
pub type ThrItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_LAST_BYTE_IT` reader - "]
pub type RxFifoLastByteItR = crate::BitReader;
#[doc = "Field `RX_FIFO_LAST_BYTE_IT` writer - "]
pub type RxFifoLastByteItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OE_IT` reader - "]
pub type RxOeItR = crate::BitReader;
#[doc = "Field `RX_OE_IT` writer - "]
pub type RxOeItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STS_FIFO_IT` reader - "]
pub type StsFifoItR = crate::BitReader;
#[doc = "Field `STS_FIFO_IT` writer - "]
pub type StsFifoItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_STATUS_IT` reader - "]
pub type TxStatusItR = crate::BitReader;
#[doc = "Field `TX_STATUS_IT` writer - "]
pub type TxStatusItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE_STS_IT` reader - "]
pub type LineStsItR = crate::BitReader;
#[doc = "Field `LINE_STS_IT` writer - "]
pub type LineStsItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOF_IT` reader - "]
pub type EofItR = crate::BitReader;
#[doc = "Field `EOF_IT` writer - "]
pub type EofItW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rhr_it(&self) -> RhrItR {
        RhrItR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn thr_it(&self) -> ThrItR {
        ThrItR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_fifo_last_byte_it(&self) -> RxFifoLastByteItR {
        RxFifoLastByteItR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_oe_it(&self) -> RxOeItR {
        RxOeItR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sts_fifo_it(&self) -> StsFifoItR {
        StsFifoItR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_status_it(&self) -> TxStatusItR {
        TxStatusItR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn line_sts_it(&self) -> LineStsItR {
        LineStsItR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn eof_it(&self) -> EofItR {
        EofItR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rhr_it(&mut self) -> RhrItW<MemIirIrdaSpec> {
        RhrItW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn thr_it(&mut self) -> ThrItW<MemIirIrdaSpec> {
        ThrItW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_last_byte_it(&mut self) -> RxFifoLastByteItW<MemIirIrdaSpec> {
        RxFifoLastByteItW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rx_oe_it(&mut self) -> RxOeItW<MemIirIrdaSpec> {
        RxOeItW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sts_fifo_it(&mut self) -> StsFifoItW<MemIirIrdaSpec> {
        StsFifoItW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tx_status_it(&mut self) -> TxStatusItW<MemIirIrdaSpec> {
        TxStatusItW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn line_sts_it(&mut self) -> LineStsItW<MemIirIrdaSpec> {
        LineStsItW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn eof_it(&mut self) -> EofItW<MemIirIrdaSpec> {
        EofItW::new(self, 7)
    }
}
#[doc = "The IIR is a read-only register, which provides the source of the interrupt in a prioritized manner.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_iir_irda::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_iir_irda::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemIirIrdaSpec;
impl crate::RegisterSpec for MemIirIrdaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_iir_irda::R`](R) reader structure"]
impl crate::Readable for MemIirIrdaSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_iir_irda::W`](W) writer structure"]
impl crate::Writable for MemIirIrdaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_IIR_IRDA to value 0"]
impl crate::Resettable for MemIirIrdaSpec {
    const RESET_VALUE: u32 = 0;
}
