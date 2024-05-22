#[doc = "Register `CFG0_PADCONFIG0` reader"]
pub type R = crate::R<Cfg0Padconfig0Spec>;
#[doc = "Register `CFG0_PADCONFIG0` writer"]
pub type W = crate::W<Cfg0Padconfig0Spec>;
#[doc = "Field `PADCONFIG0_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig0MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG0_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig0MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG0_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig0DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG0_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig0DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG0_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig0StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG0_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig0StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG0_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig0PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG0_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig0PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG0_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig0PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG0_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig0PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG0_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig0RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG0_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig0RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG0_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig0DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG0_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig0DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG0_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig0TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG0_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig0TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG0_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig0LockR = crate::BitReader;
#[doc = "Field `PADCONFIG0_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig0LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig0_muxmode(&self) -> Padconfig0MuxmodeR {
        Padconfig0MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig0_debounce_sel(&self) -> Padconfig0DebounceSelR {
        Padconfig0DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig0_st_en(&self) -> Padconfig0StEnR {
        Padconfig0StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig0_pulluden(&self) -> Padconfig0PulludenR {
        Padconfig0PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig0_pulltypesel(&self) -> Padconfig0PulltypeselR {
        Padconfig0PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig0_rxactive(&self) -> Padconfig0RxactiveR {
        Padconfig0RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig0_drv_str(&self) -> Padconfig0DrvStrR {
        Padconfig0DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig0_tx_dis(&self) -> Padconfig0TxDisR {
        Padconfig0TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig0_lock(&self) -> Padconfig0LockR {
        Padconfig0LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig0_muxmode(&mut self) -> Padconfig0MuxmodeW<Cfg0Padconfig0Spec> {
        Padconfig0MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig0_debounce_sel(&mut self) -> Padconfig0DebounceSelW<Cfg0Padconfig0Spec> {
        Padconfig0DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig0_st_en(&mut self) -> Padconfig0StEnW<Cfg0Padconfig0Spec> {
        Padconfig0StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig0_pulluden(&mut self) -> Padconfig0PulludenW<Cfg0Padconfig0Spec> {
        Padconfig0PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig0_pulltypesel(&mut self) -> Padconfig0PulltypeselW<Cfg0Padconfig0Spec> {
        Padconfig0PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig0_rxactive(&mut self) -> Padconfig0RxactiveW<Cfg0Padconfig0Spec> {
        Padconfig0RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig0_drv_str(&mut self) -> Padconfig0DrvStrW<Cfg0Padconfig0Spec> {
        Padconfig0DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig0_tx_dis(&mut self) -> Padconfig0TxDisW<Cfg0Padconfig0Spec> {
        Padconfig0TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig0_lock(&mut self) -> Padconfig0LockW<Cfg0Padconfig0Spec> {
        Padconfig0LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig0Spec;
impl crate::RegisterSpec for Cfg0Padconfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig0::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig0::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG0 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig0Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
