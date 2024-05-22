#[doc = "Register `CFG0_PADCONFIG13` reader"]
pub type R = crate::R<Cfg0Padconfig13Spec>;
#[doc = "Register `CFG0_PADCONFIG13` writer"]
pub type W = crate::W<Cfg0Padconfig13Spec>;
#[doc = "Field `PADCONFIG13_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig13MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG13_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig13MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG13_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig13DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG13_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig13DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG13_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig13StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG13_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig13StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG13_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig13PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG13_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig13PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG13_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig13PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG13_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig13PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG13_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig13RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG13_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig13RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG13_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig13DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG13_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig13DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG13_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig13TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG13_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig13TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG13_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig13LockR = crate::BitReader;
#[doc = "Field `PADCONFIG13_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig13LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig13_muxmode(&self) -> Padconfig13MuxmodeR {
        Padconfig13MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig13_debounce_sel(&self) -> Padconfig13DebounceSelR {
        Padconfig13DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig13_st_en(&self) -> Padconfig13StEnR {
        Padconfig13StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig13_pulluden(&self) -> Padconfig13PulludenR {
        Padconfig13PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig13_pulltypesel(&self) -> Padconfig13PulltypeselR {
        Padconfig13PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig13_rxactive(&self) -> Padconfig13RxactiveR {
        Padconfig13RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig13_drv_str(&self) -> Padconfig13DrvStrR {
        Padconfig13DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig13_tx_dis(&self) -> Padconfig13TxDisR {
        Padconfig13TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig13_lock(&self) -> Padconfig13LockR {
        Padconfig13LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig13_muxmode(&mut self) -> Padconfig13MuxmodeW<Cfg0Padconfig13Spec> {
        Padconfig13MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig13_debounce_sel(&mut self) -> Padconfig13DebounceSelW<Cfg0Padconfig13Spec> {
        Padconfig13DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig13_st_en(&mut self) -> Padconfig13StEnW<Cfg0Padconfig13Spec> {
        Padconfig13StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig13_pulluden(&mut self) -> Padconfig13PulludenW<Cfg0Padconfig13Spec> {
        Padconfig13PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig13_pulltypesel(&mut self) -> Padconfig13PulltypeselW<Cfg0Padconfig13Spec> {
        Padconfig13PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig13_rxactive(&mut self) -> Padconfig13RxactiveW<Cfg0Padconfig13Spec> {
        Padconfig13RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig13_drv_str(&mut self) -> Padconfig13DrvStrW<Cfg0Padconfig13Spec> {
        Padconfig13DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig13_tx_dis(&mut self) -> Padconfig13TxDisW<Cfg0Padconfig13Spec> {
        Padconfig13TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig13_lock(&mut self) -> Padconfig13LockW<Cfg0Padconfig13Spec> {
        Padconfig13LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig13Spec;
impl crate::RegisterSpec for Cfg0Padconfig13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig13::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig13Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig13::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG13 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig13Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
