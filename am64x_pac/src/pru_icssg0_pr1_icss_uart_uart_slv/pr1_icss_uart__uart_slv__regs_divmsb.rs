#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_DIVMSB` reader"]
pub type R = crate::R<Pr1IcssUart_UartSlv_RegsDivmsbSpec>;
#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_DIVMSB` writer"]
pub type W = crate::W<Pr1IcssUart_UartSlv_RegsDivmsbSpec>;
#[doc = "Field `DLH` reader - 7:0\\]
Divisor Latch \\[MSB\\]"]
pub type DlhR = crate::FieldReader;
#[doc = "Field `DLH` writer - 7:0\\]
Divisor Latch \\[MSB\\]"]
pub type DlhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Divisor Latch \\[MSB\\]"]
    #[inline(always)]
    pub fn dlh(&self) -> DlhR {
        DlhR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Divisor Latch \\[MSB\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dlh(&mut self) -> DlhW<Pr1IcssUart_UartSlv_RegsDivmsbSpec> {
        DlhW::new(self, 0)
    }
}
#[doc = "UART Divisor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_divmsb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_divmsb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssUart_UartSlv_RegsDivmsbSpec;
impl crate::RegisterSpec for Pr1IcssUart_UartSlv_RegsDivmsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_uart__uart_slv__regs_divmsb::R`](R) reader structure"]
impl crate::Readable for Pr1IcssUart_UartSlv_RegsDivmsbSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_uart__uart_slv__regs_divmsb::W`](W) writer structure"]
impl crate::Writable for Pr1IcssUart_UartSlv_RegsDivmsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_UART__UART_SLV__REGS_DIVMSB to value 0"]
impl crate::Resettable for Pr1IcssUart_UartSlv_RegsDivmsbSpec {
    const RESET_VALUE: u32 = 0;
}
