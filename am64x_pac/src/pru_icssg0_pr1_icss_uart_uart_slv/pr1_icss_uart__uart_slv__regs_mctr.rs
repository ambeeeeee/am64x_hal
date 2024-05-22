#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_MCTR` reader"]
pub type R = crate::R<Pr1IcssUart_UartSlv_RegsMctrSpec>;
#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_MCTR` writer"]
pub type W = crate::W<Pr1IcssUart_UartSlv_RegsMctrSpec>;
#[doc = "Field `DTR` reader - 0:0\\]
Data Terminal Ready"]
pub type DtrR = crate::BitReader;
#[doc = "Field `DTR` writer - 0:0\\]
Data Terminal Ready"]
pub type DtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS` reader - 1:1\\]
Ready to Send"]
pub type RtsR = crate::BitReader;
#[doc = "Field `RTS` writer - 1:1\\]
Ready to Send"]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT1` reader - 2:2\\]
Out1 Bit"]
pub type Out1R = crate::BitReader;
#[doc = "Field `OUT1` writer - 2:2\\]
Out1 Bit"]
pub type Out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT2` reader - 3:3\\]
Out2 Bit"]
pub type Out2R = crate::BitReader;
#[doc = "Field `OUT2` writer - 3:3\\]
Out2 Bit"]
pub type Out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOP` reader - 4:4\\]
LOOP Bit"]
pub type LoopR = crate::BitReader;
#[doc = "Field `LOOP` writer - 4:4\\]
LOOP Bit"]
pub type LoopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFE` reader - 5:5\\]
Autoflow Control Enable"]
pub type AfeR = crate::BitReader;
#[doc = "Field `AFE` writer - 5:5\\]
Autoflow Control Enable"]
pub type AfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data Terminal Ready"]
    #[inline(always)]
    pub fn dtr(&self) -> DtrR {
        DtrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Ready to Send"]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Out1 Bit"]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Out2 Bit"]
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
LOOP Bit"]
    #[inline(always)]
    pub fn loop_(&self) -> LoopR {
        LoopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Autoflow Control Enable"]
    #[inline(always)]
    pub fn afe(&self) -> AfeR {
        AfeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data Terminal Ready"]
    #[inline(always)]
    #[must_use]
    pub fn dtr(&mut self) -> DtrW<Pr1IcssUart_UartSlv_RegsMctrSpec> {
        DtrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Ready to Send"]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RtsW<Pr1IcssUart_UartSlv_RegsMctrSpec> {
        RtsW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Out1 Bit"]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> Out1W<Pr1IcssUart_UartSlv_RegsMctrSpec> {
        Out1W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Out2 Bit"]
    #[inline(always)]
    #[must_use]
    pub fn out2(&mut self) -> Out2W<Pr1IcssUart_UartSlv_RegsMctrSpec> {
        Out2W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
LOOP Bit"]
    #[inline(always)]
    #[must_use]
    pub fn loop_(&mut self) -> LoopW<Pr1IcssUart_UartSlv_RegsMctrSpec> {
        LoopW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Autoflow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn afe(&mut self) -> AfeW<Pr1IcssUart_UartSlv_RegsMctrSpec> {
        AfeW::new(self, 5)
    }
}
#[doc = "Modem Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_mctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_mctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssUart_UartSlv_RegsMctrSpec;
impl crate::RegisterSpec for Pr1IcssUart_UartSlv_RegsMctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_uart__uart_slv__regs_mctr::R`](R) reader structure"]
impl crate::Readable for Pr1IcssUart_UartSlv_RegsMctrSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_uart__uart_slv__regs_mctr::W`](W) writer structure"]
impl crate::Writable for Pr1IcssUart_UartSlv_RegsMctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_UART__UART_SLV__REGS_MCTR to value 0"]
impl crate::Resettable for Pr1IcssUart_UartSlv_RegsMctrSpec {
    const RESET_VALUE: u32 = 0;
}
