#[doc = "Register `CFG0_PADCONFIG5` reader"]
pub type R = crate::R<Cfg0Padconfig5Spec>;
#[doc = "Register `CFG0_PADCONFIG5` writer"]
pub type W = crate::W<Cfg0Padconfig5Spec>;
#[doc = "Field `PADCONFIG5_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig5MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG5_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig5MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG5_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig5DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG5_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig5DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG5_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig5StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG5_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig5StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG5_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig5PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG5_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig5PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG5_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig5PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG5_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig5PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG5_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig5RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG5_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig5RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG5_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig5DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG5_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig5DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG5_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig5TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG5_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig5TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG5_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig5LockR = crate::BitReader;
#[doc = "Field `PADCONFIG5_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig5LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig5_muxmode(&self) -> Padconfig5MuxmodeR {
        Padconfig5MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig5_debounce_sel(&self) -> Padconfig5DebounceSelR {
        Padconfig5DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig5_st_en(&self) -> Padconfig5StEnR {
        Padconfig5StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig5_pulluden(&self) -> Padconfig5PulludenR {
        Padconfig5PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig5_pulltypesel(&self) -> Padconfig5PulltypeselR {
        Padconfig5PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig5_rxactive(&self) -> Padconfig5RxactiveR {
        Padconfig5RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig5_drv_str(&self) -> Padconfig5DrvStrR {
        Padconfig5DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig5_tx_dis(&self) -> Padconfig5TxDisR {
        Padconfig5TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig5_lock(&self) -> Padconfig5LockR {
        Padconfig5LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_muxmode(&mut self) -> Padconfig5MuxmodeW<Cfg0Padconfig5Spec> {
        Padconfig5MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_debounce_sel(&mut self) -> Padconfig5DebounceSelW<Cfg0Padconfig5Spec> {
        Padconfig5DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_st_en(&mut self) -> Padconfig5StEnW<Cfg0Padconfig5Spec> {
        Padconfig5StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_pulluden(&mut self) -> Padconfig5PulludenW<Cfg0Padconfig5Spec> {
        Padconfig5PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_pulltypesel(&mut self) -> Padconfig5PulltypeselW<Cfg0Padconfig5Spec> {
        Padconfig5PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_rxactive(&mut self) -> Padconfig5RxactiveW<Cfg0Padconfig5Spec> {
        Padconfig5RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_drv_str(&mut self) -> Padconfig5DrvStrW<Cfg0Padconfig5Spec> {
        Padconfig5DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_tx_dis(&mut self) -> Padconfig5TxDisW<Cfg0Padconfig5Spec> {
        Padconfig5TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_lock(&mut self) -> Padconfig5LockW<Cfg0Padconfig5Spec> {
        Padconfig5LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig5Spec;
impl crate::RegisterSpec for Cfg0Padconfig5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig5::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig5Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig5::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG5 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig5Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}