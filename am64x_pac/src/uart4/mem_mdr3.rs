#[doc = "Register `MEM_MDR3` reader"]
pub type R = crate::R<MemMdr3Spec>;
#[doc = "Register `MEM_MDR3` writer"]
pub type W = crate::W<MemMdr3Spec>;
#[doc = "Field `DISABLE_CIR_RX_DEMOD` reader - 0:0\\]
Disables\\[1\\]/Enables\\[0\\]
CIR RX demodulation"]
pub type DisableCirRxDemodR = crate::BitReader;
#[doc = "Field `DISABLE_CIR_RX_DEMOD` writer - 0:0\\]
Disables\\[1\\]/Enables\\[0\\]
CIR RX demodulation"]
pub type DisableCirRxDemodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NONDEFAULT_FREQ` reader - 1:1\\]
Enables\\[1\\]/Disables\\[0\\]
using NONDEFAULT fclk frequencies"]
pub type NondefaultFreqR = crate::BitReader;
#[doc = "Field `NONDEFAULT_FREQ` writer - 1:1\\]
Enables\\[1\\]/Disables\\[0\\]
using NONDEFAULT fclk frequencies"]
pub type NondefaultFreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_DMA_TX_THRESHOLD` reader - 2:2\\]
Enable to set different TX DMA threshold then 64-trigger \\[usage of new register TX_DNA_THRESHOLD\\]"]
pub type SetDmaTxThresholdR = crate::BitReader;
#[doc = "Field `SET_DMA_TX_THRESHOLD` writer - 2:2\\]
Enable to set different TX DMA threshold then 64-trigger \\[usage of new register TX_DNA_THRESHOLD\\]"]
pub type SetDmaTxThresholdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR_POL` reader - 3:3\\]
RS-485 External Transceiver Direction Polarity. 0 => TX: RTS=0, RX: RTS=1. 1 => TX: RTS=1, RX: RTS=0"]
pub type DirPolR = crate::BitReader;
#[doc = "Field `DIR_POL` writer - 3:3\\]
RS-485 External Transceiver Direction Polarity. 0 => TX: RTS=0, RX: RTS=1. 1 => TX: RTS=1, RX: RTS=0"]
pub type DirPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR_EN` reader - 4:4\\]
RS-485 External Transceiver Direction Enable"]
pub type DirEnR = crate::BitReader;
#[doc = "Field `DIR_EN` writer - 4:4\\]
RS-485 External Transceiver Direction Enable"]
pub type DirEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - "]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - "]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disables\\[1\\]/Enables\\[0\\]
CIR RX demodulation"]
    #[inline(always)]
    pub fn disable_cir_rx_demod(&self) -> DisableCirRxDemodR {
        DisableCirRxDemodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables\\[1\\]/Disables\\[0\\]
using NONDEFAULT fclk frequencies"]
    #[inline(always)]
    pub fn nondefault_freq(&self) -> NondefaultFreqR {
        NondefaultFreqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable to set different TX DMA threshold then 64-trigger \\[usage of new register TX_DNA_THRESHOLD\\]"]
    #[inline(always)]
    pub fn set_dma_tx_threshold(&self) -> SetDmaTxThresholdR {
        SetDmaTxThresholdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
RS-485 External Transceiver Direction Polarity. 0 => TX: RTS=0, RX: RTS=1. 1 => TX: RTS=1, RX: RTS=0"]
    #[inline(always)]
    pub fn dir_pol(&self) -> DirPolR {
        DirPolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
RS-485 External Transceiver Direction Enable"]
    #[inline(always)]
    pub fn dir_en(&self) -> DirEnR {
        DirEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Disables\\[1\\]/Enables\\[0\\]
CIR RX demodulation"]
    #[inline(always)]
    #[must_use]
    pub fn disable_cir_rx_demod(&mut self) -> DisableCirRxDemodW<MemMdr3Spec> {
        DisableCirRxDemodW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables\\[1\\]/Disables\\[0\\]
using NONDEFAULT fclk frequencies"]
    #[inline(always)]
    #[must_use]
    pub fn nondefault_freq(&mut self) -> NondefaultFreqW<MemMdr3Spec> {
        NondefaultFreqW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable to set different TX DMA threshold then 64-trigger \\[usage of new register TX_DNA_THRESHOLD\\]"]
    #[inline(always)]
    #[must_use]
    pub fn set_dma_tx_threshold(&mut self) -> SetDmaTxThresholdW<MemMdr3Spec> {
        SetDmaTxThresholdW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
RS-485 External Transceiver Direction Polarity. 0 => TX: RTS=0, RX: RTS=1. 1 => TX: RTS=1, RX: RTS=0"]
    #[inline(always)]
    #[must_use]
    pub fn dir_pol(&mut self) -> DirPolW<MemMdr3Spec> {
        DirPolW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
RS-485 External Transceiver Direction Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dir_en(&mut self) -> DirEnW<MemMdr3Spec> {
        DirEnW::new(self, 4)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<MemMdr3Spec> {
        Reserved2W::new(self, 8)
    }
}
#[doc = "Mode definition register 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mdr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mdr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemMdr3Spec;
impl crate::RegisterSpec for MemMdr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_mdr3::R`](R) reader structure"]
impl crate::Readable for MemMdr3Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_mdr3::W`](W) writer structure"]
impl crate::Writable for MemMdr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_MDR3 to value 0"]
impl crate::Resettable for MemMdr3Spec {
    const RESET_VALUE: u32 = 0;
}
