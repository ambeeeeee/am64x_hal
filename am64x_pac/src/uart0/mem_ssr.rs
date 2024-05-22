#[doc = "Register `MEM_SSR` reader"]
pub type R = crate::R<MemSsrSpec>;
#[doc = "Register `MEM_SSR` writer"]
pub type W = crate::W<MemSsrSpec>;
#[doc = "Field `TX_FIFO_FULL` reader - "]
pub type TxFifoFullR = crate::BitReader;
#[doc = "Field `TX_FIFO_FULL` writer - "]
pub type TxFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CTS_DSR_WAKE_UP_STS` reader - "]
pub type RxCtsDsrWakeUpStsR = crate::BitReader;
#[doc = "Field `RX_CTS_DSR_WAKE_UP_STS` writer - "]
pub type RxCtsDsrWakeUpStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_COUNTER_RST` reader - "]
pub type DmaCounterRstR = crate::BitReader;
#[doc = "Field `DMA_COUNTER_RST` writer - "]
pub type DmaCounterRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_fifo_full(&self) -> TxFifoFullR {
        TxFifoFullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_cts_dsr_wake_up_sts(&self) -> RxCtsDsrWakeUpStsR {
        RxCtsDsrWakeUpStsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_counter_rst(&self) -> DmaCounterRstR {
        DmaCounterRstR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_full(&mut self) -> TxFifoFullW<MemSsrSpec> {
        TxFifoFullW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_cts_dsr_wake_up_sts(&mut self) -> RxCtsDsrWakeUpStsW<MemSsrSpec> {
        RxCtsDsrWakeUpStsW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dma_counter_rst(&mut self) -> DmaCounterRstW<MemSsrSpec> {
        DmaCounterRstW::new(self, 2)
    }
}
#[doc = "Note: Bit 1 is reset only when SCR\\[4\\]
is reset to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ssr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ssr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSsrSpec;
impl crate::RegisterSpec for MemSsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ssr::R`](R) reader structure"]
impl crate::Readable for MemSsrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_ssr::W`](W) writer structure"]
impl crate::Writable for MemSsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SSR to value 0x04"]
impl crate::Resettable for MemSsrSpec {
    const RESET_VALUE: u32 = 0x04;
}
