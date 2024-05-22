#[doc = "Register `CFG0_PADCONFIG2_PROXY` reader"]
pub type R = crate::R<Cfg0Padconfig2ProxySpec>;
#[doc = "Register `CFG0_PADCONFIG2_PROXY` writer"]
pub type W = crate::W<Cfg0Padconfig2ProxySpec>;
#[doc = "Field `PADCONFIG2_MUXMODE_PROXY` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig2MuxmodeProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG2_MUXMODE_PROXY` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig2MuxmodeProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG2_DEBOUNCE_SEL_PROXY` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig2DebounceSelProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG2_DEBOUNCE_SEL_PROXY` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig2DebounceSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG2_ST_EN_PROXY` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig2StEnProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG2_ST_EN_PROXY` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig2StEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG2_PULLUDEN_PROXY` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig2PulludenProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG2_PULLUDEN_PROXY` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig2PulludenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG2_PULLTYPESEL_PROXY` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig2PulltypeselProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG2_PULLTYPESEL_PROXY` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig2PulltypeselProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG2_RXACTIVE_PROXY` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig2RxactiveProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG2_RXACTIVE_PROXY` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig2RxactiveProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG2_DRV_STR_PROXY` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig2DrvStrProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG2_DRV_STR_PROXY` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig2DrvStrProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG2_TX_DIS_PROXY` reader - 21:21\\]
Driver Disable"]
pub type Padconfig2TxDisProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG2_TX_DIS_PROXY` writer - 21:21\\]
Driver Disable"]
pub type Padconfig2TxDisProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG2_LOCK_PROXY` reader - 31:31\\]
Lock"]
pub type Padconfig2LockProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG2_LOCK_PROXY` writer - 31:31\\]
Lock"]
pub type Padconfig2LockProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig2_muxmode_proxy(&self) -> Padconfig2MuxmodeProxyR {
        Padconfig2MuxmodeProxyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig2_debounce_sel_proxy(&self) -> Padconfig2DebounceSelProxyR {
        Padconfig2DebounceSelProxyR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig2_st_en_proxy(&self) -> Padconfig2StEnProxyR {
        Padconfig2StEnProxyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig2_pulluden_proxy(&self) -> Padconfig2PulludenProxyR {
        Padconfig2PulludenProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig2_pulltypesel_proxy(&self) -> Padconfig2PulltypeselProxyR {
        Padconfig2PulltypeselProxyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig2_rxactive_proxy(&self) -> Padconfig2RxactiveProxyR {
        Padconfig2RxactiveProxyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig2_drv_str_proxy(&self) -> Padconfig2DrvStrProxyR {
        Padconfig2DrvStrProxyR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig2_tx_dis_proxy(&self) -> Padconfig2TxDisProxyR {
        Padconfig2TxDisProxyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig2_lock_proxy(&self) -> Padconfig2LockProxyR {
        Padconfig2LockProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig2_muxmode_proxy(&mut self) -> Padconfig2MuxmodeProxyW<Cfg0Padconfig2ProxySpec> {
        Padconfig2MuxmodeProxyW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig2_debounce_sel_proxy(
        &mut self,
    ) -> Padconfig2DebounceSelProxyW<Cfg0Padconfig2ProxySpec> {
        Padconfig2DebounceSelProxyW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig2_st_en_proxy(&mut self) -> Padconfig2StEnProxyW<Cfg0Padconfig2ProxySpec> {
        Padconfig2StEnProxyW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig2_pulluden_proxy(
        &mut self,
    ) -> Padconfig2PulludenProxyW<Cfg0Padconfig2ProxySpec> {
        Padconfig2PulludenProxyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig2_pulltypesel_proxy(
        &mut self,
    ) -> Padconfig2PulltypeselProxyW<Cfg0Padconfig2ProxySpec> {
        Padconfig2PulltypeselProxyW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig2_rxactive_proxy(
        &mut self,
    ) -> Padconfig2RxactiveProxyW<Cfg0Padconfig2ProxySpec> {
        Padconfig2RxactiveProxyW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig2_drv_str_proxy(&mut self) -> Padconfig2DrvStrProxyW<Cfg0Padconfig2ProxySpec> {
        Padconfig2DrvStrProxyW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig2_tx_dis_proxy(&mut self) -> Padconfig2TxDisProxyW<Cfg0Padconfig2ProxySpec> {
        Padconfig2TxDisProxyW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig2_lock_proxy(&mut self) -> Padconfig2LockProxyW<Cfg0Padconfig2ProxySpec> {
        Padconfig2LockProxyW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG2_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig2_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig2_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig2ProxySpec;
impl crate::RegisterSpec for Cfg0Padconfig2ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig2_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig2ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig2_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig2ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG2_PROXY to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig2ProxySpec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
