#[doc = "Register `MEM_TLR` reader"]
pub type R = crate::R<MemTlrSpec>;
#[doc = "Register `MEM_TLR` writer"]
pub type W = crate::W<MemTlrSpec>;
#[doc = "Field `TX_FIFO_TRIG_DMA` reader - 3:0\\]
Transmit FIFO trigger level"]
pub type TxFifoTrigDmaR = crate::FieldReader;
#[doc = "Field `TX_FIFO_TRIG_DMA` writer - 3:0\\]
Transmit FIFO trigger level"]
pub type TxFifoTrigDmaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_FIFO_TRIG_DMA` reader - 7:4\\]
Receive FIFO trigger level"]
pub type RxFifoTrigDmaR = crate::FieldReader;
#[doc = "Field `RX_FIFO_TRIG_DMA` writer - 7:4\\]
Receive FIFO trigger level"]
pub type RxFifoTrigDmaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Transmit FIFO trigger level"]
    #[inline(always)]
    pub fn tx_fifo_trig_dma(&self) -> TxFifoTrigDmaR {
        TxFifoTrigDmaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Receive FIFO trigger level"]
    #[inline(always)]
    pub fn rx_fifo_trig_dma(&self) -> RxFifoTrigDmaR {
        RxFifoTrigDmaR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Transmit FIFO trigger level"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_trig_dma(&mut self) -> TxFifoTrigDmaW<MemTlrSpec> {
        TxFifoTrigDmaW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Receive FIFO trigger level"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_trig_dma(&mut self) -> RxFifoTrigDmaW<MemTlrSpec> {
        RxFifoTrigDmaW::new(self, 4)
    }
}
#[doc = "Trigger Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_tlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_tlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemTlrSpec;
impl crate::RegisterSpec for MemTlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_tlr::R`](R) reader structure"]
impl crate::Readable for MemTlrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_tlr::W`](W) writer structure"]
impl crate::Writable for MemTlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_TLR to value 0"]
impl crate::Resettable for MemTlrSpec {
    const RESET_VALUE: u32 = 0;
}
