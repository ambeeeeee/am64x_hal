#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_LCTR` reader"]
pub type R = crate::R<Pr1IcssUart_UartSlv_RegsLctrSpec>;
#[doc = "Register `PR1_ICSS_UART__UART_SLV__REGS_LCTR` writer"]
pub type W = crate::W<Pr1IcssUart_UartSlv_RegsLctrSpec>;
#[doc = "Field `WLS0` reader - 0:0\\]
Word Length Select Bit 0"]
pub type Wls0R = crate::BitReader;
#[doc = "Field `WLS0` writer - 0:0\\]
Word Length Select Bit 0"]
pub type Wls0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLS1` reader - 1:1\\]
Word Length Select Bit 1"]
pub type Wls1R = crate::BitReader;
#[doc = "Field `WLS1` writer - 1:1\\]
Word Length Select Bit 1"]
pub type Wls1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STB` reader - 2:2\\]
Number of Stop Bits"]
pub type StbR = crate::BitReader;
#[doc = "Field `STB` writer - 2:2\\]
Number of Stop Bits"]
pub type StbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - 3:3\\]
Parity Enable"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - 3:3\\]
Parity Enable"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPS` reader - 4:4\\]
Even Parity Select"]
pub type EpsR = crate::BitReader;
#[doc = "Field `EPS` writer - 4:4\\]
Even Parity Select"]
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP` reader - 5:5\\]
Stick Parity"]
pub type SpR = crate::BitReader;
#[doc = "Field `SP` writer - 5:5\\]
Stick Parity"]
pub type SpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BC` reader - 6:6\\]
Break Control"]
pub type BcR = crate::BitReader;
#[doc = "Field `BC` writer - 6:6\\]
Break Control"]
pub type BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLAB` reader - 7:7\\]
Divisor Latch Access Bit"]
pub type DlabR = crate::BitReader;
#[doc = "Field `DLAB` writer - 7:7\\]
Divisor Latch Access Bit"]
pub type DlabW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Word Length Select Bit 0"]
    #[inline(always)]
    pub fn wls0(&self) -> Wls0R {
        Wls0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Word Length Select Bit 1"]
    #[inline(always)]
    pub fn wls1(&self) -> Wls1R {
        Wls1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Number of Stop Bits"]
    #[inline(always)]
    pub fn stb(&self) -> StbR {
        StbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Parity Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Even Parity Select"]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Stick Parity"]
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Break Control"]
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&self) -> DlabR {
        DlabR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Word Length Select Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wls0(&mut self) -> Wls0W<Pr1IcssUart_UartSlv_RegsLctrSpec> {
        Wls0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Word Length Select Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn wls1(&mut self) -> Wls1W<Pr1IcssUart_UartSlv_RegsLctrSpec> {
        Wls1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Number of Stop Bits"]
    #[inline(always)]
    #[must_use]
    pub fn stb(&mut self) -> StbW<Pr1IcssUart_UartSlv_RegsLctrSpec> {
        StbW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Parity Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<Pr1IcssUart_UartSlv_RegsLctrSpec> {
        PenW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Even Parity Select"]
    #[inline(always)]
    #[must_use]
    pub fn eps(&mut self) -> EpsW<Pr1IcssUart_UartSlv_RegsLctrSpec> {
        EpsW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Stick Parity"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SpW<Pr1IcssUart_UartSlv_RegsLctrSpec> {
        SpW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Break Control"]
    #[inline(always)]
    #[must_use]
    pub fn bc(&mut self) -> BcW<Pr1IcssUart_UartSlv_RegsLctrSpec> {
        BcW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Divisor Latch Access Bit"]
    #[inline(always)]
    #[must_use]
    pub fn dlab(&mut self) -> DlabW<Pr1IcssUart_UartSlv_RegsLctrSpec> {
        DlabW::new(self, 7)
    }
}
#[doc = "Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_uart__uart_slv__regs_lctr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_uart__uart_slv__regs_lctr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssUart_UartSlv_RegsLctrSpec;
impl crate::RegisterSpec for Pr1IcssUart_UartSlv_RegsLctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_uart__uart_slv__regs_lctr::R`](R) reader structure"]
impl crate::Readable for Pr1IcssUart_UartSlv_RegsLctrSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_uart__uart_slv__regs_lctr::W`](W) writer structure"]
impl crate::Writable for Pr1IcssUart_UartSlv_RegsLctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_UART__UART_SLV__REGS_LCTR to value 0"]
impl crate::Resettable for Pr1IcssUart_UartSlv_RegsLctrSpec {
    const RESET_VALUE: u32 = 0;
}
