#[doc = "Register `CFG0_PADCONFIG62` reader"]
pub type R = crate::R<Cfg0Padconfig62Spec>;
#[doc = "Register `CFG0_PADCONFIG62` writer"]
pub type W = crate::W<Cfg0Padconfig62Spec>;
#[doc = "Field `PADCONFIG62_MUXMODE` reader - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
pub type Padconfig62MuxmodeR = crate::FieldReader;
#[doc = "Field `PADCONFIG62_MUXMODE` writer - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
pub type Padconfig62MuxmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG62_DEBOUNCE_SEL` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig62DebounceSelR = crate::FieldReader;
#[doc = "Field `PADCONFIG62_DEBOUNCE_SEL` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig62DebounceSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG62_ST_EN` reader - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
pub type Padconfig62StEnR = crate::BitReader;
#[doc = "Field `PADCONFIG62_ST_EN` writer - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
pub type Padconfig62StEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG62_PULLUDEN` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
pub type Padconfig62PulludenR = crate::BitReader;
#[doc = "Field `PADCONFIG62_PULLUDEN` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
pub type Padconfig62PulludenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG62_PULLTYPESEL` reader - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
pub type Padconfig62PulltypeselR = crate::BitReader;
#[doc = "Field `PADCONFIG62_PULLTYPESEL` writer - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
pub type Padconfig62PulltypeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG62_RXACTIVE` reader - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
pub type Padconfig62RxactiveR = crate::BitReader;
#[doc = "Field `PADCONFIG62_RXACTIVE` writer - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
pub type Padconfig62RxactiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG62_DRV_STR` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig62DrvStrR = crate::FieldReader;
#[doc = "Field `PADCONFIG62_DRV_STR` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig62DrvStrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG62_TX_DIS` reader - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
pub type Padconfig62TxDisR = crate::BitReader;
#[doc = "Field `PADCONFIG62_TX_DIS` writer - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
pub type Padconfig62TxDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG62_LOCK` reader - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
pub type Padconfig62LockR = crate::BitReader;
#[doc = "Field `PADCONFIG62_LOCK` writer - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
pub type Padconfig62LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
    #[inline(always)]
    pub fn padconfig62_muxmode(&self) -> Padconfig62MuxmodeR {
        Padconfig62MuxmodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig62_debounce_sel(&self) -> Padconfig62DebounceSelR {
        Padconfig62DebounceSelR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn padconfig62_st_en(&self) -> Padconfig62StEnR {
        Padconfig62StEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
    #[inline(always)]
    pub fn padconfig62_pulluden(&self) -> Padconfig62PulludenR {
        Padconfig62PulludenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
    #[inline(always)]
    pub fn padconfig62_pulltypesel(&self) -> Padconfig62PulltypeselR {
        Padconfig62PulltypeselR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
    #[inline(always)]
    pub fn padconfig62_rxactive(&self) -> Padconfig62RxactiveR {
        Padconfig62RxactiveR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig62_drv_str(&self) -> Padconfig62DrvStrR {
        Padconfig62DrvStrR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
    #[inline(always)]
    pub fn padconfig62_tx_dis(&self) -> Padconfig62TxDisR {
        Padconfig62TxDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
    #[inline(always)]
    pub fn padconfig62_lock(&self) -> Padconfig62LockR {
        Padconfig62LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig62_muxmode(&mut self) -> Padconfig62MuxmodeW<Cfg0Padconfig62Spec> {
        Padconfig62MuxmodeW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig62_debounce_sel(&mut self) -> Padconfig62DebounceSelW<Cfg0Padconfig62Spec> {
        Padconfig62DebounceSelW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig62_st_en(&mut self) -> Padconfig62StEnW<Cfg0Padconfig62Spec> {
        Padconfig62StEnW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig62_pulluden(&mut self) -> Padconfig62PulludenW<Cfg0Padconfig62Spec> {
        Padconfig62PulludenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig62_pulltypesel(&mut self) -> Padconfig62PulltypeselW<Cfg0Padconfig62Spec> {
        Padconfig62PulltypeselW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig62_rxactive(&mut self) -> Padconfig62RxactiveW<Cfg0Padconfig62Spec> {
        Padconfig62RxactiveW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig62_drv_str(&mut self) -> Padconfig62DrvStrW<Cfg0Padconfig62Spec> {
        Padconfig62DrvStrW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig62_tx_dis(&mut self) -> Padconfig62TxDisW<Cfg0Padconfig62Spec> {
        Padconfig62TxDisW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig62_lock(&mut self) -> Padconfig62LockW<Cfg0Padconfig62Spec> {
        Padconfig62LockW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG62\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig62::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig62::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig62Spec;
impl crate::RegisterSpec for Cfg0Padconfig62Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig62::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig62Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig62::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig62Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG62 to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig62Spec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
