#[doc = "Register `CFG_RX_DMA_CTRL` reader"]
pub type R = crate::R<CfgRxDmaCtrlSpec>;
#[doc = "Register `CFG_RX_DMA_CTRL` writer"]
pub type W = crate::W<CfgRxDmaCtrlSpec>;
#[doc = "Field `DMA_EVT_EN` reader - 0:0\\]
DMA Event Enable bitThis bit will enable a DMA Event to be generated upon the completion of a frame reception. 0h \\[R/W\\]
= A DMA event will not be generated.1h \\[R/W\\]
= A DMA event will be generated upon the reception of a frame. Note: The DMA event will only be generated for data frames."]
pub type DmaEvtEnR = crate::BitReader;
#[doc = "Field `DMA_EVT_EN` writer - 0:0\\]
DMA Event Enable bitThis bit will enable a DMA Event to be generated upon the completion of a frame reception. 0h \\[R/W\\]
= A DMA event will not be generated.1h \\[R/W\\]
= A DMA event will be generated upon the reception of a frame. Note: The DMA event will only be generated for data frames."]
pub type DmaEvtEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMA Event Enable bitThis bit will enable a DMA Event to be generated upon the completion of a frame reception. 0h \\[R/W\\]
= A DMA event will not be generated.1h \\[R/W\\]
= A DMA event will be generated upon the reception of a frame. Note: The DMA event will only be generated for data frames."]
    #[inline(always)]
    pub fn dma_evt_en(&self) -> DmaEvtEnR {
        DmaEvtEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMA Event Enable bitThis bit will enable a DMA Event to be generated upon the completion of a frame reception. 0h \\[R/W\\]
= A DMA event will not be generated.1h \\[R/W\\]
= A DMA event will be generated upon the reception of a frame. Note: The DMA event will only be generated for data frames."]
    #[inline(always)]
    #[must_use]
    pub fn dma_evt_en(&mut self) -> DmaEvtEnW<CfgRxDmaCtrlSpec> {
        DmaEvtEnW::new(self, 0)
    }
}
#[doc = "Receive DMA event control register. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_dma_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_dma_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxDmaCtrlSpec;
impl crate::RegisterSpec for CfgRxDmaCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_dma_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgRxDmaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_dma_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgRxDmaCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_DMA_CTRL to value 0"]
impl crate::Resettable for CfgRxDmaCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
