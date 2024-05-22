#[doc = "Register `CFG0_PADCONFIG32` reader"]
pub type R = crate::R<Cfg0Padconfig32Spec>;
#[doc = "Register `CFG0_PADCONFIG32` writer"]
pub type W = crate::W<Cfg0Padconfig32Spec>;
#[doc = "Field `PADCONFIG32_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig32MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG32_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig32MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG32_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig32DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG32_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig32DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG32_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig32StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG32_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig32StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG32_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig32PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG32_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig32PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG32_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig32PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG32_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig32PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG32_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig32RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG32_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig32RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG32_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig32DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG32_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig32DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG32_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig32TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG32_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig32TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG32_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig32LockR = crate::BitReader;
#[doc = "Field `PADCONFIG32_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig32LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig32_muxmode(&self) -> Padconfig32MuxmodeR {
        Padconfig32MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig32_debounce_sel(&self) -> Padconfig32DebounceSelR {
        Padconfig32DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig32_st_en(&self) -> Padconfig32StEnR {
        Padconfig32StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig32_pulluden(&self) -> Padconfig32PulludenR {
        Padconfig32PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig32_pulltypesel(&self) -> Padconfig32PulltypeselR {
        Padconfig32PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig32_rxactive(&self) -> Padconfig32RxactiveR {
        Padconfig32RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig32_drv_str(&self) -> Padconfig32DrvStrR {
        Padconfig32DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig32_tx_dis(&self) -> Padconfig32TxDisR {
        Padconfig32TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig32_lock(&self) -> Padconfig32LockR {
        Padconfig32LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig32_muxmode(&mut self) -> Padconfig32MuxmodeW<Cfg0Padconfig32Spec> {
        Padconfig32MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig32_debounce_sel(&mut self) -> Padconfig32DebounceSelW<Cfg0Padconfig32Spec> {
        Padconfig32DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig32_st_en(&mut self) -> Padconfig32StEnW<Cfg0Padconfig32Spec> {
        Padconfig32StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig32_pulluden(&mut self) -> Padconfig32PulludenW<Cfg0Padconfig32Spec> {
        Padconfig32PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig32_pulltypesel(&mut self) -> Padconfig32PulltypeselW<Cfg0Padconfig32Spec> {
        Padconfig32PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig32_rxactive(&mut self) -> Padconfig32RxactiveW<Cfg0Padconfig32Spec> {
        Padconfig32RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig32_drv_str(&mut self) -> Padconfig32DrvStrW<Cfg0Padconfig32Spec> {
        Padconfig32DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig32_tx_dis(&mut self) -> Padconfig32TxDisW<Cfg0Padconfig32Spec> {
        Padconfig32TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig32_lock(&mut self) -> Padconfig32LockW<Cfg0Padconfig32Spec> {
        Padconfig32LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig32Spec;
impl crate::RegisterSpec for Cfg0Padconfig32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig32::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig32Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig32::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG32 to value 0x0026_4000"]
impl crate::Resettable for Cfg0Padconfig32Spec {
    const RESET_VALUE: u32 = 0x0026_4000;
}
