#[doc = "Register `CFG0_PADCONFIG28` reader"]
pub type R = crate::R<Cfg0Padconfig28Spec>;
#[doc = "Register `CFG0_PADCONFIG28` writer"]
pub type W = crate::W<Cfg0Padconfig28Spec>;
#[doc = "Field `PADCONFIG28_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig28MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG28_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig28MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG28_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig28DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG28_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig28DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG28_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig28StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG28_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig28StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG28_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig28PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG28_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig28PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG28_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig28PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG28_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig28PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG28_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig28RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG28_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig28RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG28_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig28DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG28_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig28DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG28_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig28TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG28_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig28TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG28_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig28LockR = crate::BitReader;
#[doc = "Field `PADCONFIG28_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig28LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig28_muxmode(&self) -> Padconfig28MuxmodeR {
        Padconfig28MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig28_debounce_sel(&self) -> Padconfig28DebounceSelR {
        Padconfig28DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig28_st_en(&self) -> Padconfig28StEnR {
        Padconfig28StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig28_pulluden(&self) -> Padconfig28PulludenR {
        Padconfig28PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig28_pulltypesel(&self) -> Padconfig28PulltypeselR {
        Padconfig28PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig28_rxactive(&self) -> Padconfig28RxactiveR {
        Padconfig28RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig28_drv_str(&self) -> Padconfig28DrvStrR {
        Padconfig28DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig28_tx_dis(&self) -> Padconfig28TxDisR {
        Padconfig28TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig28_lock(&self) -> Padconfig28LockR {
        Padconfig28LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig28_muxmode(&mut self) -> Padconfig28MuxmodeW<Cfg0Padconfig28Spec> {
        Padconfig28MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig28_debounce_sel(&mut self) -> Padconfig28DebounceSelW<Cfg0Padconfig28Spec> {
        Padconfig28DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig28_st_en(&mut self) -> Padconfig28StEnW<Cfg0Padconfig28Spec> {
        Padconfig28StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig28_pulluden(&mut self) -> Padconfig28PulludenW<Cfg0Padconfig28Spec> {
        Padconfig28PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig28_pulltypesel(&mut self) -> Padconfig28PulltypeselW<Cfg0Padconfig28Spec> {
        Padconfig28PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig28_rxactive(&mut self) -> Padconfig28RxactiveW<Cfg0Padconfig28Spec> {
        Padconfig28RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig28_drv_str(&mut self) -> Padconfig28DrvStrW<Cfg0Padconfig28Spec> {
        Padconfig28DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig28_tx_dis(&mut self) -> Padconfig28TxDisW<Cfg0Padconfig28Spec> {
        Padconfig28TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig28_lock(&mut self) -> Padconfig28LockW<Cfg0Padconfig28Spec> {
        Padconfig28LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig28Spec;
impl crate::RegisterSpec for Cfg0Padconfig28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig28::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig28Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig28::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG28 to value 0x0026_4000"]
impl crate::Resettable for Cfg0Padconfig28Spec {
    const RESET_VALUE: u32 = 0x0026_4000;
}
