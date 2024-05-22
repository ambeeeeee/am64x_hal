#[doc = "Register `CFG0_PADCONFIG15_PROXY` reader"]
pub type R = crate::R<Cfg0Padconfig15ProxySpec>;
#[doc = "Register `CFG0_PADCONFIG15_PROXY` writer"]
pub type W = crate::W<Cfg0Padconfig15ProxySpec>;
#[doc = "Field `PADCONFIG15_MUXMODE_PROXY` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig15MuxmodeProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG15_MUXMODE_PROXY` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig15MuxmodeProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG15_DEBOUNCE_SEL_PROXY` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig15DebounceSelProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG15_DEBOUNCE_SEL_PROXY` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig15DebounceSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG15_ST_EN_PROXY` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig15StEnProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG15_ST_EN_PROXY` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig15StEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG15_PULLUDEN_PROXY` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig15PulludenProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG15_PULLUDEN_PROXY` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig15PulludenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG15_PULLTYPESEL_PROXY` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig15PulltypeselProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG15_PULLTYPESEL_PROXY` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig15PulltypeselProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG15_RXACTIVE_PROXY` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig15RxactiveProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG15_RXACTIVE_PROXY` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig15RxactiveProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG15_DRV_STR_PROXY` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig15DrvStrProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG15_DRV_STR_PROXY` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig15DrvStrProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG15_TX_DIS_PROXY` reader - 21:21\\]
Driver Disable"]
pub type Padconfig15TxDisProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG15_TX_DIS_PROXY` writer - 21:21\\]
Driver Disable"]
pub type Padconfig15TxDisProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG15_LOCK_PROXY` reader - 31:31\\]
Lock"]
pub type Padconfig15LockProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG15_LOCK_PROXY` writer - 31:31\\]
Lock"]
pub type Padconfig15LockProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig15_muxmode_proxy(&self) -> Padconfig15MuxmodeProxyR {
        Padconfig15MuxmodeProxyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig15_debounce_sel_proxy(&self) -> Padconfig15DebounceSelProxyR {
        Padconfig15DebounceSelProxyR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig15_st_en_proxy(&self) -> Padconfig15StEnProxyR {
        Padconfig15StEnProxyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig15_pulluden_proxy(&self) -> Padconfig15PulludenProxyR {
        Padconfig15PulludenProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig15_pulltypesel_proxy(&self) -> Padconfig15PulltypeselProxyR {
        Padconfig15PulltypeselProxyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig15_rxactive_proxy(&self) -> Padconfig15RxactiveProxyR {
        Padconfig15RxactiveProxyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig15_drv_str_proxy(&self) -> Padconfig15DrvStrProxyR {
        Padconfig15DrvStrProxyR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig15_tx_dis_proxy(&self) -> Padconfig15TxDisProxyR {
        Padconfig15TxDisProxyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig15_lock_proxy(&self) -> Padconfig15LockProxyR {
        Padconfig15LockProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_muxmode_proxy(
        &mut self,
    ) -> Padconfig15MuxmodeProxyW<Cfg0Padconfig15ProxySpec> {
        Padconfig15MuxmodeProxyW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_debounce_sel_proxy(
        &mut self,
    ) -> Padconfig15DebounceSelProxyW<Cfg0Padconfig15ProxySpec> {
        Padconfig15DebounceSelProxyW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_st_en_proxy(&mut self) -> Padconfig15StEnProxyW<Cfg0Padconfig15ProxySpec> {
        Padconfig15StEnProxyW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_pulluden_proxy(
        &mut self,
    ) -> Padconfig15PulludenProxyW<Cfg0Padconfig15ProxySpec> {
        Padconfig15PulludenProxyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_pulltypesel_proxy(
        &mut self,
    ) -> Padconfig15PulltypeselProxyW<Cfg0Padconfig15ProxySpec> {
        Padconfig15PulltypeselProxyW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_rxactive_proxy(
        &mut self,
    ) -> Padconfig15RxactiveProxyW<Cfg0Padconfig15ProxySpec> {
        Padconfig15RxactiveProxyW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_drv_str_proxy(
        &mut self,
    ) -> Padconfig15DrvStrProxyW<Cfg0Padconfig15ProxySpec> {
        Padconfig15DrvStrProxyW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_tx_dis_proxy(&mut self) -> Padconfig15TxDisProxyW<Cfg0Padconfig15ProxySpec> {
        Padconfig15TxDisProxyW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig15_lock_proxy(&mut self) -> Padconfig15LockProxyW<Cfg0Padconfig15ProxySpec> {
        Padconfig15LockProxyW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG15_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig15_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig15_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig15ProxySpec;
impl crate::RegisterSpec for Cfg0Padconfig15ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig15_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig15ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig15_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig15ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG15_PROXY to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig15ProxySpec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
