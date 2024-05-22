#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_SCRATCH` reader"]
pub type R = crate::R<Pr1IcssUart_UartSlv_RegsScratchSpec>;
#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_SCRATCH` writer"]
pub type W = crate::W<Pr1IcssUart_UartSlv_RegsScratchSpec>;
#[doc = "Field `DATA` reader - 7:0\\]
Scratch Register Bits"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - 7:0\\]
Scratch Register Bits"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Scratch Register Bits"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Scratch Register Bits"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Pr1IcssUart_UartSlv_RegsScratchSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "UART Scratch Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_scratch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_scratch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssUart_UartSlv_RegsScratchSpec;
impl crate::RegisterSpec for Pr1IcssUart_UartSlv_RegsScratchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_uart__uart_slv__regs_scratch::R`](R) reader structure"]
impl crate::Readable for Pr1IcssUart_UartSlv_RegsScratchSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_uart__uart_slv__regs_scratch::W`](W) writer structure"]
impl crate::Writable for Pr1IcssUart_UartSlv_RegsScratchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_UART__UART_SLV__REGS_SCRATCH to value 0"]
impl crate::Resettable for Pr1IcssUart_UartSlv_RegsScratchSpec {
    const RESET_VALUE: u32 = 0;
}
