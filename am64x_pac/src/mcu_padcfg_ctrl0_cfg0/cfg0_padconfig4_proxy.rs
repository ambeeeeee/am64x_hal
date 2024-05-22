#[doc = "Register `CFG0_PADCONFIG4_PROXY` reader"]
pub type R = crate::R<Cfg0Padconfig4ProxySpec>;
#[doc = "Register `CFG0_PADCONFIG4_PROXY` writer"]
pub type W = crate::W<Cfg0Padconfig4ProxySpec>;
#[doc = "Field `PADCONFIG4_MUXMODE_PROXY` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig4MuxmodeProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG4_MUXMODE_PROXY` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig4MuxmodeProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG4_DEBOUNCE_SEL_PROXY` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig4DebounceSelProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG4_DEBOUNCE_SEL_PROXY` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig4DebounceSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG4_ST_EN_PROXY` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig4StEnProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG4_ST_EN_PROXY` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig4StEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG4_PULLUDEN_PROXY` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig4PulludenProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG4_PULLUDEN_PROXY` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig4PulludenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG4_PULLTYPESEL_PROXY` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig4PulltypeselProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG4_PULLTYPESEL_PROXY` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig4PulltypeselProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG4_RXACTIVE_PROXY` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig4RxactiveProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG4_RXACTIVE_PROXY` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig4RxactiveProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG4_DRV_STR_PROXY` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig4DrvStrProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG4_DRV_STR_PROXY` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig4DrvStrProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG4_TX_DIS_PROXY` reader - 21:21\\]
Driver Disable"]
pub type Padconfig4TxDisProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG4_TX_DIS_PROXY` writer - 21:21\\]
Driver Disable"]
pub type Padconfig4TxDisProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG4_LOCK_PROXY` reader - 31:31\\]
Lock"]
pub type Padconfig4LockProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG4_LOCK_PROXY` writer - 31:31\\]
Lock"]
pub type Padconfig4LockProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig4_muxmode_proxy(&self) -> Padconfig4MuxmodeProxyR {
        Padconfig4MuxmodeProxyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig4_debounce_sel_proxy(&self) -> Padconfig4DebounceSelProxyR {
        Padconfig4DebounceSelProxyR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig4_st_en_proxy(&self) -> Padconfig4StEnProxyR {
        Padconfig4StEnProxyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig4_pulluden_proxy(&self) -> Padconfig4PulludenProxyR {
        Padconfig4PulludenProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig4_pulltypesel_proxy(&self) -> Padconfig4PulltypeselProxyR {
        Padconfig4PulltypeselProxyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig4_rxactive_proxy(&self) -> Padconfig4RxactiveProxyR {
        Padconfig4RxactiveProxyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig4_drv_str_proxy(&self) -> Padconfig4DrvStrProxyR {
        Padconfig4DrvStrProxyR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig4_tx_dis_proxy(&self) -> Padconfig4TxDisProxyR {
        Padconfig4TxDisProxyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig4_lock_proxy(&self) -> Padconfig4LockProxyR {
        Padconfig4LockProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_muxmode_proxy(&mut self) -> Padconfig4MuxmodeProxyW<Cfg0Padconfig4ProxySpec> {
        Padconfig4MuxmodeProxyW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_debounce_sel_proxy(
        &mut self,
    ) -> Padconfig4DebounceSelProxyW<Cfg0Padconfig4ProxySpec> {
        Padconfig4DebounceSelProxyW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_st_en_proxy(&mut self) -> Padconfig4StEnProxyW<Cfg0Padconfig4ProxySpec> {
        Padconfig4StEnProxyW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_pulluden_proxy(
        &mut self,
    ) -> Padconfig4PulludenProxyW<Cfg0Padconfig4ProxySpec> {
        Padconfig4PulludenProxyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_pulltypesel_proxy(
        &mut self,
    ) -> Padconfig4PulltypeselProxyW<Cfg0Padconfig4ProxySpec> {
        Padconfig4PulltypeselProxyW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_rxactive_proxy(
        &mut self,
    ) -> Padconfig4RxactiveProxyW<Cfg0Padconfig4ProxySpec> {
        Padconfig4RxactiveProxyW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_drv_str_proxy(&mut self) -> Padconfig4DrvStrProxyW<Cfg0Padconfig4ProxySpec> {
        Padconfig4DrvStrProxyW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_tx_dis_proxy(&mut self) -> Padconfig4TxDisProxyW<Cfg0Padconfig4ProxySpec> {
        Padconfig4TxDisProxyW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig4_lock_proxy(&mut self) -> Padconfig4LockProxyW<Cfg0Padconfig4ProxySpec> {
        Padconfig4LockProxyW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG4_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig4_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig4_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig4ProxySpec;
impl crate::RegisterSpec for Cfg0Padconfig4ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig4_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig4ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig4_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig4ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG4_PROXY to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig4ProxySpec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
