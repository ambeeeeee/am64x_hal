#[doc = "Register `CFG0_PADCONFIG1` reader"]
pub type R = crate::R<Cfg0Padconfig1Spec>;
#[doc = "Register `CFG0_PADCONFIG1` writer"]
pub type W = crate::W<Cfg0Padconfig1Spec>;
#[doc = "Field `PADCONFIG1_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig1MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG1_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig1MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG1_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig1DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG1_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig1DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG1_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig1StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG1_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig1StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG1_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig1PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG1_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig1PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG1_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig1PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG1_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig1PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG1_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig1RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG1_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig1RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG1_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig1DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG1_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig1DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG1_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig1TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG1_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig1TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG1_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig1LockR = crate::BitReader;
#[doc = "Field `PADCONFIG1_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig1LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig1_muxmode(&self) -> Padconfig1MuxmodeR {
        Padconfig1MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig1_debounce_sel(&self) -> Padconfig1DebounceSelR {
        Padconfig1DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig1_st_en(&self) -> Padconfig1StEnR {
        Padconfig1StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig1_pulluden(&self) -> Padconfig1PulludenR {
        Padconfig1PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig1_pulltypesel(&self) -> Padconfig1PulltypeselR {
        Padconfig1PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig1_rxactive(&self) -> Padconfig1RxactiveR {
        Padconfig1RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig1_drv_str(&self) -> Padconfig1DrvStrR {
        Padconfig1DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig1_tx_dis(&self) -> Padconfig1TxDisR {
        Padconfig1TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig1_lock(&self) -> Padconfig1LockR {
        Padconfig1LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig1_muxmode(&mut self) -> Padconfig1MuxmodeW<Cfg0Padconfig1Spec> {
        Padconfig1MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig1_debounce_sel(&mut self) -> Padconfig1DebounceSelW<Cfg0Padconfig1Spec> {
        Padconfig1DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig1_st_en(&mut self) -> Padconfig1StEnW<Cfg0Padconfig1Spec> {
        Padconfig1StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig1_pulluden(&mut self) -> Padconfig1PulludenW<Cfg0Padconfig1Spec> {
        Padconfig1PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig1_pulltypesel(&mut self) -> Padconfig1PulltypeselW<Cfg0Padconfig1Spec> {
        Padconfig1PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig1_rxactive(&mut self) -> Padconfig1RxactiveW<Cfg0Padconfig1Spec> {
        Padconfig1RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig1_drv_str(&mut self) -> Padconfig1DrvStrW<Cfg0Padconfig1Spec> {
        Padconfig1DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig1_tx_dis(&mut self) -> Padconfig1TxDisW<Cfg0Padconfig1Spec> {
        Padconfig1TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig1_lock(&mut self) -> Padconfig1LockW<Cfg0Padconfig1Spec> {
        Padconfig1LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig1Spec;
impl crate::RegisterSpec for Cfg0Padconfig1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig1::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig1::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG1 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig1Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
