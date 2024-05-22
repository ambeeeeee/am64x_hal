#[doc = "Register `CFG0_PADCONFIG5_PROXY` reader"]
pub type R = crate::R<Cfg0Padconfig5ProxySpec>;
#[doc = "Register `CFG0_PADCONFIG5_PROXY` writer"]
pub type W = crate::W<Cfg0Padconfig5ProxySpec>;
#[doc = "Field `PADCONFIG5_MUXMODE_PROXY` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig5MuxmodeProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG5_MUXMODE_PROXY` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig5MuxmodeProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG5_DEBOUNCE_SEL_PROXY` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig5DebounceSelProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG5_DEBOUNCE_SEL_PROXY` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig5DebounceSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG5_ST_EN_PROXY` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig5StEnProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG5_ST_EN_PROXY` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig5StEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG5_PULLUDEN_PROXY` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig5PulludenProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG5_PULLUDEN_PROXY` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig5PulludenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG5_PULLTYPESEL_PROXY` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig5PulltypeselProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG5_PULLTYPESEL_PROXY` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig5PulltypeselProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG5_RXACTIVE_PROXY` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig5RxactiveProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG5_RXACTIVE_PROXY` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig5RxactiveProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG5_DRV_STR_PROXY` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig5DrvStrProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG5_DRV_STR_PROXY` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig5DrvStrProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG5_TX_DIS_PROXY` reader - 21:21\\]
Driver Disable"]
pub type Padconfig5TxDisProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG5_TX_DIS_PROXY` writer - 21:21\\]
Driver Disable"]
pub type Padconfig5TxDisProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG5_LOCK_PROXY` reader - 31:31\\]
Lock"]
pub type Padconfig5LockProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG5_LOCK_PROXY` writer - 31:31\\]
Lock"]
pub type Padconfig5LockProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig5_muxmode_proxy(&self) -> Padconfig5MuxmodeProxyR {
        Padconfig5MuxmodeProxyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig5_debounce_sel_proxy(&self) -> Padconfig5DebounceSelProxyR {
        Padconfig5DebounceSelProxyR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig5_st_en_proxy(&self) -> Padconfig5StEnProxyR {
        Padconfig5StEnProxyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig5_pulluden_proxy(&self) -> Padconfig5PulludenProxyR {
        Padconfig5PulludenProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig5_pulltypesel_proxy(&self) -> Padconfig5PulltypeselProxyR {
        Padconfig5PulltypeselProxyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig5_rxactive_proxy(&self) -> Padconfig5RxactiveProxyR {
        Padconfig5RxactiveProxyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig5_drv_str_proxy(&self) -> Padconfig5DrvStrProxyR {
        Padconfig5DrvStrProxyR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig5_tx_dis_proxy(&self) -> Padconfig5TxDisProxyR {
        Padconfig5TxDisProxyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig5_lock_proxy(&self) -> Padconfig5LockProxyR {
        Padconfig5LockProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_muxmode_proxy(&mut self) -> Padconfig5MuxmodeProxyW<Cfg0Padconfig5ProxySpec> {
        Padconfig5MuxmodeProxyW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_debounce_sel_proxy(
        &mut self,
    ) -> Padconfig5DebounceSelProxyW<Cfg0Padconfig5ProxySpec> {
        Padconfig5DebounceSelProxyW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_st_en_proxy(&mut self) -> Padconfig5StEnProxyW<Cfg0Padconfig5ProxySpec> {
        Padconfig5StEnProxyW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_pulluden_proxy(
        &mut self,
    ) -> Padconfig5PulludenProxyW<Cfg0Padconfig5ProxySpec> {
        Padconfig5PulludenProxyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_pulltypesel_proxy(
        &mut self,
    ) -> Padconfig5PulltypeselProxyW<Cfg0Padconfig5ProxySpec> {
        Padconfig5PulltypeselProxyW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_rxactive_proxy(
        &mut self,
    ) -> Padconfig5RxactiveProxyW<Cfg0Padconfig5ProxySpec> {
        Padconfig5RxactiveProxyW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_drv_str_proxy(&mut self) -> Padconfig5DrvStrProxyW<Cfg0Padconfig5ProxySpec> {
        Padconfig5DrvStrProxyW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_tx_dis_proxy(&mut self) -> Padconfig5TxDisProxyW<Cfg0Padconfig5ProxySpec> {
        Padconfig5TxDisProxyW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig5_lock_proxy(&mut self) -> Padconfig5LockProxyW<Cfg0Padconfig5ProxySpec> {
        Padconfig5LockProxyW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG5_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig5_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig5_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig5ProxySpec;
impl crate::RegisterSpec for Cfg0Padconfig5ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig5_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig5ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig5_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig5ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG5_PROXY to value 0x0021_4007"]
impl crate::Resettable for Cfg0Padconfig5ProxySpec {
    const RESET_VALUE: u32 = 0x0021_4007;
}
