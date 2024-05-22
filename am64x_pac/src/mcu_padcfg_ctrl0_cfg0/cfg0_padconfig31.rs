#[doc = "Register `CFG0_PADCONFIG31` reader"]
pub type R = crate::R<Cfg0Padconfig31Spec>;
#[doc = "Register `CFG0_PADCONFIG31` writer"]
pub type W = crate::W<Cfg0Padconfig31Spec>;
#[doc = "Field `PADCONFIG31_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig31MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG31_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig31MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG31_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig31DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG31_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig31DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG31_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig31StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG31_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig31StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG31_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig31PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG31_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig31PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG31_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig31PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG31_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig31PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG31_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig31RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG31_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig31RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG31_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig31DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG31_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig31DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG31_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig31TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG31_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig31TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG31_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig31LockR = crate::BitReader;
#[doc = "Field `PADCONFIG31_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig31LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig31_muxmode(&self) -> Padconfig31MuxmodeR {
        Padconfig31MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig31_debounce_sel(&self) -> Padconfig31DebounceSelR {
        Padconfig31DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig31_st_en(&self) -> Padconfig31StEnR {
        Padconfig31StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig31_pulluden(&self) -> Padconfig31PulludenR {
        Padconfig31PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig31_pulltypesel(&self) -> Padconfig31PulltypeselR {
        Padconfig31PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig31_rxactive(&self) -> Padconfig31RxactiveR {
        Padconfig31RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig31_drv_str(&self) -> Padconfig31DrvStrR {
        Padconfig31DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig31_tx_dis(&self) -> Padconfig31TxDisR {
        Padconfig31TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig31_lock(&self) -> Padconfig31LockR {
        Padconfig31LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig31_muxmode(&mut self) -> Padconfig31MuxmodeW<Cfg0Padconfig31Spec> {
        Padconfig31MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig31_debounce_sel(&mut self) -> Padconfig31DebounceSelW<Cfg0Padconfig31Spec> {
        Padconfig31DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig31_st_en(&mut self) -> Padconfig31StEnW<Cfg0Padconfig31Spec> {
        Padconfig31StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig31_pulluden(&mut self) -> Padconfig31PulludenW<Cfg0Padconfig31Spec> {
        Padconfig31PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig31_pulltypesel(&mut self) -> Padconfig31PulltypeselW<Cfg0Padconfig31Spec> {
        Padconfig31PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig31_rxactive(&mut self) -> Padconfig31RxactiveW<Cfg0Padconfig31Spec> {
        Padconfig31RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig31_drv_str(&mut self) -> Padconfig31DrvStrW<Cfg0Padconfig31Spec> {
        Padconfig31DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig31_tx_dis(&mut self) -> Padconfig31TxDisW<Cfg0Padconfig31Spec> {
        Padconfig31TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig31_lock(&mut self) -> Padconfig31LockW<Cfg0Padconfig31Spec> {
        Padconfig31LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig31Spec;
impl crate::RegisterSpec for Cfg0Padconfig31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig31::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig31Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig31::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG31 to value 0x0026_4000"]
impl crate::Resettable for Cfg0Padconfig31Spec {
    const RESET_VALUE: u32 = 0x0026_4000;
}
