#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_PWR` reader"]
pub type R = crate::R<Pr1IcssUart_UartSlv_RegsPwrSpec>;
#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_PWR` writer"]
pub type W = crate::W<Pr1IcssUart_UartSlv_RegsPwrSpec>;
#[doc = "Field `FREE` reader - 0:0\\]
Free Bit"]
pub type FreeR = crate::BitReader;
#[doc = "Field `FREE` writer - 0:0\\]
Free Bit"]
pub type FreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES` reader - 1:1\\]
Free Bit"]
pub type ResR = crate::BitReader;
#[doc = "Field `RES` writer - 1:1\\]
Free Bit"]
pub type ResW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URRST` reader - 13:13\\]
UART Receiver Reset Bit"]
pub type UrrstR = crate::BitReader;
#[doc = "Field `URRST` writer - 13:13\\]
UART Receiver Reset Bit"]
pub type UrrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTRST` reader - 14:14\\]
UART Transmitter Reset Bit"]
pub type UtrstR = crate::BitReader;
#[doc = "Field `UTRST` writer - 14:14\\]
UART Transmitter Reset Bit"]
pub type UtrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URST` reader - 15:15\\]
UART Reset Bit"]
pub type UrstR = crate::BitReader;
#[doc = "Field `URST` writer - 15:15\\]
UART Reset Bit"]
pub type UrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Free Bit"]
    #[inline(always)]
    pub fn free(&self) -> FreeR {
        FreeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Free Bit"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
UART Receiver Reset Bit"]
    #[inline(always)]
    pub fn urrst(&self) -> UrrstR {
        UrrstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
UART Transmitter Reset Bit"]
    #[inline(always)]
    pub fn utrst(&self) -> UtrstR {
        UtrstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
UART Reset Bit"]
    #[inline(always)]
    pub fn urst(&self) -> UrstR {
        UrstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Free Bit"]
    #[inline(always)]
    #[must_use]
    pub fn free(&mut self) -> FreeW<Pr1IcssUart_UartSlv_RegsPwrSpec> {
        FreeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Free Bit"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> ResW<Pr1IcssUart_UartSlv_RegsPwrSpec> {
        ResW::new(self, 1)
    }
    #[doc = "Bit 13 - 13:13\\]
UART Receiver Reset Bit"]
    #[inline(always)]
    #[must_use]
    pub fn urrst(&mut self) -> UrrstW<Pr1IcssUart_UartSlv_RegsPwrSpec> {
        UrrstW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
UART Transmitter Reset Bit"]
    #[inline(always)]
    #[must_use]
    pub fn utrst(&mut self) -> UtrstW<Pr1IcssUart_UartSlv_RegsPwrSpec> {
        UtrstW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
UART Reset Bit"]
    #[inline(always)]
    #[must_use]
    pub fn urst(&mut self) -> UrstW<Pr1IcssUart_UartSlv_RegsPwrSpec> {
        UrstW::new(self, 15)
    }
}
#[doc = "UART PowerManagement and Emulation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_pwr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_pwr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssUart_UartSlv_RegsPwrSpec;
impl crate::RegisterSpec for Pr1IcssUart_UartSlv_RegsPwrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_uart__uart_slv__regs_pwr::R`](R) reader structure"]
impl crate::Readable for Pr1IcssUart_UartSlv_RegsPwrSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_uart__uart_slv__regs_pwr::W`](W) writer structure"]
impl crate::Writable for Pr1IcssUart_UartSlv_RegsPwrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_UART__UART_SLV__REGS_PWR to value 0x02"]
impl crate::Resettable for Pr1IcssUart_UartSlv_RegsPwrSpec {
    const RESET_VALUE: u32 = 0x02;
}
