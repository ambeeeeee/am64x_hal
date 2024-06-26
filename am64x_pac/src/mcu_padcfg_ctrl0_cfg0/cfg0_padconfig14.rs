#[doc = "Register `CFG0_PADCONFIG14` reader"]
pub type R = crate::R<Cfg0Padconfig14Spec>;
#[doc = "Register `CFG0_PADCONFIG14` writer"]
pub type W = crate::W<Cfg0Padconfig14Spec>;
#[doc = "Field `PADCONFIG14_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig14MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG14_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig14MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG14_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig14DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG14_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig14DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG14_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig14StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG14_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig14StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG14_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig14PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG14_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig14PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG14_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig14PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG14_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig14PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG14_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig14RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG14_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig14RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG14_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig14DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG14_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig14DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG14_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig14TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG14_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig14TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG14_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig14LockR = crate::BitReader;
#[doc = "Field `PADCONFIG14_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig14LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig14_muxmode(&self) -> Padconfig14MuxmodeR {
        Padconfig14MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig14_debounce_sel(&self) -> Padconfig14DebounceSelR {
        Padconfig14DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig14_st_en(&self) -> Padconfig14StEnR {
        Padconfig14StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig14_pulluden(&self) -> Padconfig14PulludenR {
        Padconfig14PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig14_pulltypesel(&self) -> Padconfig14PulltypeselR {
        Padconfig14PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig14_rxactive(&self) -> Padconfig14RxactiveR {
        Padconfig14RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig14_drv_str(&self) -> Padconfig14DrvStrR {
        Padconfig14DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig14_tx_dis(&self) -> Padconfig14TxDisR {
        Padconfig14TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig14_lock(&self) -> Padconfig14LockR {
        Padconfig14LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig14_muxmode(&mut self) -> Padconfig14MuxmodeW<Cfg0Padconfig14Spec> {
        Padconfig14MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig14_debounce_sel(&mut self) -> Padconfig14DebounceSelW<Cfg0Padconfig14Spec> {
        Padconfig14DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig14_st_en(&mut self) -> Padconfig14StEnW<Cfg0Padconfig14Spec> {
        Padconfig14StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig14_pulluden(&mut self) -> Padconfig14PulludenW<Cfg0Padconfig14Spec> {
        Padconfig14PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig14_pulltypesel(&mut self) -> Padconfig14PulltypeselW<Cfg0Padconfig14Spec> {
        Padconfig14PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig14_rxactive(&mut self) -> Padconfig14RxactiveW<Cfg0Padconfig14Spec> {
        Padconfig14RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig14_drv_str(&mut self) -> Padconfig14DrvStrW<Cfg0Padconfig14Spec> {
        Padconfig14DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig14_tx_dis(&mut self) -> Padconfig14TxDisW<Cfg0Padconfig14Spec> {
        Padconfig14TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig14_lock(&mut self) -> Padconfig14LockW<Cfg0Padconfig14Spec> {
        Padconfig14LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig14Spec;
impl crate::RegisterSpec for Cfg0Padconfig14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig14::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig14Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig14::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG14 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig14Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
