#[doc = "Register `CFG0_PADCONFIG27` reader"]
pub type R = crate::R<Cfg0Padconfig27Spec>;
#[doc = "Register `CFG0_PADCONFIG27` writer"]
pub type W = crate::W<Cfg0Padconfig27Spec>;
#[doc = "Field `PADCONFIG27_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig27MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG27_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig27MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG27_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig27DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG27_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig27DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG27_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig27StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG27_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig27StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG27_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig27PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG27_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig27PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG27_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig27PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG27_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig27PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG27_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig27RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG27_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig27RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG27_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig27DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG27_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig27DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG27_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig27TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG27_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig27TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG27_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig27LockR = crate::BitReader;
#[doc = "Field `PADCONFIG27_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig27LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig27_muxmode(&self) -> Padconfig27MuxmodeR {
        Padconfig27MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig27_debounce_sel(&self) -> Padconfig27DebounceSelR {
        Padconfig27DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig27_st_en(&self) -> Padconfig27StEnR {
        Padconfig27StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig27_pulluden(&self) -> Padconfig27PulludenR {
        Padconfig27PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig27_pulltypesel(&self) -> Padconfig27PulltypeselR {
        Padconfig27PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig27_rxactive(&self) -> Padconfig27RxactiveR {
        Padconfig27RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig27_drv_str(&self) -> Padconfig27DrvStrR {
        Padconfig27DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig27_tx_dis(&self) -> Padconfig27TxDisR {
        Padconfig27TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig27_lock(&self) -> Padconfig27LockR {
        Padconfig27LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_muxmode(&mut self) -> Padconfig27MuxmodeW<Cfg0Padconfig27Spec> {
        Padconfig27MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_debounce_sel(&mut self) -> Padconfig27DebounceSelW<Cfg0Padconfig27Spec> {
        Padconfig27DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_st_en(&mut self) -> Padconfig27StEnW<Cfg0Padconfig27Spec> {
        Padconfig27StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_pulluden(&mut self) -> Padconfig27PulludenW<Cfg0Padconfig27Spec> {
        Padconfig27PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_pulltypesel(&mut self) -> Padconfig27PulltypeselW<Cfg0Padconfig27Spec> {
        Padconfig27PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_rxactive(&mut self) -> Padconfig27RxactiveW<Cfg0Padconfig27Spec> {
        Padconfig27RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_drv_str(&mut self) -> Padconfig27DrvStrW<Cfg0Padconfig27Spec> {
        Padconfig27DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_tx_dis(&mut self) -> Padconfig27TxDisW<Cfg0Padconfig27Spec> {
        Padconfig27TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_lock(&mut self) -> Padconfig27LockW<Cfg0Padconfig27Spec> {
        Padconfig27LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig27Spec;
impl crate::RegisterSpec for Cfg0Padconfig27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig27::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig27Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig27::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG27 to value 0x0024_4000"]
impl crate::Resettable for Cfg0Padconfig27Spec {
    const RESET_VALUE: u32 = 0x0024_4000;
}
