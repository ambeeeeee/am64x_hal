#[doc = "Register `CFG0_MCU_MTOG_CTRL_PROXY` reader"]
pub type R = crate::R<Cfg0McuMtogCtrlProxySpec>;
#[doc = "Register `CFG0_MCU_MTOG_CTRL_PROXY` writer"]
pub type W = crate::W<Cfg0McuMtogCtrlProxySpec>;
#[doc = "Field `MCU_MTOG_CTRL_TIMEOUT_VAL_PROXY` reader - 2:0\\]
Gasket Timeout Value"]
pub type McuMtogCtrlTimeoutValProxyR = crate::FieldReader;
#[doc = "Field `MCU_MTOG_CTRL_TIMEOUT_VAL_PROXY` writer - 2:0\\]
Gasket Timeout Value"]
pub type McuMtogCtrlTimeoutValProxyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCU_MTOG_CTRL_TIMEOUT_EN_PROXY` reader - 15:15\\]
Timeout Active"]
pub type McuMtogCtrlTimeoutEnProxyR = crate::BitReader;
#[doc = "Field `MCU_MTOG_CTRL_TIMEOUT_EN_PROXY` writer - 15:15\\]
Timeout Active"]
pub type McuMtogCtrlTimeoutEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_MTOG_CTRL_FORCE_TIMEOUT_PROXY` reader - 23:16\\]
Force Timout"]
pub type McuMtogCtrlForceTimeoutProxyR = crate::FieldReader;
#[doc = "Field `MCU_MTOG_CTRL_FORCE_TIMEOUT_PROXY` writer - 23:16\\]
Force Timout"]
pub type McuMtogCtrlForceTimeoutProxyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MCU_MTOG_CTRL_IDLE_STAT_PROXY` reader - 31:31\\]
Idle Status:"]
pub type McuMtogCtrlIdleStatProxyR = crate::BitReader;
#[doc = "Field `MCU_MTOG_CTRL_IDLE_STAT_PROXY` writer - 31:31\\]
Idle Status:"]
pub type McuMtogCtrlIdleStatProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Gasket Timeout Value"]
    #[inline(always)]
    pub fn mcu_mtog_ctrl_timeout_val_proxy(&self) -> McuMtogCtrlTimeoutValProxyR {
        McuMtogCtrlTimeoutValProxyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Timeout Active"]
    #[inline(always)]
    pub fn mcu_mtog_ctrl_timeout_en_proxy(&self) -> McuMtogCtrlTimeoutEnProxyR {
        McuMtogCtrlTimeoutEnProxyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Force Timout"]
    #[inline(always)]
    pub fn mcu_mtog_ctrl_force_timeout_proxy(&self) -> McuMtogCtrlForceTimeoutProxyR {
        McuMtogCtrlForceTimeoutProxyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Idle Status:"]
    #[inline(always)]
    pub fn mcu_mtog_ctrl_idle_stat_proxy(&self) -> McuMtogCtrlIdleStatProxyR {
        McuMtogCtrlIdleStatProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Gasket Timeout Value"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_mtog_ctrl_timeout_val_proxy(
        &mut self,
    ) -> McuMtogCtrlTimeoutValProxyW<Cfg0McuMtogCtrlProxySpec> {
        McuMtogCtrlTimeoutValProxyW::new(self, 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Timeout Active"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_mtog_ctrl_timeout_en_proxy(
        &mut self,
    ) -> McuMtogCtrlTimeoutEnProxyW<Cfg0McuMtogCtrlProxySpec> {
        McuMtogCtrlTimeoutEnProxyW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Force Timout"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_mtog_ctrl_force_timeout_proxy(
        &mut self,
    ) -> McuMtogCtrlForceTimeoutProxyW<Cfg0McuMtogCtrlProxySpec> {
        McuMtogCtrlForceTimeoutProxyW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Idle Status:"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_mtog_ctrl_idle_stat_proxy(
        &mut self,
    ) -> McuMtogCtrlIdleStatProxyW<Cfg0McuMtogCtrlProxySpec> {
        McuMtogCtrlIdleStatProxyW::new(self, 31)
    }
}
#[doc = "CFG0_MCU_MTOG_CTRL_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_mtog_ctrl_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_mtog_ctrl_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuMtogCtrlProxySpec;
impl crate::RegisterSpec for Cfg0McuMtogCtrlProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_mtog_ctrl_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0McuMtogCtrlProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_mtog_ctrl_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0McuMtogCtrlProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_MTOG_CTRL_PROXY to value 0"]
impl crate::Resettable for Cfg0McuMtogCtrlProxySpec {
    const RESET_VALUE: u32 = 0;
}
