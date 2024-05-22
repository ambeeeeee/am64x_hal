#[doc = "Register `MEM_SCR` reader"]
pub type R = crate::R<MemScrSpec>;
#[doc = "Register `MEM_SCR` writer"]
pub type W = crate::W<MemScrSpec>;
#[doc = "Field `DMA_MODE_CTL` reader - "]
pub type DmaModeCtlR = crate::BitReader;
#[doc = "Field `DMA_MODE_CTL` writer - "]
pub type DmaModeCtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_MODE_2` reader - 2:1\\]
Used to specify the DMA mode valid if SCR\\[0\\]
= 1"]
pub type DmaMode2R = crate::FieldReader;
#[doc = "Field `DMA_MODE_2` writer - 2:1\\]
Used to specify the DMA mode valid if SCR\\[0\\]
= 1"]
pub type DmaMode2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_EMPTY_CTL_IT` reader - "]
pub type TxEmptyCtlItR = crate::BitReader;
#[doc = "Field `TX_EMPTY_CTL_IT` writer - "]
pub type TxEmptyCtlItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CTS_DSR_WAKE_UP_ENABLE` reader - "]
pub type RxCtsDsrWakeUpEnableR = crate::BitReader;
#[doc = "Field `RX_CTS_DSR_WAKE_UP_ENABLE` writer - "]
pub type RxCtsDsrWakeUpEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR_IT` reader - "]
pub type DsrItR = crate::BitReader;
#[doc = "Field `DSR_IT` writer - "]
pub type DsrItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_TRIG_GRANU1` reader - "]
pub type TxTrigGranu1R = crate::BitReader;
#[doc = "Field `TX_TRIG_GRANU1` writer - "]
pub type TxTrigGranu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TRIG_GRANU1` reader - "]
pub type RxTrigGranu1R = crate::BitReader;
#[doc = "Field `RX_TRIG_GRANU1` writer - "]
pub type RxTrigGranu1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_mode_ctl(&self) -> DmaModeCtlR {
        DmaModeCtlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Used to specify the DMA mode valid if SCR\\[0\\]
= 1"]
    #[inline(always)]
    pub fn dma_mode_2(&self) -> DmaMode2R {
        DmaMode2R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_empty_ctl_it(&self) -> TxEmptyCtlItR {
        TxEmptyCtlItR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_cts_dsr_wake_up_enable(&self) -> RxCtsDsrWakeUpEnableR {
        RxCtsDsrWakeUpEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dsr_it(&self) -> DsrItR {
        DsrItR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tx_trig_granu1(&self) -> TxTrigGranu1R {
        TxTrigGranu1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_trig_granu1(&self) -> RxTrigGranu1R {
        RxTrigGranu1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_mode_ctl(&mut self) -> DmaModeCtlW<MemScrSpec> {
        DmaModeCtlW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Used to specify the DMA mode valid if SCR\\[0\\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn dma_mode_2(&mut self) -> DmaMode2W<MemScrSpec> {
        DmaMode2W::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty_ctl_it(&mut self) -> TxEmptyCtlItW<MemScrSpec> {
        TxEmptyCtlItW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rx_cts_dsr_wake_up_enable(&mut self) -> RxCtsDsrWakeUpEnableW<MemScrSpec> {
        RxCtsDsrWakeUpEnableW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dsr_it(&mut self) -> DsrItW<MemScrSpec> {
        DsrItW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tx_trig_granu1(&mut self) -> TxTrigGranu1W<MemScrSpec> {
        TxTrigGranu1W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rx_trig_granu1(&mut self) -> RxTrigGranu1W<MemScrSpec> {
        RxTrigGranu1W::new(self, 7)
    }
}
#[doc = "Note: Bit 4 enables the wake-up interrupt, but this interrupt is not mapped into the IIR register. Therefore, when an interrupt occurs and there is no interrupt pending in the IIR register, the SSR\\[1\\]
bit must be checked. To clear the wake-up interrupt, bit SCR\\[4\\]
must be reset to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemScrSpec;
impl crate::RegisterSpec for MemScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_scr::R`](R) reader structure"]
impl crate::Readable for MemScrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_scr::W`](W) writer structure"]
impl crate::Writable for MemScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SCR to value 0"]
impl crate::Resettable for MemScrSpec {
    const RESET_VALUE: u32 = 0;
}
