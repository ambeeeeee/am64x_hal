#[doc = "Register `MEM_IER_IRDA` reader"]
pub type R = crate::R<MemIerIrdaSpec>;
#[doc = "Register `MEM_IER_IRDA` writer"]
pub type W = crate::W<MemIerIrdaSpec>;
#[doc = "Field `RHR_IT` reader - "]
pub type RhrItR = crate::BitReader;
#[doc = "Field `RHR_IT` writer - "]
pub type RhrItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_IT` reader - "]
pub type ThrItR = crate::BitReader;
#[doc = "Field `THR_IT` writer - "]
pub type ThrItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAST_RX_BYTE_IT` reader - "]
pub type LastRxByteItR = crate::BitReader;
#[doc = "Field `LAST_RX_BYTE_IT` writer - "]
pub type LastRxByteItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OVERRUN_IT` reader - "]
pub type RxOverrunItR = crate::BitReader;
#[doc = "Field `RX_OVERRUN_IT` writer - "]
pub type RxOverrunItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STS_FIFO_TRIG_IT` reader - "]
pub type StsFifoTrigItR = crate::BitReader;
#[doc = "Field `STS_FIFO_TRIG_IT` writer - "]
pub type StsFifoTrigItW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn last_rx_byte_it(&self) -> LastRxByteItR {
        LastRxByteItR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_overrun_it(&self) -> RxOverrunItR {
        RxOverrunItR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sts_fifo_trig_it(&self) -> StsFifoTrigItR {
        StsFifoTrigItR::new(((self.bits >> 4) & 1) != 0)
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
    pub fn rhr_it(&mut self) -> RhrItW<MemIerIrdaSpec> {
        RhrItW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn thr_it(&mut self) -> ThrItW<MemIerIrdaSpec> {
        ThrItW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn last_rx_byte_it(&mut self) -> LastRxByteItW<MemIerIrdaSpec> {
        LastRxByteItW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rx_overrun_it(&mut self) -> RxOverrunItW<MemIerIrdaSpec> {
        RxOverrunItW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sts_fifo_trig_it(&mut self) -> StsFifoTrigItW<MemIerIrdaSpec> {
        StsFifoTrigItW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tx_status_it(&mut self) -> TxStatusItW<MemIerIrdaSpec> {
        TxStatusItW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn line_sts_it(&mut self) -> LineStsItW<MemIerIrdaSpec> {
        LineStsItW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn eof_it(&mut self) -> EofItW<MemIerIrdaSpec> {
        EofItW::new(self, 7)
    }
}
#[doc = "The interrupt enable register (IER) can be programmed to enable/disable any interrupt. There are 8 types of interrupt in these modes, received EOF, LSR interrupt, TX status, status FIFO interrupt, RX overrun, last byte in RX FIFO, THR interrupt and RHR interrupt and they can be enabled/disabled individually.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ier_irda::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ier_irda::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemIerIrdaSpec;
impl crate::RegisterSpec for MemIerIrdaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ier_irda::R`](R) reader structure"]
impl crate::Readable for MemIerIrdaSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_ier_irda::W`](W) writer structure"]
impl crate::Writable for MemIerIrdaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_IER_IRDA to value 0"]
impl crate::Resettable for MemIerIrdaSpec {
    const RESET_VALUE: u32 = 0;
}
