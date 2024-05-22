#[doc = "Register `MEM_TX_DMA_THRESHOLD` reader"]
pub type R = crate::R<MemTxDmaThresholdSpec>;
#[doc = "Register `MEM_TX_DMA_THRESHOLD` writer"]
pub type W = crate::W<MemTxDmaThresholdSpec>;
#[doc = "Field `TX_DMA_THRESHOLD` reader - 5:0\\]
Use to manually set the TX DMA threshold level."]
pub type TxDmaThresholdR = crate::FieldReader;
#[doc = "Field `TX_DMA_THRESHOLD` writer - 5:0\\]
Use to manually set the TX DMA threshold level."]
pub type TxDmaThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Use to manually set the TX DMA threshold level."]
    #[inline(always)]
    pub fn tx_dma_threshold(&self) -> TxDmaThresholdR {
        TxDmaThresholdR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Use to manually set the TX DMA threshold level."]
    #[inline(always)]
    #[must_use]
    pub fn tx_dma_threshold(&mut self) -> TxDmaThresholdW<MemTxDmaThresholdSpec> {
        TxDmaThresholdW::new(self, 0)
    }
}
#[doc = "Use to manually set the TX DMA threshold level. MDR3\\[2\\]
SET_TX_DMA_THRESHOLD must be one and must be value + tx_trigger_level &lt;= 64 (TX FIFO size). If not, 64-tx_trigger_level will be used w/o modifying the value of this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_tx_dma_threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_tx_dma_threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemTxDmaThresholdSpec;
impl crate::RegisterSpec for MemTxDmaThresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_tx_dma_threshold::R`](R) reader structure"]
impl crate::Readable for MemTxDmaThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_tx_dma_threshold::W`](W) writer structure"]
impl crate::Writable for MemTxDmaThresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_TX_DMA_THRESHOLD to value 0"]
impl crate::Resettable for MemTxDmaThresholdSpec {
    const RESET_VALUE: u32 = 0;
}
