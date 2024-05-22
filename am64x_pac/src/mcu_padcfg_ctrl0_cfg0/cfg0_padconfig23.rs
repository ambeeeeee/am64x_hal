#[doc = "Register `CFG0_PADCONFIG23` reader"]
pub type R = crate::R<Cfg0Padconfig23Spec>;
#[doc = "Register `CFG0_PADCONFIG23` writer"]
pub type W = crate::W<Cfg0Padconfig23Spec>;
#[doc = "Field `PADCONFIG23_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig23MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG23_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig23MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG23_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig23DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG23_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig23DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG23_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig23StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG23_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig23StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG23_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig23PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG23_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig23PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG23_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig23PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG23_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig23PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG23_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig23RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG23_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig23RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG23_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig23DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG23_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig23DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG23_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig23TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG23_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig23TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG23_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig23LockR = crate::BitReader;
#[doc = "Field `PADCONFIG23_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig23LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig23_muxmode(&self) -> Padconfig23MuxmodeR {
        Padconfig23MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig23_debounce_sel(&self) -> Padconfig23DebounceSelR {
        Padconfig23DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig23_st_en(&self) -> Padconfig23StEnR {
        Padconfig23StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig23_pulluden(&self) -> Padconfig23PulludenR {
        Padconfig23PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig23_pulltypesel(&self) -> Padconfig23PulltypeselR {
        Padconfig23PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig23_rxactive(&self) -> Padconfig23RxactiveR {
        Padconfig23RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig23_drv_str(&self) -> Padconfig23DrvStrR {
        Padconfig23DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig23_tx_dis(&self) -> Padconfig23TxDisR {
        Padconfig23TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig23_lock(&self) -> Padconfig23LockR {
        Padconfig23LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig23_muxmode(&mut self) -> Padconfig23MuxmodeW<Cfg0Padconfig23Spec> {
        Padconfig23MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig23_debounce_sel(&mut self) -> Padconfig23DebounceSelW<Cfg0Padconfig23Spec> {
        Padconfig23DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig23_st_en(&mut self) -> Padconfig23StEnW<Cfg0Padconfig23Spec> {
        Padconfig23StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig23_pulluden(&mut self) -> Padconfig23PulludenW<Cfg0Padconfig23Spec> {
        Padconfig23PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig23_pulltypesel(&mut self) -> Padconfig23PulltypeselW<Cfg0Padconfig23Spec> {
        Padconfig23PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig23_rxactive(&mut self) -> Padconfig23RxactiveW<Cfg0Padconfig23Spec> {
        Padconfig23RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig23_drv_str(&mut self) -> Padconfig23DrvStrW<Cfg0Padconfig23Spec> {
        Padconfig23DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig23_tx_dis(&mut self) -> Padconfig23TxDisW<Cfg0Padconfig23Spec> {
        Padconfig23TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig23_lock(&mut self) -> Padconfig23LockW<Cfg0Padconfig23Spec> {
        Padconfig23LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig23Spec;
impl crate::RegisterSpec for Cfg0Padconfig23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig23::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig23Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig23::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG23 to value 0x0001_4000"]
impl crate::Resettable for Cfg0Padconfig23Spec {
    const RESET_VALUE: u32 = 0x0001_4000;
}
