#[doc = "Register `CFG0_PADCONFIG20` reader"]
pub type R = crate::R<Cfg0Padconfig20Spec>;
#[doc = "Register `CFG0_PADCONFIG20` writer"]
pub type W = crate::W<Cfg0Padconfig20Spec>;
#[doc = "Field `PADCONFIG20_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig20MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG20_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig20MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG20_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig20DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG20_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig20DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG20_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig20StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG20_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig20StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG20_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig20PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG20_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig20PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG20_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig20PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG20_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig20PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG20_RXACTIVE` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig20RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG20_RXACTIVE` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig20RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG20_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig20DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG20_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig20DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG20_TX_DIS` reader - 21:21\\]
Driver Disable"]
pub type Padconfig20TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG20_TX_DIS` writer - 21:21\\]
Driver Disable"]
pub type Padconfig20TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG20_LOCK` reader - 31:31\\]
Lock"]
pub type Padconfig20LockR = crate::BitReader;
#[doc = "Field `PADCONFIG20_LOCK` writer - 31:31\\]
Lock"]
pub type Padconfig20LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig20_muxmode(&self) -> Padconfig20MuxmodeR {
        Padconfig20MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig20_debounce_sel(&self) -> Padconfig20DebounceSelR {
        Padconfig20DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig20_st_en(&self) -> Padconfig20StEnR {
        Padconfig20StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig20_pulluden(&self) -> Padconfig20PulludenR {
        Padconfig20PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig20_pulltypesel(&self) -> Padconfig20PulltypeselR {
        Padconfig20PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig20_rxactive(&self) -> Padconfig20RxactiveR {
        Padconfig20RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig20_drv_str(&self) -> Padconfig20DrvStrR {
        Padconfig20DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig20_tx_dis(&self) -> Padconfig20TxDisR {
        Padconfig20TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig20_lock(&self) -> Padconfig20LockR {
        Padconfig20LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig20_muxmode(&mut self) -> Padconfig20MuxmodeW<Cfg0Padconfig20Spec> {
        Padconfig20MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig20_debounce_sel(&mut self) -> Padconfig20DebounceSelW<Cfg0Padconfig20Spec> {
        Padconfig20DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig20_st_en(&mut self) -> Padconfig20StEnW<Cfg0Padconfig20Spec> {
        Padconfig20StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig20_pulluden(&mut self) -> Padconfig20PulludenW<Cfg0Padconfig20Spec> {
        Padconfig20PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig20_pulltypesel(&mut self) -> Padconfig20PulltypeselW<Cfg0Padconfig20Spec> {
        Padconfig20PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig20_rxactive(&mut self) -> Padconfig20RxactiveW<Cfg0Padconfig20Spec> {
        Padconfig20RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig20_drv_str(&mut self) -> Padconfig20DrvStrW<Cfg0Padconfig20Spec> {
        Padconfig20DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig20_tx_dis(&mut self) -> Padconfig20TxDisW<Cfg0Padconfig20Spec> {
        Padconfig20TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig20_lock(&mut self) -> Padconfig20LockW<Cfg0Padconfig20Spec> {
        Padconfig20LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig20Spec;
impl crate::RegisterSpec for Cfg0Padconfig20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig20::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig20Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig20::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG20 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig20Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
