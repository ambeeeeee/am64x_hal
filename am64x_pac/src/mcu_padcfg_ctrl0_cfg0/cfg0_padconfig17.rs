#[doc = "Register `CFG0_PADCONFIG17` reader"]
pub type R = crate::R<Cfg0Padconfig17Spec>;
#[doc = "Register `CFG0_PADCONFIG17` writer"]
pub type W = crate::W<Cfg0Padconfig17Spec>;
#[doc = "Field `PADCONFIG17_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig17MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG17_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig17MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG17_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig17DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG17_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig17DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG17_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig17StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG17_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig17StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG17_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig17PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG17_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig17PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG17_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig17PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG17_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig17PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG17_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig17RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG17_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig17RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG17_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig17DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG17_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig17DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG17_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig17TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG17_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig17TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG17_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig17LockR = crate::BitReader;
#[doc = "Field `PADCONFIG17_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig17LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig17_muxmode(&self) -> Padconfig17MuxmodeR {
        Padconfig17MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig17_debounce_sel(&self) -> Padconfig17DebounceSelR {
        Padconfig17DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig17_st_en(&self) -> Padconfig17StEnR {
        Padconfig17StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig17_pulluden(&self) -> Padconfig17PulludenR {
        Padconfig17PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig17_pulltypesel(&self) -> Padconfig17PulltypeselR {
        Padconfig17PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig17_rxactive(&self) -> Padconfig17RxactiveR {
        Padconfig17RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig17_drv_str(&self) -> Padconfig17DrvStrR {
        Padconfig17DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig17_tx_dis(&self) -> Padconfig17TxDisR {
        Padconfig17TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig17_lock(&self) -> Padconfig17LockR {
        Padconfig17LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig17_muxmode(&mut self) -> Padconfig17MuxmodeW<Cfg0Padconfig17Spec> {
        Padconfig17MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig17_debounce_sel(&mut self) -> Padconfig17DebounceSelW<Cfg0Padconfig17Spec> {
        Padconfig17DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig17_st_en(&mut self) -> Padconfig17StEnW<Cfg0Padconfig17Spec> {
        Padconfig17StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig17_pulluden(&mut self) -> Padconfig17PulludenW<Cfg0Padconfig17Spec> {
        Padconfig17PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig17_pulltypesel(&mut self) -> Padconfig17PulltypeselW<Cfg0Padconfig17Spec> {
        Padconfig17PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig17_rxactive(&mut self) -> Padconfig17RxactiveW<Cfg0Padconfig17Spec> {
        Padconfig17RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig17_drv_str(&mut self) -> Padconfig17DrvStrW<Cfg0Padconfig17Spec> {
        Padconfig17DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig17_tx_dis(&mut self) -> Padconfig17TxDisW<Cfg0Padconfig17Spec> {
        Padconfig17TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig17_lock(&mut self) -> Padconfig17LockW<Cfg0Padconfig17Spec> {
        Padconfig17LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig17Spec;
impl crate::RegisterSpec for Cfg0Padconfig17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig17::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig17Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig17::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG17 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig17Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
