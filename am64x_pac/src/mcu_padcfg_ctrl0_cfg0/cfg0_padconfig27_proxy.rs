#[doc = "Register `CFG0_PADCONFIG27_PROXY` reader"]
pub type R = crate::R<Cfg0Padconfig27ProxySpec>;
#[doc = "Register `CFG0_PADCONFIG27_PROXY` writer"]
pub type W = crate::W<Cfg0Padconfig27ProxySpec>;
#[doc = "Field `PADCONFIG27_MUXMODE_PROXY` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig27MuxmodeProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG27_MUXMODE_PROXY` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig27MuxmodeProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG27_DEBOUNCE_SEL_PROXY` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig27DebounceSelProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG27_DEBOUNCE_SEL_PROXY` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig27DebounceSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG27_ST_EN_PROXY` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig27StEnProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG27_ST_EN_PROXY` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig27StEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG27_PULLUDEN_PROXY` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig27PulludenProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG27_PULLUDEN_PROXY` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig27PulludenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG27_PULLTYPESEL_PROXY` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig27PulltypeselProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG27_PULLTYPESEL_PROXY` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig27PulltypeselProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG27_RXACTIVE_PROXY` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig27RxactiveProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG27_RXACTIVE_PROXY` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig27RxactiveProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG27_DRV_STR_PROXY` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig27DrvStrProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG27_DRV_STR_PROXY` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig27DrvStrProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG27_TX_DIS_PROXY` reader - 21:21\\]
Driver Disable"]
pub type Padconfig27TxDisProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG27_TX_DIS_PROXY` writer - 21:21\\]
Driver Disable"]
pub type Padconfig27TxDisProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG27_LOCK_PROXY` reader - 31:31\\]
Lock"]
pub type Padconfig27LockProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG27_LOCK_PROXY` writer - 31:31\\]
Lock"]
pub type Padconfig27LockProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig27_muxmode_proxy(&self) -> Padconfig27MuxmodeProxyR {
        Padconfig27MuxmodeProxyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig27_debounce_sel_proxy(&self) -> Padconfig27DebounceSelProxyR {
        Padconfig27DebounceSelProxyR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig27_st_en_proxy(&self) -> Padconfig27StEnProxyR {
        Padconfig27StEnProxyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig27_pulluden_proxy(&self) -> Padconfig27PulludenProxyR {
        Padconfig27PulludenProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig27_pulltypesel_proxy(&self) -> Padconfig27PulltypeselProxyR {
        Padconfig27PulltypeselProxyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig27_rxactive_proxy(&self) -> Padconfig27RxactiveProxyR {
        Padconfig27RxactiveProxyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig27_drv_str_proxy(&self) -> Padconfig27DrvStrProxyR {
        Padconfig27DrvStrProxyR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig27_tx_dis_proxy(&self) -> Padconfig27TxDisProxyR {
        Padconfig27TxDisProxyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig27_lock_proxy(&self) -> Padconfig27LockProxyR {
        Padconfig27LockProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_muxmode_proxy(
        &mut self,
    ) -> Padconfig27MuxmodeProxyW<Cfg0Padconfig27ProxySpec> {
        Padconfig27MuxmodeProxyW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_debounce_sel_proxy(
        &mut self,
    ) -> Padconfig27DebounceSelProxyW<Cfg0Padconfig27ProxySpec> {
        Padconfig27DebounceSelProxyW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_st_en_proxy(&mut self) -> Padconfig27StEnProxyW<Cfg0Padconfig27ProxySpec> {
        Padconfig27StEnProxyW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_pulluden_proxy(
        &mut self,
    ) -> Padconfig27PulludenProxyW<Cfg0Padconfig27ProxySpec> {
        Padconfig27PulludenProxyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_pulltypesel_proxy(
        &mut self,
    ) -> Padconfig27PulltypeselProxyW<Cfg0Padconfig27ProxySpec> {
        Padconfig27PulltypeselProxyW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_rxactive_proxy(
        &mut self,
    ) -> Padconfig27RxactiveProxyW<Cfg0Padconfig27ProxySpec> {
        Padconfig27RxactiveProxyW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_drv_str_proxy(
        &mut self,
    ) -> Padconfig27DrvStrProxyW<Cfg0Padconfig27ProxySpec> {
        Padconfig27DrvStrProxyW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_tx_dis_proxy(&mut self) -> Padconfig27TxDisProxyW<Cfg0Padconfig27ProxySpec> {
        Padconfig27TxDisProxyW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig27_lock_proxy(&mut self) -> Padconfig27LockProxyW<Cfg0Padconfig27ProxySpec> {
        Padconfig27LockProxyW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG27_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig27_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig27_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig27ProxySpec;
impl crate::RegisterSpec for Cfg0Padconfig27ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig27_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig27ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig27_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig27ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG27_PROXY to value 0x0024_4000"]
impl crate::Resettable for Cfg0Padconfig27ProxySpec {
    const RESET_VALUE: u32 = 0x0024_4000;
}
