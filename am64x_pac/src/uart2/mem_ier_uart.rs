#[doc = "Register `MEM_IER_UART` reader"]
pub type R = crate::R<MemIerUartSpec>;
#[doc = "Register `MEM_IER_UART` writer"]
pub type W = crate::W<MemIerUartSpec>;
#[doc = "Field `RHR_IT` reader - "]
pub type RhrItR = crate::BitReader;
#[doc = "Field `RHR_IT` writer - "]
pub type RhrItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THR_IT` reader - "]
pub type ThrItR = crate::BitReader;
#[doc = "Field `THR_IT` writer - "]
pub type ThrItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE_STS_IT` reader - "]
pub type LineStsItR = crate::BitReader;
#[doc = "Field `LINE_STS_IT` writer - "]
pub type LineStsItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_STS_IT` reader - "]
pub type ModemStsItR = crate::BitReader;
#[doc = "Field `MODEM_STS_IT` writer - "]
pub type ModemStsItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_MODE` reader - "]
pub type SleepModeR = crate::BitReader;
#[doc = "Field `SLEEP_MODE` writer - "]
pub type SleepModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOFF_IT` reader - "]
pub type XoffItR = crate::BitReader;
#[doc = "Field `XOFF_IT` writer - "]
pub type XoffItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS_IT` reader - "]
pub type RtsItR = crate::BitReader;
#[doc = "Field `RTS_IT` writer - "]
pub type RtsItW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_IT` reader - "]
pub type CtsItR = crate::BitReader;
#[doc = "Field `CTS_IT` writer - "]
pub type CtsItW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rhr_it(&self) -> RhrItR {
        RhrItR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn thr_it(&self) -> ThrItR {
        ThrItR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn line_sts_it(&self) -> LineStsItR {
        LineStsItR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn modem_sts_it(&self) -> ModemStsItR {
        ModemStsItR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sleep_mode(&self) -> SleepModeR {
        SleepModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn xoff_it(&self) -> XoffItR {
        XoffItR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rts_it(&self) -> RtsItR {
        RtsItR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cts_it(&self) -> CtsItR {
        CtsItR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rhr_it(&mut self) -> RhrItW<MemIerUartSpec> {
        RhrItW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn thr_it(&mut self) -> ThrItW<MemIerUartSpec> {
        ThrItW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn line_sts_it(&mut self) -> LineStsItW<MemIerUartSpec> {
        LineStsItW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn modem_sts_it(&mut self) -> ModemStsItW<MemIerUartSpec> {
        ModemStsItW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_mode(&mut self) -> SleepModeW<MemIerUartSpec> {
        SleepModeW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn xoff_it(&mut self) -> XoffItW<MemIerUartSpec> {
        XoffItW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rts_it(&mut self) -> RtsItW<MemIerUartSpec> {
        RtsItW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cts_it(&mut self) -> CtsItW<MemIerUartSpec> {
        CtsItW::new(self, 7)
    }
}
#[doc = "The interrupt enable register (IER) can be programmed to enable/disable any interrupt. There are seven types of interrupt in this mode: receiver error, RHR interrupt, THR interrupt, XOFF received and CTS*/RTS* change of state from low to high. Each interrupt can be enabled/disabled individually. There is also a sleep mode enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ier_uart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ier_uart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemIerUartSpec;
impl crate::RegisterSpec for MemIerUartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ier_uart::R`](R) reader structure"]
impl crate::Readable for MemIerUartSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_ier_uart::W`](W) writer structure"]
impl crate::Writable for MemIerUartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_IER_UART to value 0"]
impl crate::Resettable for MemIerUartSpec {
    const RESET_VALUE: u32 = 0;
}
