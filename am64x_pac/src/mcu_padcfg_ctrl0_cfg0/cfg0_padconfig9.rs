#[doc = "Register `CFG0_PADCONFIG9` reader"]
pub type R = crate::R<Cfg0Padconfig9Spec>;
#[doc = "Register `CFG0_PADCONFIG9` writer"]
pub type W = crate::W<Cfg0Padconfig9Spec>;
#[doc = "Field `PADCONFIG9_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig9MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG9_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig9MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG9_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig9DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG9_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig9DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG9_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig9StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG9_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig9StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG9_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig9PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG9_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig9PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG9_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig9PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG9_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig9PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG9_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig9RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG9_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig9RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG9_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig9DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG9_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig9DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG9_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig9TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG9_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig9TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG9_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig9LockR = crate::BitReader;
#[doc = "Field `PADCONFIG9_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig9LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig9_muxmode(&self) -> Padconfig9MuxmodeR {
        Padconfig9MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig9_debounce_sel(&self) -> Padconfig9DebounceSelR {
        Padconfig9DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig9_st_en(&self) -> Padconfig9StEnR {
        Padconfig9StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig9_pulluden(&self) -> Padconfig9PulludenR {
        Padconfig9PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig9_pulltypesel(&self) -> Padconfig9PulltypeselR {
        Padconfig9PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig9_rxactive(&self) -> Padconfig9RxactiveR {
        Padconfig9RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig9_drv_str(&self) -> Padconfig9DrvStrR {
        Padconfig9DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig9_tx_dis(&self) -> Padconfig9TxDisR {
        Padconfig9TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig9_lock(&self) -> Padconfig9LockR {
        Padconfig9LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig9_muxmode(&mut self) -> Padconfig9MuxmodeW<Cfg0Padconfig9Spec> {
        Padconfig9MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig9_debounce_sel(&mut self) -> Padconfig9DebounceSelW<Cfg0Padconfig9Spec> {
        Padconfig9DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig9_st_en(&mut self) -> Padconfig9StEnW<Cfg0Padconfig9Spec> {
        Padconfig9StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig9_pulluden(&mut self) -> Padconfig9PulludenW<Cfg0Padconfig9Spec> {
        Padconfig9PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig9_pulltypesel(&mut self) -> Padconfig9PulltypeselW<Cfg0Padconfig9Spec> {
        Padconfig9PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig9_rxactive(&mut self) -> Padconfig9RxactiveW<Cfg0Padconfig9Spec> {
        Padconfig9RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig9_drv_str(&mut self) -> Padconfig9DrvStrW<Cfg0Padconfig9Spec> {
        Padconfig9DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig9_tx_dis(&mut self) -> Padconfig9TxDisW<Cfg0Padconfig9Spec> {
        Padconfig9TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig9_lock(&mut self) -> Padconfig9LockW<Cfg0Padconfig9Spec> {
        Padconfig9LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig9Spec;
impl crate::RegisterSpec for Cfg0Padconfig9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig9::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig9Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig9::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG9 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig9Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
