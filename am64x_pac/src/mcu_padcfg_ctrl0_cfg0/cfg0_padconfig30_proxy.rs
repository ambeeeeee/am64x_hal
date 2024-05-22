#[doc = "Register `CFG0_PADCONFIG30_PROXY` reader"]
pub type R = crate::R<Cfg0Padconfig30ProxySpec>;
#[doc = "Register `CFG0_PADCONFIG30_PROXY` writer"]
pub type W = crate::W<Cfg0Padconfig30ProxySpec>;
#[doc = "Field `PADCONFIG30_MUXMODE_PROXY` reader - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig30MuxmodeProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG30_MUXMODE_PROXY` writer - 3:0\\]
Pad functional signal mux selection"]
pub type Padconfig30MuxmodeProxyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PADCONFIG30_DEBOUNCE_SEL_PROXY` reader - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig30DebounceSelProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG30_DEBOUNCE_SEL_PROXY` writer - 13:11\\]
Selects the debouce period for the pad."]
pub type Padconfig30DebounceSelProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PADCONFIG30_ST_EN_PROXY` reader - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig30StEnProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG30_ST_EN_PROXY` writer - 14:14\\]
Receiver Schmitt Trigger enable"]
pub type Padconfig30StEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG30_PULLUDEN_PROXY` reader - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig30PulludenProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG30_PULLUDEN_PROXY` writer - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
pub type Padconfig30PulludenProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG30_PULLTYPESEL_PROXY` reader - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig30PulltypeselProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG30_PULLTYPESEL_PROXY` writer - 17:17\\]
Pad Pullup / Pulldown type selection"]
pub type Padconfig30PulltypeselProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG30_RXACTIVE_PROXY` reader - 18:18\\]
Input enable for the Pad"]
pub type Padconfig30RxactiveProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG30_RXACTIVE_PROXY` writer - 18:18\\]
Input enable for the Pad"]
pub type Padconfig30RxactiveProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG30_DRV_STR_PROXY` reader - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig30DrvStrProxyR = crate::FieldReader;
#[doc = "Field `PADCONFIG30_DRV_STR_PROXY` writer - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
pub type Padconfig30DrvStrProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PADCONFIG30_TX_DIS_PROXY` reader - 21:21\\]
Driver Disable"]
pub type Padconfig30TxDisProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG30_TX_DIS_PROXY` writer - 21:21\\]
Driver Disable"]
pub type Padconfig30TxDisProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCONFIG30_LOCK_PROXY` reader - 31:31\\]
Lock"]
pub type Padconfig30LockProxyR = crate::BitReader;
#[doc = "Field `PADCONFIG30_LOCK_PROXY` writer - 31:31\\]
Lock"]
pub type Padconfig30LockProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    pub fn padconfig30_muxmode_proxy(&self) -> Padconfig30MuxmodeProxyR {
        Padconfig30MuxmodeProxyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    pub fn padconfig30_debounce_sel_proxy(&self) -> Padconfig30DebounceSelProxyR {
        Padconfig30DebounceSelProxyR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    pub fn padconfig30_st_en_proxy(&self) -> Padconfig30StEnProxyR {
        Padconfig30StEnProxyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    pub fn padconfig30_pulluden_proxy(&self) -> Padconfig30PulludenProxyR {
        Padconfig30PulludenProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    pub fn padconfig30_pulltypesel_proxy(&self) -> Padconfig30PulltypeselProxyR {
        Padconfig30PulltypeselProxyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    pub fn padconfig30_rxactive_proxy(&self) -> Padconfig30RxactiveProxyR {
        Padconfig30RxactiveProxyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    pub fn padconfig30_drv_str_proxy(&self) -> Padconfig30DrvStrProxyR {
        Padconfig30DrvStrProxyR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    pub fn padconfig30_tx_dis_proxy(&self) -> Padconfig30TxDisProxyR {
        Padconfig30TxDisProxyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    pub fn padconfig30_lock_proxy(&self) -> Padconfig30LockProxyR {
        Padconfig30LockProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Pad functional signal mux selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_muxmode_proxy(
        &mut self,
    ) -> Padconfig30MuxmodeProxyW<Cfg0Padconfig30ProxySpec> {
        Padconfig30MuxmodeProxyW::new(self, 0)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Selects the debouce period for the pad."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_debounce_sel_proxy(
        &mut self,
    ) -> Padconfig30DebounceSelProxyW<Cfg0Padconfig30ProxySpec> {
        Padconfig30DebounceSelProxyW::new(self, 11)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver Schmitt Trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_st_en_proxy(&mut self) -> Padconfig30StEnProxyW<Cfg0Padconfig30ProxySpec> {
        Padconfig30StEnProxyW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Pad Pullup / Pulldown enable. This is an active low signal."]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_pulluden_proxy(
        &mut self,
    ) -> Padconfig30PulludenProxyW<Cfg0Padconfig30ProxySpec> {
        Padconfig30PulludenProxyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Pad Pullup / Pulldown type selection"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_pulltypesel_proxy(
        &mut self,
    ) -> Padconfig30PulltypeselProxyW<Cfg0Padconfig30ProxySpec> {
        Padconfig30PulltypeselProxyW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Input enable for the Pad"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_rxactive_proxy(
        &mut self,
    ) -> Padconfig30RxactiveProxyW<Cfg0Padconfig30ProxySpec> {
        Padconfig30RxactiveProxyW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Drive Strength Control . Selects the drive strength value for LVCMOS pins. (Does not apply to other pin types)"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_drv_str_proxy(
        &mut self,
    ) -> Padconfig30DrvStrProxyW<Cfg0Padconfig30ProxySpec> {
        Padconfig30DrvStrProxyW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Driver Disable"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_tx_dis_proxy(&mut self) -> Padconfig30TxDisProxyW<Cfg0Padconfig30ProxySpec> {
        Padconfig30TxDisProxyW::new(self, 21)
    }
    #[doc = "Bit 31 - 31:31\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn padconfig30_lock_proxy(&mut self) -> Padconfig30LockProxyW<Cfg0Padconfig30ProxySpec> {
        Padconfig30LockProxyW::new(self, 31)
    }
}
#[doc = "CFG0_PADCONFIG30_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_padconfig30_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_padconfig30_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Padconfig30ProxySpec;
impl crate::RegisterSpec for Cfg0Padconfig30ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_padconfig30_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Padconfig30ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_padconfig30_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Padconfig30ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PADCONFIG30_PROXY to value 0x0026_4000"]
impl crate::Resettable for Cfg0Padconfig30ProxySpec {
    const RESET_VALUE: u32 = 0x0026_4000;
}
