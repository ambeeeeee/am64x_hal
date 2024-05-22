#[doc = "Register `CFG0_PADCONFIG7` reader"]
pub type R = crate::R<Cfg0Padconfig7Spec>;
#[doc = "Register `CFG0_PADCONFIG7` writer"]
pub type W = crate::W<Cfg0Padconfig7Spec>;
#[doc = "Field `PADCONFIG7_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig7MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG7_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig7MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG7_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig7DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG7_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig7DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG7_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig7StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG7_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig7StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG7_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig7PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG7_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig7PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG7_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig7PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG7_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig7PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG7_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig7RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG7_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig7RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG7_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig7DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG7_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig7DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG7_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig7TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG7_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig7TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG7_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig7LockR = crate::BitReader;
#[doc = "Field `PADCONFIG7_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig7LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig7_muxmode(&self) -> Padconfig7MuxmodeR {
        Padconfig7MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig7_debounce_sel(&self) -> Padconfig7DebounceSelR {
        Padconfig7DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig7_st_en(&self) -> Padconfig7StEnR {
        Padconfig7StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig7_pulluden(&self) -> Padconfig7PulludenR {
        Padconfig7PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig7_pulltypesel(&self) -> Padconfig7PulltypeselR {
        Padconfig7PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig7_rxactive(&self) -> Padconfig7RxactiveR {
        Padconfig7RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig7_drv_str(&self) -> Padconfig7DrvStrR {
        Padconfig7DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig7_tx_dis(&self) -> Padconfig7TxDisR {
        Padconfig7TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig7_lock(&self) -> Padconfig7LockR {
        Padconfig7LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig7_muxmode(&mut self) -> Padconfig7MuxmodeW<Cfg0Padconfig7Spec> {
        Padconfig7MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig7_debounce_sel(&mut self) -> Padconfig7DebounceSelW<Cfg0Padconfig7Spec> {
        Padconfig7DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig7_st_en(&mut self) -> Padconfig7StEnW<Cfg0Padconfig7Spec> {
        Padconfig7StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig7_pulluden(&mut self) -> Padconfig7PulludenW<Cfg0Padconfig7Spec> {
        Padconfig7PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig7_pulltypesel(&mut self) -> Padconfig7PulltypeselW<Cfg0Padconfig7Spec> {
        Padconfig7PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig7_rxactive(&mut self) -> Padconfig7RxactiveW<Cfg0Padconfig7Spec> {
        Padconfig7RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig7_drv_str(&mut self) -> Padconfig7DrvStrW<Cfg0Padconfig7Spec> {
        Padconfig7DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig7_tx_dis(&mut self) -> Padconfig7TxDisW<Cfg0Padconfig7Spec> {
        Padconfig7TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig7_lock(&mut self) -> Padconfig7LockW<Cfg0Padconfig7Spec> {
        Padconfig7LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig7Spec;
impl crate::RegisterSpec for Cfg0Padconfig7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig7::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig7Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig7::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG7 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig7Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
