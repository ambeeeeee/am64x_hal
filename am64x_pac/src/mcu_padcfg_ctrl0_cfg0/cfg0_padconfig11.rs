#[doc = "Register `CFG0_PADCONFIG11` reader"]
pub type R = crate::R<Cfg0Padconfig11Spec>;
#[doc = "Register `CFG0_PADCONFIG11` writer"]
pub type W = crate::W<Cfg0Padconfig11Spec>;
#[doc = "Field `PADCONFIG11_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig11MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG11_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig11MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG11_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig11DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG11_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig11DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG11_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig11StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG11_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig11StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG11_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig11PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG11_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig11PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG11_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig11PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG11_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig11PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG11_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig11RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG11_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig11RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG11_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig11DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG11_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig11DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG11_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig11TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG11_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig11TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG11_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig11LockR = crate::BitReader;
#[doc = "Field `PADCONFIG11_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig11LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig11_muxmode(&self) -> Padconfig11MuxmodeR {
        Padconfig11MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig11_debounce_sel(&self) -> Padconfig11DebounceSelR {
        Padconfig11DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig11_st_en(&self) -> Padconfig11StEnR {
        Padconfig11StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig11_pulluden(&self) -> Padconfig11PulludenR {
        Padconfig11PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig11_pulltypesel(&self) -> Padconfig11PulltypeselR {
        Padconfig11PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig11_rxactive(&self) -> Padconfig11RxactiveR {
        Padconfig11RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig11_drv_str(&self) -> Padconfig11DrvStrR {
        Padconfig11DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig11_tx_dis(&self) -> Padconfig11TxDisR {
        Padconfig11TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig11_lock(&self) -> Padconfig11LockR {
        Padconfig11LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig11_muxmode(&mut self) -> Padconfig11MuxmodeW<Cfg0Padconfig11Spec> {
        Padconfig11MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig11_debounce_sel(&mut self) -> Padconfig11DebounceSelW<Cfg0Padconfig11Spec> {
        Padconfig11DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig11_st_en(&mut self) -> Padconfig11StEnW<Cfg0Padconfig11Spec> {
        Padconfig11StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig11_pulluden(&mut self) -> Padconfig11PulludenW<Cfg0Padconfig11Spec> {
        Padconfig11PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig11_pulltypesel(&mut self) -> Padconfig11PulltypeselW<Cfg0Padconfig11Spec> {
        Padconfig11PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig11_rxactive(&mut self) -> Padconfig11RxactiveW<Cfg0Padconfig11Spec> {
        Padconfig11RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig11_drv_str(&mut self) -> Padconfig11DrvStrW<Cfg0Padconfig11Spec> {
        Padconfig11DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig11_tx_dis(&mut self) -> Padconfig11TxDisW<Cfg0Padconfig11Spec> {
        Padconfig11TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig11_lock(&mut self) -> Padconfig11LockW<Cfg0Padconfig11Spec> {
        Padconfig11LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig11Spec;
impl crate::RegisterSpec for Cfg0Padconfig11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig11::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig11Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig11::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG11 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig11Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
