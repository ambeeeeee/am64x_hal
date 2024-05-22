#[doc = "Register `CFG0_PADCONFIG30` reader"]
pub type R = crate::R<Cfg0Padconfig30Spec>;
#[doc = "Register `CFG0_PADCONFIG30` writer"]
pub type W = crate::W<Cfg0Padconfig30Spec>;
#[doc = "Field `PADCONFIG30_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig30MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG30_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig30MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG30_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig30DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG30_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig30DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG30_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig30StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG30_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig30StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG30_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig30PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG30_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig30PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG30_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig30PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG30_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig30PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG30_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig30RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG30_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig30RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG30_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig30DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG30_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig30DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG30_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig30TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG30_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig30TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG30_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig30LockR = crate::BitReader;
#[doc = "Field `PADCONFIG30_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig30LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig30_muxmode(&self) -> Padconfig30MuxmodeR {
        Padconfig30MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig30_debounce_sel(&self) -> Padconfig30DebounceSelR {
        Padconfig30DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig30_st_en(&self) -> Padconfig30StEnR {
        Padconfig30StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig30_pulluden(&self) -> Padconfig30PulludenR {
        Padconfig30PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig30_pulltypesel(&self) -> Padconfig30PulltypeselR {
        Padconfig30PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig30_rxactive(&self) -> Padconfig30RxactiveR {
        Padconfig30RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig30_drv_str(&self) -> Padconfig30DrvStrR {
        Padconfig30DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig30_tx_dis(&self) -> Padconfig30TxDisR {
        Padconfig30TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig30_lock(&self) -> Padconfig30LockR {
        Padconfig30LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_muxmode(&mut self) -> Padconfig30MuxmodeW<Cfg0Padconfig30Spec> {
        Padconfig30MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_debounce_sel(&mut self) -> Padconfig30DebounceSelW<Cfg0Padconfig30Spec> {
        Padconfig30DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_st_en(&mut self) -> Padconfig30StEnW<Cfg0Padconfig30Spec> {
        Padconfig30StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_pulluden(&mut self) -> Padconfig30PulludenW<Cfg0Padconfig30Spec> {
        Padconfig30PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_pulltypesel(&mut self) -> Padconfig30PulltypeselW<Cfg0Padconfig30Spec> {
        Padconfig30PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_rxactive(&mut self) -> Padconfig30RxactiveW<Cfg0Padconfig30Spec> {
        Padconfig30RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_drv_str(&mut self) -> Padconfig30DrvStrW<Cfg0Padconfig30Spec> {
        Padconfig30DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_tx_dis(&mut self) -> Padconfig30TxDisW<Cfg0Padconfig30Spec> {
        Padconfig30TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_lock(&mut self) -> Padconfig30LockW<Cfg0Padconfig30Spec> {
        Padconfig30LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig30Spec;
impl crate::RegisterSpec for Cfg0Padconfig30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig30::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig30Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig30::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG30 to value 0x0026_4000"]
impl crate::Resettable for Cfg0Padconfig30Spec {
    const RESET_VALUE: u32 = 0x0026_4000;
}
