#[doc = "Register `CFG0_PADCONFIG19` reader"]
pub type R = crate::R<Cfg0Padconfig19Spec>;
#[doc = "Register `CFG0_PADCONFIG19` writer"]
pub type W = crate::W<Cfg0Padconfig19Spec>;
#[doc = "Field `PADCONFIG19_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig19MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG19_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig19MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG19_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig19DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG19_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig19DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG19_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig19StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG19_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig19StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG19_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig19PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG19_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig19PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG19_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig19PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG19_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig19PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG19_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig19RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG19_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig19RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG19_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig19DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG19_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig19DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG19_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig19TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG19_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig19TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG19_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig19LockR = crate::BitReader;
#[doc = "Field `PADCONFIG19_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig19LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig19_muxmode(&self) -> Padconfig19MuxmodeR {
        Padconfig19MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig19_debounce_sel(&self) -> Padconfig19DebounceSelR {
        Padconfig19DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig19_st_en(&self) -> Padconfig19StEnR {
        Padconfig19StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig19_pulluden(&self) -> Padconfig19PulludenR {
        Padconfig19PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig19_pulltypesel(&self) -> Padconfig19PulltypeselR {
        Padconfig19PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig19_rxactive(&self) -> Padconfig19RxactiveR {
        Padconfig19RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig19_drv_str(&self) -> Padconfig19DrvStrR {
        Padconfig19DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig19_tx_dis(&self) -> Padconfig19TxDisR {
        Padconfig19TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig19_lock(&self) -> Padconfig19LockR {
        Padconfig19LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig19_muxmode(&mut self) -> Padconfig19MuxmodeW<Cfg0Padconfig19Spec> {
        Padconfig19MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig19_debounce_sel(&mut self) -> Padconfig19DebounceSelW<Cfg0Padconfig19Spec> {
        Padconfig19DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig19_st_en(&mut self) -> Padconfig19StEnW<Cfg0Padconfig19Spec> {
        Padconfig19StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig19_pulluden(&mut self) -> Padconfig19PulludenW<Cfg0Padconfig19Spec> {
        Padconfig19PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig19_pulltypesel(&mut self) -> Padconfig19PulltypeselW<Cfg0Padconfig19Spec> {
        Padconfig19PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig19_rxactive(&mut self) -> Padconfig19RxactiveW<Cfg0Padconfig19Spec> {
        Padconfig19RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig19_drv_str(&mut self) -> Padconfig19DrvStrW<Cfg0Padconfig19Spec> {
        Padconfig19DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig19_tx_dis(&mut self) -> Padconfig19TxDisW<Cfg0Padconfig19Spec> {
        Padconfig19TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig19_lock(&mut self) -> Padconfig19LockW<Cfg0Padconfig19Spec> {
        Padconfig19LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig19Spec;
impl crate::RegisterSpec for Cfg0Padconfig19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig19::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig19Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig19::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG19 to value 0x0005_4007"]
impl crate::Resettable for Cfg0Padconfig19Spec {
    const RESET_VALUE: u32 = 0x0005_4007;
}
