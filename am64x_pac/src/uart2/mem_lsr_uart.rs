#[doc = "Register `MEM_LSR_UART` reader"]
pub type R = crate::R<MemLsrUartSpec>;
#[doc = "Register `MEM_LSR_UART` writer"]
pub type W = crate::W<MemLsrUartSpec>;
#[doc = "Field `RX_FIFO_E` reader - "]
pub type RxFifoER = crate::BitReader;
#[doc = "Field `RX_FIFO_E` writer - "]
pub type RxFifoEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OE` reader - "]
pub type RxOeR = crate::BitReader;
#[doc = "Field `RX_OE` writer - "]
pub type RxOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PE` reader - "]
pub type RxPeR = crate::BitReader;
#[doc = "Field `RX_PE` writer - "]
pub type RxPeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FE` reader - "]
pub type RxFeR = crate::BitReader;
#[doc = "Field `RX_FE` writer - "]
pub type RxFeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_BI` reader - "]
pub type RxBiR = crate::BitReader;
#[doc = "Field `RX_BI` writer - "]
pub type RxBiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_E` reader - "]
pub type TxFifoER = crate::BitReader;
#[doc = "Field `TX_FIFO_E` writer - "]
pub type TxFifoEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_SR_E` reader - "]
pub type TxSrER = crate::BitReader;
#[doc = "Field `TX_SR_E` writer - "]
pub type TxSrEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_STS` reader - "]
pub type RxFifoStsR = crate::BitReader;
#[doc = "Field `RX_FIFO_STS` writer - "]
pub type RxFifoStsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_fifo_e(&self) -> RxFifoER {
        RxFifoER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_oe(&self) -> RxOeR {
        RxOeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_pe(&self) -> RxPeR {
        RxPeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fe(&self) -> RxFeR {
        RxFeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rx_bi(&self) -> RxBiR {
        RxBiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_fifo_e(&self) -> TxFifoER {
        TxFifoER::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tx_sr_e(&self) -> TxSrER {
        TxSrER::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_fifo_sts(&self) -> RxFifoStsR {
        RxFifoStsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_e(&mut self) -> RxFifoEW<MemLsrUartSpec> {
        RxFifoEW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx_oe(&mut self) -> RxOeW<MemLsrUartSpec> {
        RxOeW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pe(&mut self) -> RxPeW<MemLsrUartSpec> {
        RxPeW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fe(&mut self) -> RxFeW<MemLsrUartSpec> {
        RxFeW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rx_bi(&mut self) -> RxBiW<MemLsrUartSpec> {
        RxBiW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_e(&mut self) -> TxFifoEW<MemLsrUartSpec> {
        TxFifoEW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tx_sr_e(&mut self) -> TxSrEW<MemLsrUartSpec> {
        TxSrEW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_sts(&mut self) -> RxFifoStsW<MemLsrUartSpec> {
        RxFifoStsW::new(self, 7)
    }
}
#[doc = "MEM_LSR_UART\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_lsr_uart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_lsr_uart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemLsrUartSpec;
impl crate::RegisterSpec for MemLsrUartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_lsr_uart::R`](R) reader structure"]
impl crate::Readable for MemLsrUartSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_lsr_uart::W`](W) writer structure"]
impl crate::Writable for MemLsrUartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_LSR_UART to value 0x60"]
impl crate::Resettable for MemLsrUartSpec {
    const RESET_VALUE: u32 = 0x60;
}
