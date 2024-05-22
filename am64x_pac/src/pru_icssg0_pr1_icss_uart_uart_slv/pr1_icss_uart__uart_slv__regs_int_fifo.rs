#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_INT_FIFO` reader"]
pub type R = crate::R<Pr1IcssUart_UartSlv_RegsIntFifoSpec>;
#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_INT_FIFO` writer"]
pub type W = crate::W<Pr1IcssUart_UartSlv_RegsIntFifoSpec>;
#[doc = "Field `IIR_IPEND` reader - 0:0\\]
Receiver Data Available Interrupt Pending"]
pub type IirIpendR = crate::BitReader;
#[doc = "Field `IIR_IPEND` writer - 0:0\\]
Receiver Data Available Interrupt Pending"]
pub type IirIpendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IIR_INTID` reader - 3:1\\]
Interrupt Type"]
pub type IirIntidR = crate::FieldReader;
#[doc = "Field `IIR_INTID` writer - 3:1\\]
Interrupt Type"]
pub type IirIntidW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IIR_FIFOEN` reader - 7:6\\]
FIFOs enabled"]
pub type IirFifoenR = crate::FieldReader;
#[doc = "Field `IIR_FIFOEN` writer - 7:6\\]
FIFOs enabled"]
pub type IirFifoenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FCR_FIFOEN` reader - 8:8\\]
FIFO Enable Register"]
pub type FcrFifoenR = crate::BitReader;
#[doc = "Field `FCR_FIFOEN` writer - 8:8\\]
FIFO Enable Register"]
pub type FcrFifoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCR_RXCLR` reader - 9:9\\]
Receiver FIFO Reset"]
pub type FcrRxclrR = crate::BitReader;
#[doc = "Field `FCR_RXCLR` writer - 9:9\\]
Receiver FIFO Reset"]
pub type FcrRxclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCR_TXCLR` reader - 10:10\\]
Transmitter FIFO Reset"]
pub type FcrTxclrR = crate::BitReader;
#[doc = "Field `FCR_TXCLR` writer - 10:10\\]
Transmitter FIFO Reset"]
pub type FcrTxclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCR_DMAMODE1` reader - 11:11\\]
DMA Mode Select"]
pub type FcrDmamode1R = crate::BitReader;
#[doc = "Field `FCR_DMAMODE1` writer - 11:11\\]
DMA Mode Select"]
pub type FcrDmamode1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCR_RXFIFTL` reader - 15:14\\]
Receiver Trigger Level"]
pub type FcrRxfiftlR = crate::FieldReader;
#[doc = "Field `FCR_RXFIFTL` writer - 15:14\\]
Receiver Trigger Level"]
pub type FcrRxfiftlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receiver Data Available Interrupt Pending"]
    #[inline(always)]
    pub fn iir_ipend(&self) -> IirIpendR {
        IirIpendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Interrupt Type"]
    #[inline(always)]
    pub fn iir_intid(&self) -> IirIntidR {
        IirIntidR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
FIFOs enabled"]
    #[inline(always)]
    pub fn iir_fifoen(&self) -> IirFifoenR {
        IirFifoenR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
FIFO Enable Register"]
    #[inline(always)]
    pub fn fcr_fifoen(&self) -> FcrFifoenR {
        FcrFifoenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Receiver FIFO Reset"]
    #[inline(always)]
    pub fn fcr_rxclr(&self) -> FcrRxclrR {
        FcrRxclrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmitter FIFO Reset"]
    #[inline(always)]
    pub fn fcr_txclr(&self) -> FcrTxclrR {
        FcrTxclrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
DMA Mode Select"]
    #[inline(always)]
    pub fn fcr_dmamode1(&self) -> FcrDmamode1R {
        FcrDmamode1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Receiver Trigger Level"]
    #[inline(always)]
    pub fn fcr_rxfiftl(&self) -> FcrRxfiftlR {
        FcrRxfiftlR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receiver Data Available Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn iir_ipend(&mut self) -> IirIpendW<Pr1IcssUart_UartSlv_RegsIntFifoSpec> {
        IirIpendW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Interrupt Type"]
    #[inline(always)]
    #[must_use]
    pub fn iir_intid(&mut self) -> IirIntidW<Pr1IcssUart_UartSlv_RegsIntFifoSpec> {
        IirIntidW::new(self, 1)
    }
    #[doc = "Bits 6:7 - 7:6\\]
FIFOs enabled"]
    #[inline(always)]
    #[must_use]
    pub fn iir_fifoen(&mut self) -> IirFifoenW<Pr1IcssUart_UartSlv_RegsIntFifoSpec> {
        IirFifoenW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
FIFO Enable Register"]
    #[inline(always)]
    #[must_use]
    pub fn fcr_fifoen(&mut self) -> FcrFifoenW<Pr1IcssUart_UartSlv_RegsIntFifoSpec> {
        FcrFifoenW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Receiver FIFO Reset"]
    #[inline(always)]
    #[must_use]
    pub fn fcr_rxclr(&mut self) -> FcrRxclrW<Pr1IcssUart_UartSlv_RegsIntFifoSpec> {
        FcrRxclrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmitter FIFO Reset"]
    #[inline(always)]
    #[must_use]
    pub fn fcr_txclr(&mut self) -> FcrTxclrW<Pr1IcssUart_UartSlv_RegsIntFifoSpec> {
        FcrTxclrW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
DMA Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn fcr_dmamode1(&mut self) -> FcrDmamode1W<Pr1IcssUart_UartSlv_RegsIntFifoSpec> {
        FcrDmamode1W::new(self, 11)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Receiver Trigger Level"]
    #[inline(always)]
    #[must_use]
    pub fn fcr_rxfiftl(&mut self) -> FcrRxfiftlW<Pr1IcssUart_UartSlv_RegsIntFifoSpec> {
        FcrRxfiftlW::new(self, 14)
    }
}
#[doc = "Interrupt Identification Register / FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_int_fifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_int_fifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssUart_UartSlv_RegsIntFifoSpec;
impl crate::RegisterSpec for Pr1IcssUart_UartSlv_RegsIntFifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_uart__uart_slv__regs_int_fifo::R`](R) reader structure"]
impl crate::Readable for Pr1IcssUart_UartSlv_RegsIntFifoSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_uart__uart_slv__regs_int_fifo::W`](W) writer structure"]
impl crate::Writable for Pr1IcssUart_UartSlv_RegsIntFifoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_UART__UART_SLV__REGS_INT_FIFO to value 0x01"]
impl crate::Resettable for Pr1IcssUart_UartSlv_RegsIntFifoSpec {
    const RESET_VALUE: u32 = 0x01;
}
