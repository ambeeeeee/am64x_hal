#[doc = "Register `CFG0_PADCONFIG3` reader"]
pub type R = crate::R<Cfg0Padconfig3Spec>;
#[doc = "Register `CFG0_PADCONFIG3` writer"]
pub type W = crate::W<Cfg0Padconfig3Spec>;
#[doc = "Field `PADCONFIG3_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig3MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG3_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig3MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG3_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig3DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG3_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig3DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG3_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig3StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG3_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig3StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG3_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig3PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG3_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig3PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG3_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig3PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG3_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig3PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG3_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig3RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG3_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig3RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG3_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig3DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG3_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig3DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG3_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig3TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG3_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig3TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG3_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig3LockR = crate::BitReader;
#[doc = "Field `PADCONFIG3_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig3LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig3_muxmode(&self) -> Padconfig3MuxmodeR {
        Padconfig3MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig3_debounce_sel(&self) -> Padconfig3DebounceSelR {
        Padconfig3DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig3_st_en(&self) -> Padconfig3StEnR {
        Padconfig3StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig3_pulluden(&self) -> Padconfig3PulludenR {
        Padconfig3PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig3_pulltypesel(&self) -> Padconfig3PulltypeselR {
        Padconfig3PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig3_rxactive(&self) -> Padconfig3RxactiveR {
        Padconfig3RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig3_drv_str(&self) -> Padconfig3DrvStrR {
        Padconfig3DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig3_tx_dis(&self) -> Padconfig3TxDisR {
        Padconfig3TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig3_lock(&self) -> Padconfig3LockR {
        Padconfig3LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig3_muxmode(&mut self) -> Padconfig3MuxmodeW<Cfg0Padconfig3Spec> {
        Padconfig3MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig3_debounce_sel(&mut self) -> Padconfig3DebounceSelW<Cfg0Padconfig3Spec> {
        Padconfig3DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig3_st_en(&mut self) -> Padconfig3StEnW<Cfg0Padconfig3Spec> {
        Padconfig3StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig3_pulluden(&mut self) -> Padconfig3PulludenW<Cfg0Padconfig3Spec> {
        Padconfig3PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig3_pulltypesel(&mut self) -> Padconfig3PulltypeselW<Cfg0Padconfig3Spec> {
        Padconfig3PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig3_rxactive(&mut self) -> Padconfig3RxactiveW<Cfg0Padconfig3Spec> {
        Padconfig3RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig3_drv_str(&mut self) -> Padconfig3DrvStrW<Cfg0Padconfig3Spec> {
        Padconfig3DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig3_tx_dis(&mut self) -> Padconfig3TxDisW<Cfg0Padconfig3Spec> {
        Padconfig3TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig3_lock(&mut self) -> Padconfig3LockW<Cfg0Padconfig3Spec> {
        Padconfig3LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig3Spec;
impl crate::RegisterSpec for Cfg0Padconfig3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig3::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig3::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG3 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig3Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
