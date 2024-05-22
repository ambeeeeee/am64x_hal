#[doc = "Register `CFG0_PADCONFIG21` reader"]
pub type R = crate::R<Cfg0Padconfig21Spec>;
#[doc = "Register `CFG0_PADCONFIG21` writer"]
pub type W = crate::W<Cfg0Padconfig21Spec>;
#[doc = "Field `PADCONFIG21_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig21MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG21_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig21MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG21_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig21DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG21_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig21DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG21_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig21StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG21_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig21StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG21_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig21PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG21_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig21PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG21_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig21PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG21_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig21PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG21_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig21RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG21_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig21RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG21_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig21DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG21_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig21DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG21_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig21TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG21_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig21TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG21_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig21LockR = crate::BitReader;
#[doc = "Field `PADCONFIG21_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig21LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig21_muxmode(&self) -> Padconfig21MuxmodeR {
        Padconfig21MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig21_debounce_sel(&self) -> Padconfig21DebounceSelR {
        Padconfig21DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig21_st_en(&self) -> Padconfig21StEnR {
        Padconfig21StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig21_pulluden(&self) -> Padconfig21PulludenR {
        Padconfig21PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig21_pulltypesel(&self) -> Padconfig21PulltypeselR {
        Padconfig21PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig21_rxactive(&self) -> Padconfig21RxactiveR {
        Padconfig21RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig21_drv_str(&self) -> Padconfig21DrvStrR {
        Padconfig21DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig21_tx_dis(&self) -> Padconfig21TxDisR {
        Padconfig21TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig21_lock(&self) -> Padconfig21LockR {
        Padconfig21LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig21_muxmode(&mut self) -> Padconfig21MuxmodeW<Cfg0Padconfig21Spec> {
        Padconfig21MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig21_debounce_sel(&mut self) -> Padconfig21DebounceSelW<Cfg0Padconfig21Spec> {
        Padconfig21DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig21_st_en(&mut self) -> Padconfig21StEnW<Cfg0Padconfig21Spec> {
        Padconfig21StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig21_pulluden(&mut self) -> Padconfig21PulludenW<Cfg0Padconfig21Spec> {
        Padconfig21PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig21_pulltypesel(&mut self) -> Padconfig21PulltypeselW<Cfg0Padconfig21Spec> {
        Padconfig21PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig21_rxactive(&mut self) -> Padconfig21RxactiveW<Cfg0Padconfig21Spec> {
        Padconfig21RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig21_drv_str(&mut self) -> Padconfig21DrvStrW<Cfg0Padconfig21Spec> {
        Padconfig21DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig21_tx_dis(&mut self) -> Padconfig21TxDisW<Cfg0Padconfig21Spec> {
        Padconfig21TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig21_lock(&mut self) -> Padconfig21LockW<Cfg0Padconfig21Spec> {
        Padconfig21LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig21Spec;
impl crate::RegisterSpec for Cfg0Padconfig21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig21::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig21Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig21::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG21 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig21Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
