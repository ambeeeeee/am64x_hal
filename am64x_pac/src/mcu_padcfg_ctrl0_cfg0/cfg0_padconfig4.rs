#[doc = "Register `CFG0_PADCONFIG4` reader"]
pub type R = crate::R<Cfg0Padconfig4Spec>;
#[doc = "Register `CFG0_PADCONFIG4` writer"]
pub type W = crate::W<Cfg0Padconfig4Spec>;
#[doc = "Field `PADCONFIG4_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig4MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG4_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig4MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG4_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig4DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG4_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig4DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG4_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig4StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG4_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig4StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG4_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig4PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG4_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig4PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG4_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig4PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG4_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig4PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG4_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig4RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG4_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig4RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG4_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig4DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG4_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig4DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG4_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig4TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG4_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig4TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG4_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig4LockR = crate::BitReader;
#[doc = "Field `PADCONFIG4_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig4LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig4_muxmode(&self) -> Padconfig4MuxmodeR {
        Padconfig4MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig4_debounce_sel(&self) -> Padconfig4DebounceSelR {
        Padconfig4DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig4_st_en(&self) -> Padconfig4StEnR {
        Padconfig4StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig4_pulluden(&self) -> Padconfig4PulludenR {
        Padconfig4PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig4_pulltypesel(&self) -> Padconfig4PulltypeselR {
        Padconfig4PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig4_rxactive(&self) -> Padconfig4RxactiveR {
        Padconfig4RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig4_drv_str(&self) -> Padconfig4DrvStrR {
        Padconfig4DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig4_tx_dis(&self) -> Padconfig4TxDisR {
        Padconfig4TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig4_lock(&self) -> Padconfig4LockR {
        Padconfig4LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_muxmode(&mut self) -> Padconfig4MuxmodeW<Cfg0Padconfig4Spec> {
        Padconfig4MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_debounce_sel(&mut self) -> Padconfig4DebounceSelW<Cfg0Padconfig4Spec> {
        Padconfig4DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_st_en(&mut self) -> Padconfig4StEnW<Cfg0Padconfig4Spec> {
        Padconfig4StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_pulluden(&mut self) -> Padconfig4PulludenW<Cfg0Padconfig4Spec> {
        Padconfig4PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_pulltypesel(&mut self) -> Padconfig4PulltypeselW<Cfg0Padconfig4Spec> {
        Padconfig4PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_rxactive(&mut self) -> Padconfig4RxactiveW<Cfg0Padconfig4Spec> {
        Padconfig4RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_drv_str(&mut self) -> Padconfig4DrvStrW<Cfg0Padconfig4Spec> {
        Padconfig4DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_tx_dis(&mut self) -> Padconfig4TxDisW<Cfg0Padconfig4Spec> {
        Padconfig4TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_lock(&mut self) -> Padconfig4LockW<Cfg0Padconfig4Spec> {
        Padconfig4LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig4Spec;
impl crate::RegisterSpec for Cfg0Padconfig4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig4::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig4Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig4::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG4 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig4Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
