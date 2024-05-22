#[doc = "Register `CFG0_PADCONFIG64` reader"]
pub type R = crate::R<Cfg0Padconfig64Spec>;
#[doc = "Register `CFG0_PADCONFIG64` writer"]
pub type W = crate::W<Cfg0Padconfig64Spec>;
#[doc = "Field `PADCONFIG64_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
pub type Padconfig64MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG64_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
pub type Padconfig64MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG64_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig64DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG64_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig64DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG64_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
pub type Padconfig64StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG64_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
pub type Padconfig64StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG64_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
pub type Padconfig64PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG64_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
pub type Padconfig64PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG64_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
pub type Padconfig64PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG64_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
pub type Padconfig64PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG64_RXACTIVE` reader - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
pub type Padconfig64RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG64_RXACTIVE` writer - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
pub type Padconfig64RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG64_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig64DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG64_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig64DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG64_TX_DIS` reader - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
pub type Padconfig64TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG64_TX_DIS` writer - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
pub type Padconfig64TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG64_LOCK` reader - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
pub type Padconfig64LockR = crate::BitReader;
#[doc = "Field `PADCONFIG64_LOCK` writer - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
pub type Padconfig64LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
    #[inline(always)]
    pub fn padconfig64_muxmode(&self) -> Padconfig64MuxmodeR {
        Padconfig64MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig64_debounce_sel(&self) -> Padconfig64DebounceSelR {
        Padconfig64DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn padconfig64_st_en(&self) -> Padconfig64StEnR {
        Padconfig64StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
    #[inline(always)]
    pub fn padconfig64_pulluden(&self) -> Padconfig64PulludenR {
        Padconfig64PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
    #[inline(always)]
    pub fn padconfig64_pulltypesel(&self) -> Padconfig64PulltypeselR {
        Padconfig64PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
    #[inline(always)]
    pub fn padconfig64_rxactive(&self) -> Padconfig64RxactiveR {
        Padconfig64RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig64_drv_str(&self) -> Padconfig64DrvStrR {
        Padconfig64DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
    #[inline(always)]
    pub fn padconfig64_tx_dis(&self) -> Padconfig64TxDisR {
        Padconfig64TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
    #[inline(always)]
    pub fn padconfig64_lock(&self) -> Padconfig64LockR {
        Padconfig64LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig64_muxmode(&mut self) -> Padconfig64MuxmodeW<Cfg0Padconfig64Spec> {
        Padconfig64MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig64_debounce_sel(&mut self) -> Padconfig64DebounceSelW<Cfg0Padconfig64Spec> {
        Padconfig64DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig64_st_en(&mut self) -> Padconfig64StEnW<Cfg0Padconfig64Spec> {
        Padconfig64StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig64_pulluden(&mut self) -> Padconfig64PulludenW<Cfg0Padconfig64Spec> {
        Padconfig64PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig64_pulltypesel(&mut self) -> Padconfig64PulltypeselW<Cfg0Padconfig64Spec> {
        Padconfig64PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig64_rxactive(&mut self) -> Padconfig64RxactiveW<Cfg0Padconfig64Spec> {
        Padconfig64RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig64_drv_str(&mut self) -> Padconfig64DrvStrW<Cfg0Padconfig64Spec> {
        Padconfig64DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig64_tx_dis(&mut self) -> Padconfig64TxDisW<Cfg0Padconfig64Spec> {
        Padconfig64TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig64_lock(&mut self) -> Padconfig64LockW<Cfg0Padconfig64Spec> {
        Padconfig64LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig64::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig64::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig64Spec;
impl crate::RegisterSpec for Cfg0Padconfig64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig64::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig64Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig64::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG64 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig64Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
