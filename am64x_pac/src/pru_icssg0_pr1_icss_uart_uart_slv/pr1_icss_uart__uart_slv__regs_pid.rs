#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_PID` reader"]
pub type R = crate::R<Pr1IcssUart_UartSlv_RegsPidSpec>;
#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_PID` writer"]
pub type W = crate::W<Pr1IcssUart_UartSlv_RegsPidSpec>;
#[doc = "Field `PID` reader - "]
pub type PidR = crate::FieldReader<u32>;
#[doc = "Field `PID` writer - "]
pub type PidW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PidW<Pr1IcssUart_UartSlv_RegsPidSpec> {
        PidW::new(self, 0)
    }
}
#[doc = "Peripheral ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_pid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_pid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssUart_UartSlv_RegsPidSpec;
impl crate::RegisterSpec for Pr1IcssUart_UartSlv_RegsPidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_uart__uart_slv__regs_pid::R`](R) reader structure"]
impl crate::Readable for Pr1IcssUart_UartSlv_RegsPidSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_uart__uart_slv__regs_pid::W`](W) writer structure"]
impl crate::Writable for Pr1IcssUart_UartSlv_RegsPidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_UART__UART_SLV__REGS_PID to value 0"]
impl crate::Resettable for Pr1IcssUart_UartSlv_RegsPidSpec {
    const RESET_VALUE: u32 = 0;
}
