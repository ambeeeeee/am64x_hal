#[doc = "Register `CFG0_intr_enabled_status_clear` reader"]
pub type R = crate::R<Cfg0IntrEnabledStatusClearSpec>;
#[doc = "Register `CFG0_intr_enabled_status_clear` writer"]
pub type W = crate::W<Cfg0IntrEnabledStatusClearSpec>;
#[doc = "Field `ENABLED_PROT_ERR` reader - 0:0\\]
Protection violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledProtErrR = crate::BitReader;
#[doc = "Field `ENABLED_PROT_ERR` writer - 0:0\\]
Protection violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledProtErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_ADDR_ERR` reader - 1:1\\]
Addressing violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledAddrErrR = crate::BitReader;
#[doc = "Field `ENABLED_ADDR_ERR` writer - 1:1\\]
Addressing violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledAddrErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_KICK_ERR` reader - 2:2\\]
Kick access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledKickErrR = crate::BitReader;
#[doc = "Field `ENABLED_KICK_ERR` writer - 2:2\\]
Kick access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledKickErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED_PROXY_ERR` reader - 3:3\\]
Proxy0 access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledProxyErrR = crate::BitReader;
#[doc = "Field `ENABLED_PROXY_ERR` writer - 3:3\\]
Proxy0 access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
pub type EnabledProxyErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn enabled_prot_err(&self) -> EnabledProtErrR {
        EnabledProtErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn enabled_addr_err(&self) -> EnabledAddrErrR {
        EnabledAddrErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn enabled_kick_err(&self) -> EnabledKickErrR {
        EnabledKickErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn enabled_proxy_err(&self) -> EnabledProxyErrR {
        EnabledProxyErrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_prot_err(&mut self) -> EnabledProtErrW<Cfg0IntrEnabledStatusClearSpec> {
        EnabledProtErrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_addr_err(&mut self) -> EnabledAddrErrW<Cfg0IntrEnabledStatusClearSpec> {
        EnabledAddrErrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_kick_err(&mut self) -> EnabledKickErrW<Cfg0IntrEnabledStatusClearSpec> {
        EnabledKickErrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error. Enabled status is read. Write a 1 to clear the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn enabled_proxy_err(&mut self) -> EnabledProxyErrW<Cfg0IntrEnabledStatusClearSpec> {
        EnabledProxyErrW::new(self, 3)
    }
}
#[doc = "CFG0_intr_enabled_status_clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_enabled_status_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_enabled_status_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0IntrEnabledStatusClearSpec;
impl crate::RegisterSpec for Cfg0IntrEnabledStatusClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_intr_enabled_status_clear::R`](R) reader structure"]
impl crate::Readable for Cfg0IntrEnabledStatusClearSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_intr_enabled_status_clear::W`](W) writer structure"]
impl crate::Writable for Cfg0IntrEnabledStatusClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_intr_enabled_status_clear to value 0"]
impl crate::Resettable for Cfg0IntrEnabledStatusClearSpec {
    const RESET_VALUE: u32 = 0;
}
