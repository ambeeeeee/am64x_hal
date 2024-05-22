#[doc = "Register `CFG0_PADCONFIG25` reader"]
pub type R = crate::R<Cfg0Padconfig25Spec>;
#[doc = "Register `CFG0_PADCONFIG25` writer"]
pub type W = crate::W<Cfg0Padconfig25Spec>;
#[doc = "Field `PADCONFIG25_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig25MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG25_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig25MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG25_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig25DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG25_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig25DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG25_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig25StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG25_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig25StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG25_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig25PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG25_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig25PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG25_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig25PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG25_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig25PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG25_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig25RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG25_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig25RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG25_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig25DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG25_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig25DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG25_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig25TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG25_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig25TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG25_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig25LockR = crate::BitReader;
#[doc = "Field `PADCONFIG25_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig25LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig25_muxmode(&self) -> Padconfig25MuxmodeR {
        Padconfig25MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig25_debounce_sel(&self) -> Padconfig25DebounceSelR {
        Padconfig25DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig25_st_en(&self) -> Padconfig25StEnR {
        Padconfig25StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig25_pulluden(&self) -> Padconfig25PulludenR {
        Padconfig25PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig25_pulltypesel(&self) -> Padconfig25PulltypeselR {
        Padconfig25PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig25_rxactive(&self) -> Padconfig25RxactiveR {
        Padconfig25RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig25_drv_str(&self) -> Padconfig25DrvStrR {
        Padconfig25DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig25_tx_dis(&self) -> Padconfig25TxDisR {
        Padconfig25TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig25_lock(&self) -> Padconfig25LockR {
        Padconfig25LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig25_muxmode(&mut self) -> Padconfig25MuxmodeW<Cfg0Padconfig25Spec> {
        Padconfig25MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig25_debounce_sel(&mut self) -> Padconfig25DebounceSelW<Cfg0Padconfig25Spec> {
        Padconfig25DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig25_st_en(&mut self) -> Padconfig25StEnW<Cfg0Padconfig25Spec> {
        Padconfig25StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig25_pulluden(&mut self) -> Padconfig25PulludenW<Cfg0Padconfig25Spec> {
        Padconfig25PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig25_pulltypesel(&mut self) -> Padconfig25PulltypeselW<Cfg0Padconfig25Spec> {
        Padconfig25PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig25_rxactive(&mut self) -> Padconfig25RxactiveW<Cfg0Padconfig25Spec> {
        Padconfig25RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig25_drv_str(&mut self) -> Padconfig25DrvStrW<Cfg0Padconfig25Spec> {
        Padconfig25DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig25_tx_dis(&mut self) -> Padconfig25TxDisW<Cfg0Padconfig25Spec> {
        Padconfig25TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig25_lock(&mut self) -> Padconfig25LockW<Cfg0Padconfig25Spec> {
        Padconfig25LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig25Spec;
impl crate::RegisterSpec for Cfg0Padconfig25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig25::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig25Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig25::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG25 to value 0x0004_4000"]
impl crate::Resettable for Cfg0Padconfig25Spec {
    const RESET_VALUE: u32 = 0x0004_4000;
}
