#[doc = "Register `CFG0_PADCONFIG154_PROXY` reader"]
pub type R = crate::R<Cfg0Padconfig154ProxySpec>;
#[doc = "Register `CFG0_PADCONFIG154_PROXY` writer"]
pub type W = crate::W<Cfg0Padconfig154ProxySpec>;
#[doc = "Field `PADCONFIG154_MUXMODE_PROXY` reader - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
pub type Padconfig154MuxmodeProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG154_MUXMODE_PROXY` writer - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
pub type Padconfig154MuxmodeProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG154_DEBOUNCE_SEL_PROXY` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig154DebounceSelProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG154_DEBOUNCE_SEL_PROXY` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig154DebounceSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG154_ST_EN_PROXY` reader - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
pub type Padconfig154StEnProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG154_ST_EN_PROXY` writer - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
pub type Padconfig154StEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG154_PULLUDEN_PROXY` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
pub type Padconfig154PulludenProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG154_PULLUDEN_PROXY` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
pub type Padconfig154PulludenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG154_PULLTYPESEL_PROXY` reader - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
pub type Padconfig154PulltypeselProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG154_PULLTYPESEL_PROXY` writer - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
pub type Padconfig154PulltypeselProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG154_RXACTIVE_PROXY` reader - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
pub type Padconfig154RxactiveProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG154_RXACTIVE_PROXY` writer - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
pub type Padconfig154RxactiveProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG154_DRV_STR_PROXY` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig154DrvStrProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG154_DRV_STR_PROXY` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig154DrvStrProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG154_TX_DIS_PROXY` reader - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
pub type Padconfig154TxDisProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG154_TX_DIS_PROXY` writer - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
pub type Padconfig154TxDisProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG154_LOCK_PROXY` reader - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
pub type Padconfig154LockProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG154_LOCK_PROXY` writer - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
pub type Padconfig154LockProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
    #[inline(always)]
    pub fn padconfig154_muxmode_proxy(&self) -> Padconfig154MuxmodeProxyR {
        Padconfig154MuxmodeProxyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig154_debounce_sel_proxy(&self) -> Padconfig154DebounceSelProxyR {
        Padconfig154DebounceSelProxyR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
    #[inline(always)]
    pub fn padconfig154_st_en_proxy(&self) -> Padconfig154StEnProxyR {
        Padconfig154StEnProxyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
    #[inline(always)]
    pub fn padconfig154_pulluden_proxy(&self) -> Padconfig154PulludenProxyR {
        Padconfig154PulludenProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
    #[inline(always)]
    pub fn padconfig154_pulltypesel_proxy(&self) -> Padconfig154PulltypeselProxyR {
        Padconfig154PulltypeselProxyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
    #[inline(always)]
    pub fn padconfig154_rxactive_proxy(&self) -> Padconfig154RxactiveProxyR {
        Padconfig154RxactiveProxyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig154_drv_str_proxy(&self) -> Padconfig154DrvStrProxyR {
        Padconfig154DrvStrProxyR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
    #[inline(always)]
    pub fn padconfig154_tx_dis_proxy(&self) -> Padconfig154TxDisProxyR {
        Padconfig154TxDisProxyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
    #[inline(always)]
    pub fn padconfig154_lock_proxy(&self) -> Padconfig154LockProxyR {
        Padconfig154LockProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection Field values (others are reserved): 4'b0000 - Mux Mode 0 4'b0001 - Mux Mode 1 4'b0010 - Mux Mode 2 4'b0011 - Mux Mode 3 4'b0100 - Mux Mode 4 4'b0101 - Mux Mode 5 4'b0110 - Mux Mode 6 4'b0111 - Mux Mode 7 4'b1000 - Mux Mode 8 4'b1001 - Mux Mode 9 4'b1010 - Mux Mode 10 4'b1011 - Mux Mode 11 4'b1100 - Mux Mode 12 4'b1101 - Mux Mode 13 4'b1110 - Mux Mode 14 4'b1111 - Mux Mode 15"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig154_muxmode_proxy(
        &mut self,
    ) -> Padconfig154MuxmodeProxyW<Cfg0Padconfig154ProxySpec> {
        Padconfig154MuxmodeProxyW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig154_debounce_sel_proxy(
        &mut self,
    ) -> Padconfig154DebounceSelProxyW<Cfg0Padconfig154ProxySpec> {
        Padconfig154DebounceSelProxyW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable 0 - Schmitt trigger input disabled 1 - Schmitt trigger input enabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig154_st_en_proxy(
        &mut self,
    ) -> Padconfig154StEnProxyW<Cfg0Padconfig154ProxySpec> {
        Padconfig154StEnProxyW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal. 0 - Pullup / Pulldown enabled 1 - Pullup / Pulldown disabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig154_pulluden_proxy(
        &mut self,
    ) -> Padconfig154PulludenProxyW<Cfg0Padconfig154ProxySpec> {
        Padconfig154PulludenProxyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection 0 - Pulldown selected 1 - Pullup selected"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig154_pulltypesel_proxy(
        &mut self,
    ) -> Padconfig154PulltypeselProxyW<Cfg0Padconfig154ProxySpec> {
        Padconfig154PulltypeselProxyW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad 0 - Receiver disabled 1 - Receiver enabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig154_rxactive_proxy(
        &mut self,
    ) -> Padconfig154RxactiveProxyW<Cfg0Padconfig154ProxySpec> {
        Padconfig154RxactiveProxyW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig154_drv_str_proxy(
        &mut self,
    ) -> Padconfig154DrvStrProxyW<Cfg0Padconfig154ProxySpec> {
        Padconfig154DrvStrProxyW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable 0 - Driver is enabled 1 - Driver is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig154_tx_dis_proxy(
        &mut self,
    ) -> Padconfig154TxDisProxyW<Cfg0Padconfig154ProxySpec> {
        Padconfig154TxDisProxyW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock 0 - Padconfig register is unlocked 1 - Padconfig register is locked from further writes"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig154_lock_proxy(&mut self) -> Padconfig154LockProxyW<Cfg0Padconfig154ProxySpec> {
        Padconfig154LockProxyW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG154_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig154_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig154_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig154ProxySpec;
impl crate::RegisterSpec for Cfg0Padconfig154ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig154_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig154ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig154_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig154ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG154_PROXY to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig154ProxySpec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
