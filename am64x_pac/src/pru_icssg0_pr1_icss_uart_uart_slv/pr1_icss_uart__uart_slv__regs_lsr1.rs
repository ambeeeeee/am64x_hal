#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_LSR1` reader"]
pub type R = crate::R<Pr1IcssUart_UartSlv_RegsLsr1Spec>;
#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_LSR1` writer"]
pub type W = crate::W<Pr1IcssUart_UartSlv_RegsLsr1Spec>;
#[doc = "Field `DR` reader - 0:0\\]
Data Ready"]
pub type DrR = crate::BitReader;
#[doc = "Field `DR` writer - 0:0\\]
Data Ready"]
pub type DrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` reader - 1:1\\]
Overrun Error"]
pub type OeR = crate::BitReader;
#[doc = "Field `OE` writer - 1:1\\]
Overrun Error"]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - 2:2\\]
Parity Error"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - 2:2\\]
Parity Error"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FE` reader - 3:3\\]
Framing Error"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - 3:3\\]
Framing Error"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BI` reader - 4:4\\]
Break Interrupt"]
pub type BiR = crate::BitReader;
#[doc = "Field `BI` writer - 4:4\\]
Break Interrupt"]
pub type BiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRE` reader - 5:5\\]
Transmitter Holding Register"]
pub type ThreR = crate::BitReader;
#[doc = "Field `THRE` writer - 5:5\\]
Transmitter Holding Register"]
pub type ThreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMT` reader - 6:6\\]
Transmitter Empty"]
pub type TemtR = crate::BitReader;
#[doc = "Field `TEMT` writer - 6:6\\]
Transmitter Empty"]
pub type TemtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOE` reader - 7:7\\]
Receiver FIFO Error"]
pub type RxfifoeR = crate::BitReader;
#[doc = "Field `RXFIFOE` writer - 7:7\\]
Receiver FIFO Error"]
pub type RxfifoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data Ready"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Overrun Error"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Parity Error"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Framing Error"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Break Interrupt"]
    #[inline(always)]
    pub fn bi(&self) -> BiR {
        BiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmitter Holding Register"]
    #[inline(always)]
    pub fn thre(&self) -> ThreR {
        ThreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Transmitter Empty"]
    #[inline(always)]
    pub fn temt(&self) -> TemtR {
        TemtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Receiver FIFO Error"]
    #[inline(always)]
    pub fn rxfifoe(&self) -> RxfifoeR {
        RxfifoeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data Ready"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DrW<Pr1IcssUart_UartSlv_RegsLsr1Spec> {
        DrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Overrun Error"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OeW<Pr1IcssUart_UartSlv_RegsLsr1Spec> {
        OeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Parity Error"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<Pr1IcssUart_UartSlv_RegsLsr1Spec> {
        PeW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Framing Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FeW<Pr1IcssUart_UartSlv_RegsLsr1Spec> {
        FeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Break Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bi(&mut self) -> BiW<Pr1IcssUart_UartSlv_RegsLsr1Spec> {
        BiW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmitter Holding Register"]
    #[inline(always)]
    #[must_use]
    pub fn thre(&mut self) -> ThreW<Pr1IcssUart_UartSlv_RegsLsr1Spec> {
        ThreW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Transmitter Empty"]
    #[inline(always)]
    #[must_use]
    pub fn temt(&mut self) -> TemtW<Pr1IcssUart_UartSlv_RegsLsr1Spec> {
        TemtW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Receiver FIFO Error"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifoe(&mut self) -> RxfifoeW<Pr1IcssUart_UartSlv_RegsLsr1Spec> {
        RxfifoeW::new(self, 7)
    }
}
#[doc = "Line Status Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_lsr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_lsr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssUart_UartSlv_RegsLsr1Spec;
impl crate::RegisterSpec for Pr1IcssUart_UartSlv_RegsLsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_uart__uart_slv__regs_lsr1::R`](R) reader structure"]
impl crate::Readable for Pr1IcssUart_UartSlv_RegsLsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_uart__uart_slv__regs_lsr1::W`](W) writer structure"]
impl crate::Writable for Pr1IcssUart_UartSlv_RegsLsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_UART__UART_SLV__REGS_LSR1 to value 0x60"]
impl crate::Resettable for Pr1IcssUart_UartSlv_RegsLsr1Spec {
    const RESET_VALUE: u32 = 0x60;
}
