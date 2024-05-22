#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_DIVLSB` reader"]
pub type R = crate::R<Pr1IcssUart_UartSlv_RegsDivlsbSpec>;
#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_DIVLSB` writer"]
pub type W = crate::W<Pr1IcssUart_UartSlv_RegsDivlsbSpec>;
#[doc = "Field `DLL` reader - 7:0\\]
Divisor Latch \\[LSB\\]"]
pub type DllR = crate::FieldReader;
#[doc = "Field `DLL` writer - 7:0\\]
Divisor Latch \\[LSB\\]"]
pub type DllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Divisor Latch \\[LSB\\]"]
    #[inline(always)]
    pub fn dll(&self) -> DllR {
        DllR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Divisor Latch \\[LSB\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dll(&mut self) -> DllW<Pr1IcssUart_UartSlv_RegsDivlsbSpec> {
        DllW::new(self, 0)
    }
}
#[doc = "UART Divisor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_divlsb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_divlsb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssUart_UartSlv_RegsDivlsbSpec;
impl crate::RegisterSpec for Pr1IcssUart_UartSlv_RegsDivlsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_uart__uart_slv__regs_divlsb::R`](R) reader structure"]
impl crate::Readable for Pr1IcssUart_UartSlv_RegsDivlsbSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_uart__uart_slv__regs_divlsb::W`](W) writer structure"]
impl crate::Writable for Pr1IcssUart_UartSlv_RegsDivlsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_UART__UART_SLV__REGS_DIVLSB to value 0"]
impl crate::Resettable for Pr1IcssUart_UartSlv_RegsDivlsbSpec {
    const RESET_VALUE: u32 = 0;
}
