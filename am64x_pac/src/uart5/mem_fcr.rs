#[doc = "Register `MEM_FCR` reader"]
pub type R = crate::R<MemFcrSpec>;
#[doc = "Register `MEM_FCR` writer"]
pub type W = crate::W<MemFcrSpec>;
#[doc = "Field `FIFO_EN` reader - "]
pub type FifoEnR = crate::BitReader;
#[doc = "Field `FIFO_EN` writer - "]
pub type FifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_CLEAR` reader - "]
pub type RxFifoClearR = crate::BitReader;
#[doc = "Field `RX_FIFO_CLEAR` writer - "]
pub type RxFifoClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_CLEAR` reader - "]
pub type TxFifoClearR = crate::BitReader;
#[doc = "Field `TX_FIFO_CLEAR` writer - "]
pub type TxFifoClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_MODE` reader - 3:3\\]
This register is considered if SCR\\[0\\]
= 0."]
pub type DmaModeR = crate::BitReader;
#[doc = "Field `DMA_MODE` writer - 3:3\\]
This register is considered if SCR\\[0\\]
= 0."]
pub type DmaModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_TRIG` reader - 5:4\\]
Sets the trigger level for the TX FIFO: If SCR\\[6\\]
= 0 and TLR\\[3:0\\]
= 0000: 00: 8 spaces 01: 16 spaces 10: 32 spaces 11: 56 spaces If SCR\\[6\\]
= 0 and TLR\\[3:0\\]
!= 0000, TX_FIFO_TRIG is not considered. If SCR\\[6\\]=1, TX_FIFO_TRIG is 2 LSB of the trigger level \\[1-63 on 6 bits\\]
with the granularity 1"]
pub type TxFifoTrigR = crate::FieldReader;
#[doc = "Field `TX_FIFO_TRIG` writer - 5:4\\]
Sets the trigger level for the TX FIFO: If SCR\\[6\\]
= 0 and TLR\\[3:0\\]
= 0000: 00: 8 spaces 01: 16 spaces 10: 32 spaces 11: 56 spaces If SCR\\[6\\]
= 0 and TLR\\[3:0\\]
!= 0000, TX_FIFO_TRIG is not considered. If SCR\\[6\\]=1, TX_FIFO_TRIG is 2 LSB of the trigger level \\[1-63 on 6 bits\\]
with the granularity 1"]
pub type TxFifoTrigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_FIFO_TRIG` reader - 7:6\\]
Sets the trigger level for the RX FIFO: If SCR\\[7\\]
= 0 and TLR\\[7:4\\]
= 0000: 00: 8 characters 01: 16 characters 10: 56 characters 11: 60 characters If SCR\\[7\\]
= 0 and TLR\\[7:4\\]
!= 0000, RX_FIFO_TRIG is not considered. If SCR\\[7\\]=1, RX_FIFO_TRIG is 2 LSB of the trigger level \\[1-63 on 6 bits\\]
with the granularity 1."]
pub type RxFifoTrigR = crate::FieldReader;
#[doc = "Field `RX_FIFO_TRIG` writer - 7:6\\]
Sets the trigger level for the RX FIFO: If SCR\\[7\\]
= 0 and TLR\\[7:4\\]
= 0000: 00: 8 characters 01: 16 characters 10: 56 characters 11: 60 characters If SCR\\[7\\]
= 0 and TLR\\[7:4\\]
!= 0000, RX_FIFO_TRIG is not considered. If SCR\\[7\\]=1, RX_FIFO_TRIG is 2 LSB of the trigger level \\[1-63 on 6 bits\\]
with the granularity 1."]
pub type RxFifoTrigW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifo_en(&self) -> FifoEnR {
        FifoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_fifo_clear(&self) -> RxFifoClearR {
        RxFifoClearR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_clear(&self) -> TxFifoClearR {
        TxFifoClearR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This register is considered if SCR\\[0\\]
= 0."]
    #[inline(always)]
    pub fn dma_mode(&self) -> DmaModeR {
        DmaModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Sets the trigger level for the TX FIFO: If SCR\\[6\\]
= 0 and TLR\\[3:0\\]
= 0000: 00: 8 spaces 01: 16 spaces 10: 32 spaces 11: 56 spaces If SCR\\[6\\]
= 0 and TLR\\[3:0\\]
!= 0000, TX_FIFO_TRIG is not considered. If SCR\\[6\\]=1, TX_FIFO_TRIG is 2 LSB of the trigger level \\[1-63 on 6 bits\\]
with the granularity 1"]
    #[inline(always)]
    pub fn tx_fifo_trig(&self) -> TxFifoTrigR {
        TxFifoTrigR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Sets the trigger level for the RX FIFO: If SCR\\[7\\]
= 0 and TLR\\[7:4\\]
= 0000: 00: 8 characters 01: 16 characters 10: 56 characters 11: 60 characters If SCR\\[7\\]
= 0 and TLR\\[7:4\\]
!= 0000, RX_FIFO_TRIG is not considered. If SCR\\[7\\]=1, RX_FIFO_TRIG is 2 LSB of the trigger level \\[1-63 on 6 bits\\]
with the granularity 1."]
    #[inline(always)]
    pub fn rx_fifo_trig(&self) -> RxFifoTrigR {
        RxFifoTrigR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_en(&mut self) -> FifoEnW<MemFcrSpec> {
        FifoEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_clear(&mut self) -> RxFifoClearW<MemFcrSpec> {
        RxFifoClearW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_clear(&mut self) -> TxFifoClearW<MemFcrSpec> {
        TxFifoClearW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
This register is considered if SCR\\[0\\]
= 0."]
    #[inline(always)]
    #[must_use]
    pub fn dma_mode(&mut self) -> DmaModeW<MemFcrSpec> {
        DmaModeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Sets the trigger level for the TX FIFO: If SCR\\[6\\]
= 0 and TLR\\[3:0\\]
= 0000: 00: 8 spaces 01: 16 spaces 10: 32 spaces 11: 56 spaces If SCR\\[6\\]
= 0 and TLR\\[3:0\\]
!= 0000, TX_FIFO_TRIG is not considered. If SCR\\[6\\]=1, TX_FIFO_TRIG is 2 LSB of the trigger level \\[1-63 on 6 bits\\]
with the granularity 1"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_trig(&mut self) -> TxFifoTrigW<MemFcrSpec> {
        TxFifoTrigW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Sets the trigger level for the RX FIFO: If SCR\\[7\\]
= 0 and TLR\\[7:4\\]
= 0000: 00: 8 characters 01: 16 characters 10: 56 characters 11: 60 characters If SCR\\[7\\]
= 0 and TLR\\[7:4\\]
!= 0000, RX_FIFO_TRIG is not considered. If SCR\\[7\\]=1, RX_FIFO_TRIG is 2 LSB of the trigger level \\[1-63 on 6 bits\\]
with the granularity 1."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_trig(&mut self) -> RxFifoTrigW<MemFcrSpec> {
        RxFifoTrigW::new(self, 6)
    }
}
#[doc = "Notes: Bits 4 and 5 can only be written to when EFR\\[4\\]
= 1 Bits 0 to 3 can be changed only when the baud clock is not running (DLL and DLH set to 0) See Table 31 for FCR\\[5:4\\]
setting restriction when SCR\\[6\\]=1 See Table 32 for FCR\\[7:6\\]
setting restriction when SCR\\[7\\]=1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemFcrSpec;
impl crate::RegisterSpec for MemFcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_fcr::R`](R) reader structure"]
impl crate::Readable for MemFcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_fcr::W`](W) writer structure"]
impl crate::Writable for MemFcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_FCR to value 0"]
impl crate::Resettable for MemFcrSpec {
    const RESET_VALUE: u32 = 0;
}
