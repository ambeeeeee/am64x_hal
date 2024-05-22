#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_MODE` reader"]
pub type R = crate::R<Pr1IcssUart_UartSlv_RegsModeSpec>;
#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_MODE` writer"]
pub type W = crate::W<Pr1IcssUart_UartSlv_RegsModeSpec>;
#[doc = "Field `OSM_SEL` reader - 0:0\\]
Oversampling Mode Select"]
pub type OsmSelR = crate::BitReader;
#[doc = "Field `OSM_SEL` writer - 0:0\\]
Oversampling Mode Select"]
pub type OsmSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Oversampling Mode Select"]
    #[inline(always)]
    pub fn osm_sel(&self) -> OsmSelR {
        OsmSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Oversampling Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn osm_sel(&mut self) -> OsmSelW<Pr1IcssUart_UartSlv_RegsModeSpec> {
        OsmSelW::new(self, 0)
    }
}
#[doc = "UART Mode Definition Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssUart_UartSlv_RegsModeSpec;
impl crate::RegisterSpec for Pr1IcssUart_UartSlv_RegsModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_uart__uart_slv__regs_mode::R`](R) reader structure"]
impl crate::Readable for Pr1IcssUart_UartSlv_RegsModeSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_uart__uart_slv__regs_mode::W`](W) writer structure"]
impl crate::Writable for Pr1IcssUart_UartSlv_RegsModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_UART__UART_SLV__REGS_MODE to value 0"]
impl crate::Resettable for Pr1IcssUart_UartSlv_RegsModeSpec {
    const RESET_VALUE: u32 = 0;
}
