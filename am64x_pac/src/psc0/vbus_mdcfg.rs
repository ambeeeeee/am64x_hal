#[doc = "Register `VBUS_MDCFG` reader"]
pub type R = crate::R<VbusMdcfgSpec>;
#[doc = "Register `VBUS_MDCFG` writer"]
pub type W = crate::W<VbusMdcfgSpec>;
#[doc = "Field `NUMCLK` reader - 2:0\\]
Number of PWR_CLKSTOP interfaces required on LPSC"]
pub type NumclkR = crate::FieldReader;
#[doc = "Field `NUMCLK` writer - 2:0\\]
Number of PWR_CLKSTOP interfaces required on LPSC"]
pub type NumclkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NUMCLKEN` reader - 5:3\\]
Number of PWR_CLK_EN interfaces required on LPSC"]
pub type NumclkenR = crate::FieldReader;
#[doc = "Field `NUMCLKEN` writer - 5:3\\]
Number of PWR_CLK_EN interfaces required on LPSC"]
pub type NumclkenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NUMSCRDISBALE` reader - 8:6\\]
Number of PWR_SCR_DISABLE interfaces required on LPSC"]
pub type NumscrdisbaleR = crate::FieldReader;
#[doc = "Field `NUMSCRDISBALE` writer - 8:6\\]
Number of PWR_SCR_DISABLE interfaces required on LPSC"]
pub type NumscrdisbaleW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLHANDSHAKE` reader - 9:9\\]
RTL parameter PLL_HANDSHAKE"]
pub type PllhandshakeR = crate::BitReader;
#[doc = "Field `PLLHANDSHAKE` writer - 9:9\\]
RTL parameter PLL_HANDSHAKE"]
pub type PllhandshakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERMDIS` reader - 10:10\\]
Permanently disable"]
pub type PermdisR = crate::BitReader;
#[doc = "Field `PERMDIS` writer - 10:10\\]
Permanently disable"]
pub type PermdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK` reader - 11:11\\]
IcePick support"]
pub type IcepickR = crate::BitReader;
#[doc = "Field `ICEPICK` writer - 11:11\\]
IcePick support"]
pub type IcepickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNC` reader - 12:12\\]
Async Lpsc"]
pub type AsyncR = crate::BitReader;
#[doc = "Field `ASYNC` writer - 12:12\\]
Async Lpsc"]
pub type AsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEXTLOCK` reader - 13:13\\]
0: MDCTL.NEXT field is writable, 1: MDCTL.NEXT field is locked"]
pub type NextlockR = crate::BitReader;
#[doc = "Field `NEXTLOCK` writer - 13:13\\]
0: MDCTL.NEXT field is writable, 1: MDCTL.NEXT field is locked"]
pub type NextlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETISO` reader - 14:14\\]
0: This LPSC does not support Reset Isolation, 1: This LPSC supports Reset Isolation"]
pub type ResetisoR = crate::BitReader;
#[doc = "Field `RESETISO` writer - 14:14\\]
0: This LPSC does not support Reset Isolation, 1: This LPSC supports Reset Isolation"]
pub type ResetisoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOONLY` reader - 15:15\\]
0: This LPSC supports all modes, 1: This LPSC supports Enable, AutoSleep or AutoWake only"]
pub type AutoonlyR = crate::BitReader;
#[doc = "Field `AUTOONLY` writer - 15:15\\]
0: This LPSC supports all modes, 1: This LPSC supports Enable, AutoSleep or AutoWake only"]
pub type AutoonlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRDOM` reader - 20:16\\]
Indicates which power domain this module belongs to"]
pub type PwrdomR = crate::FieldReader;
#[doc = "Field `PWRDOM` writer - 20:16\\]
Indicates which power domain this module belongs to"]
pub type PwrdomW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Number of PWR_CLKSTOP interfaces required on LPSC"]
    #[inline(always)]
    pub fn numclk(&self) -> NumclkR {
        NumclkR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Number of PWR_CLK_EN interfaces required on LPSC"]
    #[inline(always)]
    pub fn numclken(&self) -> NumclkenR {
        NumclkenR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Number of PWR_SCR_DISABLE interfaces required on LPSC"]
    #[inline(always)]
    pub fn numscrdisbale(&self) -> NumscrdisbaleR {
        NumscrdisbaleR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
RTL parameter PLL_HANDSHAKE"]
    #[inline(always)]
    pub fn pllhandshake(&self) -> PllhandshakeR {
        PllhandshakeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Permanently disable"]
    #[inline(always)]
    pub fn permdis(&self) -> PermdisR {
        PermdisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
IcePick support"]
    #[inline(always)]
    pub fn icepick(&self) -> IcepickR {
        IcepickR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Async Lpsc"]
    #[inline(always)]
    pub fn async_(&self) -> AsyncR {
        AsyncR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: MDCTL.NEXT field is writable, 1: MDCTL.NEXT field is locked"]
    #[inline(always)]
    pub fn nextlock(&self) -> NextlockR {
        NextlockR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
0: This LPSC does not support Reset Isolation, 1: This LPSC supports Reset Isolation"]
    #[inline(always)]
    pub fn resetiso(&self) -> ResetisoR {
        ResetisoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
0: This LPSC supports all modes, 1: This LPSC supports Enable, AutoSleep or AutoWake only"]
    #[inline(always)]
    pub fn autoonly(&self) -> AutoonlyR {
        AutoonlyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Indicates which power domain this module belongs to"]
    #[inline(always)]
    pub fn pwrdom(&self) -> PwrdomR {
        PwrdomR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Number of PWR_CLKSTOP interfaces required on LPSC"]
    #[inline(always)]
    #[must_use]
    pub fn numclk(&mut self) -> NumclkW<VbusMdcfgSpec> {
        NumclkW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Number of PWR_CLK_EN interfaces required on LPSC"]
    #[inline(always)]
    #[must_use]
    pub fn numclken(&mut self) -> NumclkenW<VbusMdcfgSpec> {
        NumclkenW::new(self, 3)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Number of PWR_SCR_DISABLE interfaces required on LPSC"]
    #[inline(always)]
    #[must_use]
    pub fn numscrdisbale(&mut self) -> NumscrdisbaleW<VbusMdcfgSpec> {
        NumscrdisbaleW::new(self, 6)
    }
    #[doc = "Bit 9 - 9:9\\]
RTL parameter PLL_HANDSHAKE"]
    #[inline(always)]
    #[must_use]
    pub fn pllhandshake(&mut self) -> PllhandshakeW<VbusMdcfgSpec> {
        PllhandshakeW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Permanently disable"]
    #[inline(always)]
    #[must_use]
    pub fn permdis(&mut self) -> PermdisW<VbusMdcfgSpec> {
        PermdisW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
IcePick support"]
    #[inline(always)]
    #[must_use]
    pub fn icepick(&mut self) -> IcepickW<VbusMdcfgSpec> {
        IcepickW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Async Lpsc"]
    #[inline(always)]
    #[must_use]
    pub fn async_(&mut self) -> AsyncW<VbusMdcfgSpec> {
        AsyncW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
0: MDCTL.NEXT field is writable, 1: MDCTL.NEXT field is locked"]
    #[inline(always)]
    #[must_use]
    pub fn nextlock(&mut self) -> NextlockW<VbusMdcfgSpec> {
        NextlockW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
0: This LPSC does not support Reset Isolation, 1: This LPSC supports Reset Isolation"]
    #[inline(always)]
    #[must_use]
    pub fn resetiso(&mut self) -> ResetisoW<VbusMdcfgSpec> {
        ResetisoW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
0: This LPSC supports all modes, 1: This LPSC supports Enable, AutoSleep or AutoWake only"]
    #[inline(always)]
    #[must_use]
    pub fn autoonly(&mut self) -> AutoonlyW<VbusMdcfgSpec> {
        AutoonlyW::new(self, 15)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Indicates which power domain this module belongs to"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdom(&mut self) -> PwrdomW<VbusMdcfgSpec> {
        PwrdomW::new(self, 16)
    }
}
#[doc = "This is a constant register showing some PSC settings for easy debug. This register is read only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_mdcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_mdcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusMdcfgSpec;
impl crate::RegisterSpec for VbusMdcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_mdcfg::R`](R) reader structure"]
impl crate::Readable for VbusMdcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_mdcfg::W`](W) writer structure"]
impl crate::Writable for VbusMdcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_MDCFG to value 0"]
impl crate::Resettable for VbusMdcfgSpec {
    const RESET_VALUE: u32 = 0;
}
