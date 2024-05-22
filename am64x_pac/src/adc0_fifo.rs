#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    adc12_fifo_dma_fifo0dmadata: Adc12FifoDmaFifo0dmadata,
    _reserved1: [u8; 0xfc],
    adc12_fifo_dma_fifo1dmadata: Adc12FifoDmaFifo1dmadata,
}
impl RegisterBlock {
    #[doc = "0x100 - DMA sample FIFO"]
    #[inline(always)]
    pub const fn adc12_fifo_dma_fifo0dmadata(&self) -> &Adc12FifoDmaFifo0dmadata {
        &self.adc12_fifo_dma_fifo0dmadata
    }
    #[doc = "0x200 - DMA sample FIFO"]
    #[inline(always)]
    pub const fn adc12_fifo_dma_fifo1dmadata(&self) -> &Adc12FifoDmaFifo1dmadata {
        &self.adc12_fifo_dma_fifo1dmadata
    }
}
#[doc = "ADC12_FIFO_DMA_FIFO0DMADATA (rw) register accessor: DMA sample FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12_fifo_dma_fifo0dmadata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12_fifo_dma_fifo0dmadata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12_fifo_dma_fifo0dmadata`]
module"]
#[doc(alias = "ADC12_FIFO_DMA_FIFO0DMADATA")]
pub type Adc12FifoDmaFifo0dmadata =
    crate::Reg<adc12_fifo_dma_fifo0dmadata::Adc12FifoDmaFifo0dmadataSpec>;
#[doc = "DMA sample FIFO"]
pub mod adc12_fifo_dma_fifo0dmadata;
#[doc = "ADC12_FIFO_DMA_FIFO1DMADATA (rw) register accessor: DMA sample FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12_fifo_dma_fifo1dmadata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12_fifo_dma_fifo1dmadata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12_fifo_dma_fifo1dmadata`]
module"]
#[doc(alias = "ADC12_FIFO_DMA_FIFO1DMADATA")]
pub type Adc12FifoDmaFifo1dmadata =
    crate::Reg<adc12_fifo_dma_fifo1dmadata::Adc12FifoDmaFifo1dmadataSpec>;
#[doc = "DMA sample FIFO"]
pub mod adc12_fifo_dma_fifo1dmadata;
