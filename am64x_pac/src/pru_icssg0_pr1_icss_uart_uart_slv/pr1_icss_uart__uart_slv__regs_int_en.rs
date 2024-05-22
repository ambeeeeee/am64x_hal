#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_INT_EN` reader"]
pub type R = crate::R<Pr1IcssUart_UartSlv_RegsIntEnSpec>;
#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_INT_EN` writer"]
pub type W = crate::W<Pr1IcssUart_UartSlv_RegsIntEnSpec>;
#[doc = "Field `ERBI` reader - 0:0\\]
Enable for Receiver Data Available Interrupt"]
pub type ErbiR = crate::BitReader;
#[doc = "Field `ERBI` writer - 0:0\\]
Enable for Receiver Data Available Interrupt"]
pub type ErbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETBEI` reader - 1:1\\]
Enable for Transmitter Holding Register Empty Interrupt"]
pub type EtbeiR = crate::BitReader;
#[doc = "Field `ETBEI` writer - 1:1\\]
Enable for Transmitter Holding Register Empty Interrupt"]
pub type EtbeiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELSI` reader - 2:2\\]
Enable for Receiver Line Status Interrupt"]
pub type ElsiR = crate::BitReader;
#[doc = "Field `ELSI` writer - 2:2\\]
Enable for Receiver Line Status Interrupt"]
pub type ElsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDSSI` reader - 3:3\\]
Enable for Modem Status Interrupt"]
pub type EdssiR = crate::BitReader;
#[doc = "Field `EDSSI` writer - 3:3\\]
Enable for Modem Status Interrupt"]
pub type EdssiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable for Receiver Data Available Interrupt"]
    #[inline(always)]
    pub fn erbi(&self) -> ErbiR {
        ErbiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable for Transmitter Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn etbei(&self) -> EtbeiR {
        EtbeiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable for Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn elsi(&self) -> ElsiR {
        ElsiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable for Modem Status Interrupt"]
    #[inline(always)]
    pub fn edssi(&self) -> EdssiR {
        EdssiR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable for Receiver Data Available Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn erbi(&mut self) -> ErbiW<Pr1IcssUart_UartSlv_RegsIntEnSpec> {
        ErbiW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable for Transmitter Holding Register Empty Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn etbei(&mut self) -> EtbeiW<Pr1IcssUart_UartSlv_RegsIntEnSpec> {
        EtbeiW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable for Receiver Line Status Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn elsi(&mut self) -> ElsiW<Pr1IcssUart_UartSlv_RegsIntEnSpec> {
        ElsiW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable for Modem Status Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn edssi(&mut self) -> EdssiW<Pr1IcssUart_UartSlv_RegsIntEnSpec> {
        EdssiW::new(self, 3)
    }
}
#[doc = "UART Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssUart_UartSlv_RegsIntEnSpec;
impl crate::RegisterSpec for Pr1IcssUart_UartSlv_RegsIntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_uart__uart_slv__regs_int_en::R`](R) reader structure"]
impl crate::Readable for Pr1IcssUart_UartSlv_RegsIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_uart__uart_slv__regs_int_en::W`](W) writer structure"]
impl crate::Writable for Pr1IcssUart_UartSlv_RegsIntEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_UART__UART_SLV__REGS_INT_EN to value 0"]
impl crate::Resettable for Pr1IcssUart_UartSlv_RegsIntEnSpec {
    const RESET_VALUE: u32 = 0;
}
