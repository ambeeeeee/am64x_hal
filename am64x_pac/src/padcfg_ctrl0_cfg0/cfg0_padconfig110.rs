#[doc = "Register `CFG0_PADCONFIG110` reader"]
pub type R = crate::R<Cfg0Padconfig110Spec>;
#[doc = "Register `CFG0_PADCONFIG110` writer"]
pub type W = crate::W<Cfg0Padconfig110Spec>;
#[doc = "Field `PADCONFIG110_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
pub type Padconfig110MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG110_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
pub type Padconfig110MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG110_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig110DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG110_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig110DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG110_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
pub type Padconfig110StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG110_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
pub type Padconfig110StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG110_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
pub type Padconfig110PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG110_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
pub type Padconfig110PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG110_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
pub type Padconfig110PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG110_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
pub type Padconfig110PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG110_RXACTIVE` reader - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
pub type Padconfig110RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG110_RXACTIVE` writer - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
pub type Padconfig110RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG110_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig110DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG110_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig110DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG110_TX_DIS` reader - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
pub type Padconfig110TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG110_TX_DIS` writer - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
pub type Padconfig110TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG110_LOCK` reader - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
pub type Padconfig110LockR = crate::BitReader;
#[doc = "Field `PADCONFIG110_LOCK` writer - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
pub type Padconfig110LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
    #[inline(always)]
    pub fn padconfig110_muxmode(&self) -> Padconfig110MuxmodeR {
        Padconfig110MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig110_debounce_sel(&self) -> Padconfig110DebounceSelR {
        Padconfig110DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn padconfig110_st_en(&self) -> Padconfig110StEnR {
        Padconfig110StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
    #[inline(always)]
    pub fn padconfig110_pulluden(&self) -> Padconfig110PulludenR {
        Padconfig110PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
    #[inline(always)]
    pub fn padconfig110_pulltypesel(&self) -> Padconfig110PulltypeselR {
        Padconfig110PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
    #[inline(always)]
    pub fn padconfig110_rxactive(&self) -> Padconfig110RxactiveR {
        Padconfig110RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig110_drv_str(&self) -> Padconfig110DrvStrR {
        Padconfig110DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
    #[inline(always)]
    pub fn padconfig110_tx_dis(&self) -> Padconfig110TxDisR {
        Padconfig110TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
    #[inline(always)]
    pub fn padconfig110_lock(&self) -> Padconfig110LockR {
        Padconfig110LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig110_muxmode(&mut self) -> Padconfig110MuxmodeW<Cfg0Padconfig110Spec> {
        Padconfig110MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig110_debounce_sel(&mut self) -> Padconfig110DebounceSelW<Cfg0Padconfig110Spec> {
        Padconfig110DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig110_st_en(&mut self) -> Padconfig110StEnW<Cfg0Padconfig110Spec> {
        Padconfig110StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig110_pulluden(&mut self) -> Padconfig110PulludenW<Cfg0Padconfig110Spec> {
        Padconfig110PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig110_pulltypesel(&mut self) -> Padconfig110PulltypeselW<Cfg0Padconfig110Spec> {
        Padconfig110PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig110_rxactive(&mut self) -> Padconfig110RxactiveW<Cfg0Padconfig110Spec> {
        Padconfig110RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig110_drv_str(&mut self) -> Padconfig110DrvStrW<Cfg0Padconfig110Spec> {
        Padconfig110DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig110_tx_dis(&mut self) -> Padconfig110TxDisW<Cfg0Padconfig110Spec> {
        Padconfig110TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig110_lock(&mut self) -> Padconfig110LockW<Cfg0Padconfig110Spec> {
        Padconfig110LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG110\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig110::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig110::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig110Spec;
impl crate::RegisterSpec for Cfg0Padconfig110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig110::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig110Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig110::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig110Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG110 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig110Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
