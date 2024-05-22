#[doc = "Register `CFG0_PADCONFIG12` reader"]
pub type R = crate::R<Cfg0Padconfig12Spec>;
#[doc = "Register `CFG0_PADCONFIG12` writer"]
pub type W = crate::W<Cfg0Padconfig12Spec>;
#[doc = "Field `PADCONFIG12_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig12MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG12_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig12MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG12_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig12DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG12_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig12DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG12_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig12StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG12_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig12StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG12_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig12PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG12_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig12PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG12_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig12PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG12_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig12PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG12_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig12RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG12_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig12RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG12_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig12DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG12_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig12DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG12_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig12TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG12_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig12TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG12_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig12LockR = crate::BitReader;
#[doc = "Field `PADCONFIG12_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig12LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig12_muxmode(&self) -> Padconfig12MuxmodeR {
        Padconfig12MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig12_debounce_sel(&self) -> Padconfig12DebounceSelR {
        Padconfig12DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig12_st_en(&self) -> Padconfig12StEnR {
        Padconfig12StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig12_pulluden(&self) -> Padconfig12PulludenR {
        Padconfig12PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig12_pulltypesel(&self) -> Padconfig12PulltypeselR {
        Padconfig12PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig12_rxactive(&self) -> Padconfig12RxactiveR {
        Padconfig12RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig12_drv_str(&self) -> Padconfig12DrvStrR {
        Padconfig12DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig12_tx_dis(&self) -> Padconfig12TxDisR {
        Padconfig12TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig12_lock(&self) -> Padconfig12LockR {
        Padconfig12LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig12_muxmode(&mut self) -> Padconfig12MuxmodeW<Cfg0Padconfig12Spec> {
        Padconfig12MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig12_debounce_sel(&mut self) -> Padconfig12DebounceSelW<Cfg0Padconfig12Spec> {
        Padconfig12DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig12_st_en(&mut self) -> Padconfig12StEnW<Cfg0Padconfig12Spec> {
        Padconfig12StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig12_pulluden(&mut self) -> Padconfig12PulludenW<Cfg0Padconfig12Spec> {
        Padconfig12PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig12_pulltypesel(&mut self) -> Padconfig12PulltypeselW<Cfg0Padconfig12Spec> {
        Padconfig12PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig12_rxactive(&mut self) -> Padconfig12RxactiveW<Cfg0Padconfig12Spec> {
        Padconfig12RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig12_drv_str(&mut self) -> Padconfig12DrvStrW<Cfg0Padconfig12Spec> {
        Padconfig12DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig12_tx_dis(&mut self) -> Padconfig12TxDisW<Cfg0Padconfig12Spec> {
        Padconfig12TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig12_lock(&mut self) -> Padconfig12LockW<Cfg0Padconfig12Spec> {
        Padconfig12LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig12Spec;
impl crate::RegisterSpec for Cfg0Padconfig12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig12::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig12Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig12::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG12 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig12Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
