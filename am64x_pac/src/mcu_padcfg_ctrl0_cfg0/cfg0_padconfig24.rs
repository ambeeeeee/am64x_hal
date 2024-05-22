#[doc = "Register `CFG0_PADCONFIG24` reader"]
pub type R = crate::R<Cfg0Padconfig24Spec>;
#[doc = "Register `CFG0_PADCONFIG24` writer"]
pub type W = crate::W<Cfg0Padconfig24Spec>;
#[doc = "Field `PADCONFIG24_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig24MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG24_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig24MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG24_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig24DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG24_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig24DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG24_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig24StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG24_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig24StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG24_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig24PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG24_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig24PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG24_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig24PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG24_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig24PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG24_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig24RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG24_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig24RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG24_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig24DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG24_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig24DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG24_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig24TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG24_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig24TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG24_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig24LockR = crate::BitReader;
#[doc = "Field `PADCONFIG24_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig24LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig24_muxmode(&self) -> Padconfig24MuxmodeR {
        Padconfig24MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig24_debounce_sel(&self) -> Padconfig24DebounceSelR {
        Padconfig24DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig24_st_en(&self) -> Padconfig24StEnR {
        Padconfig24StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig24_pulluden(&self) -> Padconfig24PulludenR {
        Padconfig24PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig24_pulltypesel(&self) -> Padconfig24PulltypeselR {
        Padconfig24PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig24_rxactive(&self) -> Padconfig24RxactiveR {
        Padconfig24RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig24_drv_str(&self) -> Padconfig24DrvStrR {
        Padconfig24DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig24_tx_dis(&self) -> Padconfig24TxDisR {
        Padconfig24TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig24_lock(&self) -> Padconfig24LockR {
        Padconfig24LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig24_muxmode(&mut self) -> Padconfig24MuxmodeW<Cfg0Padconfig24Spec> {
        Padconfig24MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig24_debounce_sel(&mut self) -> Padconfig24DebounceSelW<Cfg0Padconfig24Spec> {
        Padconfig24DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig24_st_en(&mut self) -> Padconfig24StEnW<Cfg0Padconfig24Spec> {
        Padconfig24StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig24_pulluden(&mut self) -> Padconfig24PulludenW<Cfg0Padconfig24Spec> {
        Padconfig24PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig24_pulltypesel(&mut self) -> Padconfig24PulltypeselW<Cfg0Padconfig24Spec> {
        Padconfig24PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig24_rxactive(&mut self) -> Padconfig24RxactiveW<Cfg0Padconfig24Spec> {
        Padconfig24RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig24_drv_str(&mut self) -> Padconfig24DrvStrW<Cfg0Padconfig24Spec> {
        Padconfig24DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig24_tx_dis(&mut self) -> Padconfig24TxDisW<Cfg0Padconfig24Spec> {
        Padconfig24TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig24_lock(&mut self) -> Padconfig24LockW<Cfg0Padconfig24Spec> {
        Padconfig24LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig24Spec;
impl crate::RegisterSpec for Cfg0Padconfig24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig24::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig24Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig24::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG24 to value 0x0001_4000"]
impl crate::Resettable for Cfg0Padconfig24Spec {
    const RESET_VALUE: u32 = 0x0001_4000;
}
