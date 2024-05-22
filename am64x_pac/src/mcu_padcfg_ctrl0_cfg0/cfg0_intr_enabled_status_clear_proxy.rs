#[doc = "Register `CFG0_intr_enabled_status_clear_PROXY` reader"]
pub type R = crate::R<Cfg0IntrEnabledStatusClearProxySpec>;
#[doc = "Register `CFG0_intr_enabled_status_clear_PROXY` writer"]
pub type W = crate::W<Cfg0IntrEnabledStatusClearProxySpec>;
#[doc = "Field `ENABLED_PROT_ERR_PROXY` reader - 0:0\\]
Protection violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledProtErrProxyR = crate::BitReader;
#[doc = "Field `ENABLED_PROT_ERR_PROXY` writer - 0:0\\]
Protection violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledProtErrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_ADDR_ERR_PROXY` reader - 1:1\\]
Addressing violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledAddrErrProxyR = crate::BitReader;
#[doc = "Field `ENABLED_ADDR_ERR_PROXY` writer - 1:1\\]
Addressing violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledAddrErrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_KICK_ERR_PROXY` reader - 2:2\\]
Kick access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledKickErrProxyR = crate::BitReader;
#[doc = "Field `ENABLED_KICK_ERR_PROXY` writer - 2:2\\]
Kick access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledKickErrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_PROXY_ERR_PROXY` reader - 3:3\\]
Proxy0 access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledProxyErrProxyR = crate::BitReader;
#[doc = "Field `ENABLED_PROXY_ERR_PROXY` writer - 3:3\\]
Proxy0 access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledProxyErrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn enabled_prot_err_proxy(&self) -> EnabledProtErrProxyR {
        EnabledProtErrProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn enabled_addr_err_proxy(&self) -> EnabledAddrErrProxyR {
        EnabledAddrErrProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn enabled_kick_err_proxy(&self) -> EnabledKickErrProxyR {
        EnabledKickErrProxyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn enabled_proxy_err_proxy(&self) -> EnabledProxyErrProxyR {
        EnabledProxyErrProxyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_prot_err_proxy(
        &mut self,
    ) -> EnabledProtErrProxyW<Cfg0IntrEnabledStatusClearProxySpec> {
        EnabledProtErrProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_addr_err_proxy(
        &mut self,
    ) -> EnabledAddrErrProxyW<Cfg0IntrEnabledStatusClearProxySpec> {
        EnabledAddrErrProxyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_kick_err_proxy(
        &mut self,
    ) -> EnabledKickErrProxyW<Cfg0IntrEnabledStatusClearProxySpec> {
        EnabledKickErrProxyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_proxy_err_proxy(
        &mut self,
    ) -> EnabledProxyErrProxyW<Cfg0IntrEnabledStatusClearProxySpec> {
        EnabledProxyErrProxyW::new(self, 3)
    }
}
#[doc = "CFG0_intr_enabled_status_clear_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_enabled_status_clear_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_enabled_status_clear_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0IntrEnabledStatusClearProxySpec;
impl crate::RegisterSpec for Cfg0IntrEnabledStatusClearProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_intr_enabled_status_clear_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0IntrEnabledStatusClearProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_intr_enabled_status_clear_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0IntrEnabledStatusClearProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_intr_enabled_status_clear_PROXY to value 0"]
impl crate::Resettable for Cfg0IntrEnabledStatusClearProxySpec {
    const RESET_VALUE: u32 = 0;
}
