#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_MSR` reader"]
pub type R = crate::R<Pr1IcssUart_UartSlv_RegsMsrSpec>;
#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_MSR` writer"]
pub type W = crate::W<Pr1IcssUart_UartSlv_RegsMsrSpec>;
#[doc = "Field `DCTS` reader - 0:0\\]
Delta Clear To Send"]
pub type DctsR = crate::BitReader;
#[doc = "Field `DCTS` writer - 0:0\\]
Delta Clear To Send"]
pub type DctsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDSR` reader - 1:1\\]
Delta Set Ready"]
pub type DdsrR = crate::BitReader;
#[doc = "Field `DDSR` writer - 1:1\\]
Delta Set Ready"]
pub type DdsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERI` reader - 2:2\\]
Trailing Edge Ring Indicator"]
pub type TeriR = crate::BitReader;
#[doc = "Field `TERI` writer - 2:2\\]
Trailing Edge Ring Indicator"]
pub type TeriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCD` reader - 3:3\\]
Delta Carrier Detect"]
pub type DcdR = crate::BitReader;
#[doc = "Field `DCD` writer - 3:3\\]
Delta Carrier Detect"]
pub type DcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS` reader - 4:4\\]
Clear To Send"]
pub type CtsR = crate::BitReader;
#[doc = "Field `CTS` writer - 4:4\\]
Clear To Send"]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR` reader - 5:5\\]
Data Set Ready"]
pub type DsrR = crate::BitReader;
#[doc = "Field `DSR` writer - 5:5\\]
Data Set Ready"]
pub type DsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - 6:6\\]
Ring Indicator"]
pub type RiR = crate::BitReader;
#[doc = "Field `RI` writer - 6:6\\]
Ring Indicator"]
pub type RiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD` reader - 7:7\\]
Carrier Detect"]
pub type CdR = crate::BitReader;
#[doc = "Field `CD` writer - 7:7\\]
Carrier Detect"]
pub type CdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Delta Clear To Send"]
    #[inline(always)]
    pub fn dcts(&self) -> DctsR {
        DctsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Delta Set Ready"]
    #[inline(always)]
    pub fn ddsr(&self) -> DdsrR {
        DdsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Trailing Edge Ring Indicator"]
    #[inline(always)]
    pub fn teri(&self) -> TeriR {
        TeriR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Delta Carrier Detect"]
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear To Send"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Data Set Ready"]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Ring Indicator"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Carrier Detect"]
    #[inline(always)]
    pub fn cd(&self) -> CdR {
        CdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Delta Clear To Send"]
    #[inline(always)]
    #[must_use]
    pub fn dcts(&mut self) -> DctsW<Pr1IcssUart_UartSlv_RegsMsrSpec> {
        DctsW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Delta Set Ready"]
    #[inline(always)]
    #[must_use]
    pub fn ddsr(&mut self) -> DdsrW<Pr1IcssUart_UartSlv_RegsMsrSpec> {
        DdsrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Trailing Edge Ring Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn teri(&mut self) -> TeriW<Pr1IcssUart_UartSlv_RegsMsrSpec> {
        TeriW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Delta Carrier Detect"]
    #[inline(always)]
    #[must_use]
    pub fn dcd(&mut self) -> DcdW<Pr1IcssUart_UartSlv_RegsMsrSpec> {
        DcdW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear To Send"]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CtsW<Pr1IcssUart_UartSlv_RegsMsrSpec> {
        CtsW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Data Set Ready"]
    #[inline(always)]
    #[must_use]
    pub fn dsr(&mut self) -> DsrW<Pr1IcssUart_UartSlv_RegsMsrSpec> {
        DsrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Ring Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RiW<Pr1IcssUart_UartSlv_RegsMsrSpec> {
        RiW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Carrier Detect"]
    #[inline(always)]
    #[must_use]
    pub fn cd(&mut self) -> CdW<Pr1IcssUart_UartSlv_RegsMsrSpec> {
        CdW::new(self, 7)
    }
}
#[doc = "Modem Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_msr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_msr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssUart_UartSlv_RegsMsrSpec;
impl crate::RegisterSpec for Pr1IcssUart_UartSlv_RegsMsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_uart__uart_slv__regs_msr::R`](R) reader structure"]
impl crate::Readable for Pr1IcssUart_UartSlv_RegsMsrSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_uart__uart_slv__regs_msr::W`](W) writer structure"]
impl crate::Writable for Pr1IcssUart_UartSlv_RegsMsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_UART__UART_SLV__REGS_MSR to value 0"]
impl crate::Resettable for Pr1IcssUart_UartSlv_RegsMsrSpec {
    const RESET_VALUE: u32 = 0;
}
