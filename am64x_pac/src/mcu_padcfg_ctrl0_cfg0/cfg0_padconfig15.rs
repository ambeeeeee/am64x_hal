#[doc = "Register `CFG0_PADCONFIG15` reader"]
pub type R = crate::R<Cfg0Padconfig15Spec>;
#[doc = "Register `CFG0_PADCONFIG15` writer"]
pub type W = crate::W<Cfg0Padconfig15Spec>;
#[doc = "Field `PADCONFIG15_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig15MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG15_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig15MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG15_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig15DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG15_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig15DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG15_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig15StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG15_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig15StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG15_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig15PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG15_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig15PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG15_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig15PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG15_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig15PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG15_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig15RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG15_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig15RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG15_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig15DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG15_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig15DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG15_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig15TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG15_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig15TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG15_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig15LockR = crate::BitReader;
#[doc = "Field `PADCONFIG15_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig15LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig15_muxmode(&self) -> Padconfig15MuxmodeR {
        Padconfig15MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig15_debounce_sel(&self) -> Padconfig15DebounceSelR {
        Padconfig15DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig15_st_en(&self) -> Padconfig15StEnR {
        Padconfig15StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig15_pulluden(&self) -> Padconfig15PulludenR {
        Padconfig15PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig15_pulltypesel(&self) -> Padconfig15PulltypeselR {
        Padconfig15PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig15_rxactive(&self) -> Padconfig15RxactiveR {
        Padconfig15RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig15_drv_str(&self) -> Padconfig15DrvStrR {
        Padconfig15DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig15_tx_dis(&self) -> Padconfig15TxDisR {
        Padconfig15TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig15_lock(&self) -> Padconfig15LockR {
        Padconfig15LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_muxmode(&mut self) -> Padconfig15MuxmodeW<Cfg0Padconfig15Spec> {
        Padconfig15MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_debounce_sel(&mut self) -> Padconfig15DebounceSelW<Cfg0Padconfig15Spec> {
        Padconfig15DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_st_en(&mut self) -> Padconfig15StEnW<Cfg0Padconfig15Spec> {
        Padconfig15StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_pulluden(&mut self) -> Padconfig15PulludenW<Cfg0Padconfig15Spec> {
        Padconfig15PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_pulltypesel(&mut self) -> Padconfig15PulltypeselW<Cfg0Padconfig15Spec> {
        Padconfig15PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_rxactive(&mut self) -> Padconfig15RxactiveW<Cfg0Padconfig15Spec> {
        Padconfig15RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_drv_str(&mut self) -> Padconfig15DrvStrW<Cfg0Padconfig15Spec> {
        Padconfig15DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_tx_dis(&mut self) -> Padconfig15TxDisW<Cfg0Padconfig15Spec> {
        Padconfig15TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_lock(&mut self) -> Padconfig15LockW<Cfg0Padconfig15Spec> {
        Padconfig15LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig15Spec;
impl crate::RegisterSpec for Cfg0Padconfig15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig15::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig15Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig15::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG15 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig15Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
