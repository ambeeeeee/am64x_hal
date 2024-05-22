#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_RBR_TBR` reader"]
pub type R = crate::R<Pr1IcssUart_UartSlv_RegsRbrTbrSpec>;
#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_RBR_TBR` writer"]
pub type W = crate::W<Pr1IcssUart_UartSlv_RegsRbrTbrSpec>;
#[doc = "Field `RBR_DATA` reader - 7:0\\]
Receive Buffer Register"]
pub type RbrDataR = crate::FieldReader;
#[doc = "Field `RBR_DATA` writer - 7:0\\]
Receive Buffer Register"]
pub type RbrDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TBR_DATA` reader - 17:8\\]
Transmit Buffer Register"]
pub type TbrDataR = crate::FieldReader<u16>;
#[doc = "Field `TBR_DATA` writer - 17:8\\]
Transmit Buffer Register"]
pub type TbrDataW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Receive Buffer Register"]
    #[inline(always)]
    pub fn rbr_data(&self) -> RbrDataR {
        RbrDataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Transmit Buffer Register"]
    #[inline(always)]
    pub fn tbr_data(&self) -> TbrDataR {
        TbrDataR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Receive Buffer Register"]
    #[inline(always)]
    #[must_use]
    pub fn rbr_data(&mut self) -> RbrDataW<Pr1IcssUart_UartSlv_RegsRbrTbrSpec> {
        RbrDataW::new(self, 0)
    }
    #[doc = "Bits 8:17 - 17:8\\]
Transmit Buffer Register"]
    #[inline(always)]
    #[must_use]
    pub fn tbr_data(&mut self) -> TbrDataW<Pr1IcssUart_UartSlv_RegsRbrTbrSpec> {
        TbrDataW::new(self, 8)
    }
}
#[doc = "RBR_TBR Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_rbr_tbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_rbr_tbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssUart_UartSlv_RegsRbrTbrSpec;
impl crate::RegisterSpec for Pr1IcssUart_UartSlv_RegsRbrTbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_uart__uart_slv__regs_rbr_tbr::R`](R) reader structure"]
impl crate::Readable for Pr1IcssUart_UartSlv_RegsRbrTbrSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_uart__uart_slv__regs_rbr_tbr::W`](W) writer structure"]
impl crate::Writable for Pr1IcssUart_UartSlv_RegsRbrTbrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_UART__UART_SLV__REGS_RBR_TBR to value 0"]
impl crate::Resettable for Pr1IcssUart_UartSlv_RegsRbrTbrSpec {
    const RESET_VALUE: u32 = 0;
}
