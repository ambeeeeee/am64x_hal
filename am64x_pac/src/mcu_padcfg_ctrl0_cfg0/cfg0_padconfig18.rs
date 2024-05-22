#[doc = "Register `CFG0_PADCONFIG18` reader"]
pub type R = crate::R<Cfg0Padconfig18Spec>;
#[doc = "Register `CFG0_PADCONFIG18` writer"]
pub type W = crate::W<Cfg0Padconfig18Spec>;
#[doc = "Field `PADCONFIG18_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig18MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG18_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig18MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG18_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig18DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG18_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig18DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG18_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig18StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG18_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig18StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG18_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig18PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG18_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig18PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG18_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig18PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG18_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig18PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG18_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig18RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG18_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig18RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG18_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig18DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG18_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig18DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG18_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig18TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG18_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig18TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG18_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig18LockR = crate::BitReader;
#[doc = "Field `PADCONFIG18_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig18LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig18_muxmode(&self) -> Padconfig18MuxmodeR {
        Padconfig18MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig18_debounce_sel(&self) -> Padconfig18DebounceSelR {
        Padconfig18DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig18_st_en(&self) -> Padconfig18StEnR {
        Padconfig18StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig18_pulluden(&self) -> Padconfig18PulludenR {
        Padconfig18PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig18_pulltypesel(&self) -> Padconfig18PulltypeselR {
        Padconfig18PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig18_rxactive(&self) -> Padconfig18RxactiveR {
        Padconfig18RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig18_drv_str(&self) -> Padconfig18DrvStrR {
        Padconfig18DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig18_tx_dis(&self) -> Padconfig18TxDisR {
        Padconfig18TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig18_lock(&self) -> Padconfig18LockR {
        Padconfig18LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig18_muxmode(&mut self) -> Padconfig18MuxmodeW<Cfg0Padconfig18Spec> {
        Padconfig18MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig18_debounce_sel(&mut self) -> Padconfig18DebounceSelW<Cfg0Padconfig18Spec> {
        Padconfig18DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig18_st_en(&mut self) -> Padconfig18StEnW<Cfg0Padconfig18Spec> {
        Padconfig18StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig18_pulluden(&mut self) -> Padconfig18PulludenW<Cfg0Padconfig18Spec> {
        Padconfig18PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig18_pulltypesel(&mut self) -> Padconfig18PulltypeselW<Cfg0Padconfig18Spec> {
        Padconfig18PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig18_rxactive(&mut self) -> Padconfig18RxactiveW<Cfg0Padconfig18Spec> {
        Padconfig18RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig18_drv_str(&mut self) -> Padconfig18DrvStrW<Cfg0Padconfig18Spec> {
        Padconfig18DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig18_tx_dis(&mut self) -> Padconfig18TxDisW<Cfg0Padconfig18Spec> {
        Padconfig18TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig18_lock(&mut self) -> Padconfig18LockW<Cfg0Padconfig18Spec> {
        Padconfig18LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig18Spec;
impl crate::RegisterSpec for Cfg0Padconfig18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig18::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig18Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig18::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG18 to value 0x0005_4007"]
impl crate::Resettable for Cfg0Padconfig18Spec {
    const RESET_VALUE: u32 = 0x0005_4007;
}
