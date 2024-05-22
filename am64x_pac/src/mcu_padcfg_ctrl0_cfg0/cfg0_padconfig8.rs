#[doc = "Register `CFG0_PADCONFIG8` reader"]
pub type R = crate::R<Cfg0Padconfig8Spec>;
#[doc = "Register `CFG0_PADCONFIG8` writer"]
pub type W = crate::W<Cfg0Padconfig8Spec>;
#[doc = "Field `PADCONFIG8_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig8MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG8_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig8MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG8_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig8DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG8_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig8DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG8_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig8StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG8_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig8StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG8_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig8PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG8_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig8PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG8_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig8PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG8_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig8PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG8_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig8RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG8_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig8RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG8_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig8DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG8_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig8DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG8_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig8TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG8_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig8TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG8_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig8LockR = crate::BitReader;
#[doc = "Field `PADCONFIG8_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig8LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig8_muxmode(&self) -> Padconfig8MuxmodeR {
        Padconfig8MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig8_debounce_sel(&self) -> Padconfig8DebounceSelR {
        Padconfig8DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig8_st_en(&self) -> Padconfig8StEnR {
        Padconfig8StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig8_pulluden(&self) -> Padconfig8PulludenR {
        Padconfig8PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig8_pulltypesel(&self) -> Padconfig8PulltypeselR {
        Padconfig8PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig8_rxactive(&self) -> Padconfig8RxactiveR {
        Padconfig8RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig8_drv_str(&self) -> Padconfig8DrvStrR {
        Padconfig8DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig8_tx_dis(&self) -> Padconfig8TxDisR {
        Padconfig8TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig8_lock(&self) -> Padconfig8LockR {
        Padconfig8LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig8_muxmode(&mut self) -> Padconfig8MuxmodeW<Cfg0Padconfig8Spec> {
        Padconfig8MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig8_debounce_sel(&mut self) -> Padconfig8DebounceSelW<Cfg0Padconfig8Spec> {
        Padconfig8DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig8_st_en(&mut self) -> Padconfig8StEnW<Cfg0Padconfig8Spec> {
        Padconfig8StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig8_pulluden(&mut self) -> Padconfig8PulludenW<Cfg0Padconfig8Spec> {
        Padconfig8PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig8_pulltypesel(&mut self) -> Padconfig8PulltypeselW<Cfg0Padconfig8Spec> {
        Padconfig8PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig8_rxactive(&mut self) -> Padconfig8RxactiveW<Cfg0Padconfig8Spec> {
        Padconfig8RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig8_drv_str(&mut self) -> Padconfig8DrvStrW<Cfg0Padconfig8Spec> {
        Padconfig8DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig8_tx_dis(&mut self) -> Padconfig8TxDisW<Cfg0Padconfig8Spec> {
        Padconfig8TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig8_lock(&mut self) -> Padconfig8LockW<Cfg0Padconfig8Spec> {
        Padconfig8LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig8Spec;
impl crate::RegisterSpec for Cfg0Padconfig8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig8::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig8Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig8::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG8 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig8Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
