#[doc = "Register `CFG0_PADCONFIG29` reader"]
pub type R = crate::R<Cfg0Padconfig29Spec>;
#[doc = "Register `CFG0_PADCONFIG29` writer"]
pub type W = crate::W<Cfg0Padconfig29Spec>;
#[doc = "Field `PADCONFIG29_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig29MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG29_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig29MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG29_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig29DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG29_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig29DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG29_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig29StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG29_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig29StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG29_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig29PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG29_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig29PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG29_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig29PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG29_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig29PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG29_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig29RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG29_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig29RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG29_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig29DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG29_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig29DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG29_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig29TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG29_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig29TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG29_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig29LockR = crate::BitReader;
#[doc = "Field `PADCONFIG29_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig29LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig29_muxmode(&self) -> Padconfig29MuxmodeR {
        Padconfig29MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig29_debounce_sel(&self) -> Padconfig29DebounceSelR {
        Padconfig29DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig29_st_en(&self) -> Padconfig29StEnR {
        Padconfig29StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig29_pulluden(&self) -> Padconfig29PulludenR {
        Padconfig29PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig29_pulltypesel(&self) -> Padconfig29PulltypeselR {
        Padconfig29PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig29_rxactive(&self) -> Padconfig29RxactiveR {
        Padconfig29RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig29_drv_str(&self) -> Padconfig29DrvStrR {
        Padconfig29DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig29_tx_dis(&self) -> Padconfig29TxDisR {
        Padconfig29TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig29_lock(&self) -> Padconfig29LockR {
        Padconfig29LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig29_muxmode(&mut self) -> Padconfig29MuxmodeW<Cfg0Padconfig29Spec> {
        Padconfig29MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig29_debounce_sel(&mut self) -> Padconfig29DebounceSelW<Cfg0Padconfig29Spec> {
        Padconfig29DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig29_st_en(&mut self) -> Padconfig29StEnW<Cfg0Padconfig29Spec> {
        Padconfig29StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig29_pulluden(&mut self) -> Padconfig29PulludenW<Cfg0Padconfig29Spec> {
        Padconfig29PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig29_pulltypesel(&mut self) -> Padconfig29PulltypeselW<Cfg0Padconfig29Spec> {
        Padconfig29PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig29_rxactive(&mut self) -> Padconfig29RxactiveW<Cfg0Padconfig29Spec> {
        Padconfig29RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig29_drv_str(&mut self) -> Padconfig29DrvStrW<Cfg0Padconfig29Spec> {
        Padconfig29DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig29_tx_dis(&mut self) -> Padconfig29TxDisW<Cfg0Padconfig29Spec> {
        Padconfig29TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig29_lock(&mut self) -> Padconfig29LockW<Cfg0Padconfig29Spec> {
        Padconfig29LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig29Spec;
impl crate::RegisterSpec for Cfg0Padconfig29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig29::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig29Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig29::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG29 to value 0x0002_4000"]
impl crate::Resettable for Cfg0Padconfig29Spec {
    const RESET_VALUE: u32 = 0x0002_4000;
}
