#[doc = "Register `CFG0_PADCONFIG10` reader"]
pub type R = crate::R<Cfg0Padconfig10Spec>;
#[doc = "Register `CFG0_PADCONFIG10` writer"]
pub type W = crate::W<Cfg0Padconfig10Spec>;
#[doc = "Field `PADCONFIG10_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig10MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG10_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig10MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG10_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig10DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG10_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig10DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG10_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig10StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG10_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig10StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG10_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig10PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG10_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig10PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG10_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig10PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG10_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig10PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG10_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig10RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG10_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig10RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG10_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig10DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG10_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig10DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG10_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig10TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG10_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig10TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG10_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig10LockR = crate::BitReader;
#[doc = "Field `PADCONFIG10_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig10LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig10_muxmode(&self) -> Padconfig10MuxmodeR {
        Padconfig10MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig10_debounce_sel(&self) -> Padconfig10DebounceSelR {
        Padconfig10DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig10_st_en(&self) -> Padconfig10StEnR {
        Padconfig10StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig10_pulluden(&self) -> Padconfig10PulludenR {
        Padconfig10PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig10_pulltypesel(&self) -> Padconfig10PulltypeselR {
        Padconfig10PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig10_rxactive(&self) -> Padconfig10RxactiveR {
        Padconfig10RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig10_drv_str(&self) -> Padconfig10DrvStrR {
        Padconfig10DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig10_tx_dis(&self) -> Padconfig10TxDisR {
        Padconfig10TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig10_lock(&self) -> Padconfig10LockR {
        Padconfig10LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig10_muxmode(&mut self) -> Padconfig10MuxmodeW<Cfg0Padconfig10Spec> {
        Padconfig10MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig10_debounce_sel(&mut self) -> Padconfig10DebounceSelW<Cfg0Padconfig10Spec> {
        Padconfig10DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig10_st_en(&mut self) -> Padconfig10StEnW<Cfg0Padconfig10Spec> {
        Padconfig10StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig10_pulluden(&mut self) -> Padconfig10PulludenW<Cfg0Padconfig10Spec> {
        Padconfig10PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig10_pulltypesel(&mut self) -> Padconfig10PulltypeselW<Cfg0Padconfig10Spec> {
        Padconfig10PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig10_rxactive(&mut self) -> Padconfig10RxactiveW<Cfg0Padconfig10Spec> {
        Padconfig10RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig10_drv_str(&mut self) -> Padconfig10DrvStrW<Cfg0Padconfig10Spec> {
        Padconfig10DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig10_tx_dis(&mut self) -> Padconfig10TxDisW<Cfg0Padconfig10Spec> {
        Padconfig10TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig10_lock(&mut self) -> Padconfig10LockW<Cfg0Padconfig10Spec> {
        Padconfig10LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig10Spec;
impl crate::RegisterSpec for Cfg0Padconfig10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig10::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig10Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig10::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG10 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig10Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
