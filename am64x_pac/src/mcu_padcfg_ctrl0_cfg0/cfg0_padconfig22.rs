#[doc = "Register `CFG0_PADCONFIG22` reader"]
pub type R = crate::R<Cfg0Padconfig22Spec>;
#[doc = "Register `CFG0_PADCONFIG22` writer"]
pub type W = crate::W<Cfg0Padconfig22Spec>;
#[doc = "Field `PADCONFIG22_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig22MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG22_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig22MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG22_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig22DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG22_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig22DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG22_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig22StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG22_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig22StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG22_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig22PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG22_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig22PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG22_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig22PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG22_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig22PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG22_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig22RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG22_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig22RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG22_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig22DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG22_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig22DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG22_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig22TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG22_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig22TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG22_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig22LockR = crate::BitReader;
#[doc = "Field `PADCONFIG22_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig22LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig22_muxmode(&self) -> Padconfig22MuxmodeR {
        Padconfig22MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig22_debounce_sel(&self) -> Padconfig22DebounceSelR {
        Padconfig22DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig22_st_en(&self) -> Padconfig22StEnR {
        Padconfig22StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig22_pulluden(&self) -> Padconfig22PulludenR {
        Padconfig22PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig22_pulltypesel(&self) -> Padconfig22PulltypeselR {
        Padconfig22PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig22_rxactive(&self) -> Padconfig22RxactiveR {
        Padconfig22RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig22_drv_str(&self) -> Padconfig22DrvStrR {
        Padconfig22DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig22_tx_dis(&self) -> Padconfig22TxDisR {
        Padconfig22TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig22_lock(&self) -> Padconfig22LockR {
        Padconfig22LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig22_muxmode(&mut self) -> Padconfig22MuxmodeW<Cfg0Padconfig22Spec> {
        Padconfig22MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig22_debounce_sel(&mut self) -> Padconfig22DebounceSelW<Cfg0Padconfig22Spec> {
        Padconfig22DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig22_st_en(&mut self) -> Padconfig22StEnW<Cfg0Padconfig22Spec> {
        Padconfig22StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig22_pulluden(&mut self) -> Padconfig22PulludenW<Cfg0Padconfig22Spec> {
        Padconfig22PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig22_pulltypesel(&mut self) -> Padconfig22PulltypeselW<Cfg0Padconfig22Spec> {
        Padconfig22PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig22_rxactive(&mut self) -> Padconfig22RxactiveW<Cfg0Padconfig22Spec> {
        Padconfig22RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig22_drv_str(&mut self) -> Padconfig22DrvStrW<Cfg0Padconfig22Spec> {
        Padconfig22DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig22_tx_dis(&mut self) -> Padconfig22TxDisW<Cfg0Padconfig22Spec> {
        Padconfig22TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig22_lock(&mut self) -> Padconfig22LockW<Cfg0Padconfig22Spec> {
        Padconfig22LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig22Spec;
impl crate::RegisterSpec for Cfg0Padconfig22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig22::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig22Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig22::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG22 to value 0x0026_4000"]
impl crate::Resettable for Cfg0Padconfig22Spec {
    const RESET_VALUE: u32 = 0x0026_4000;
}
