#[doc = "Register `CFG0_intr_raw_status_PROXY` reader"]
pub type R = crate::R<Cfg0IntrRawStatusProxySpec>;
#[doc = "Register `CFG0_intr_raw_status_PROXY` writer"]
pub type W = crate::W<Cfg0IntrRawStatusProxySpec>;
#[doc = "Field `PROT_ERR_PROXY` reader - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type ProtErrProxyR = crate::BitReader;
#[doc = "Field `PROT_ERR_PROXY` writer - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type ProtErrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_ERR_PROXY` reader - 1:1\\]
Addressing violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type AddrErrProxyR = crate::BitReader;
#[doc = "Field `ADDR_ERR_PROXY` writer - 1:1\\]
Addressing violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type AddrErrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KICK_ERR_PROXY` reader - 2:2\\]
Kick access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type KickErrProxyR = crate::BitReader;
#[doc = "Field `KICK_ERR_PROXY` writer - 2:2\\]
Kick access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type KickErrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROXY_ERR_PROXY` reader - 3:3\\]
Proxy0 access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type ProxyErrProxyR = crate::BitReader;
#[doc = "Field `PROXY_ERR_PROXY` writer - 3:3\\]
Proxy0 access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
pub type ProxyErrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn prot_err_proxy(&self) -> ProtErrProxyR {
        ProtErrProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn addr_err_proxy(&self) -> AddrErrProxyR {
        AddrErrProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn kick_err_proxy(&self) -> KickErrProxyR {
        KickErrProxyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn proxy_err_proxy(&self) -> ProxyErrProxyR {
        ProxyErrProxyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn prot_err_proxy(&mut self) -> ProtErrProxyW<Cfg0IntrRawStatusProxySpec> {
        ProtErrProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn addr_err_proxy(&mut self) -> AddrErrProxyW<Cfg0IntrRawStatusProxySpec> {
        AddrErrProxyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn kick_err_proxy(&mut self) -> KickErrProxyW<Cfg0IntrRawStatusProxySpec> {
        KickErrProxyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error. Raw status is read. Write a 1 to set the status. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn proxy_err_proxy(&mut self) -> ProxyErrProxyW<Cfg0IntrRawStatusProxySpec> {
        ProxyErrProxyW::new(self, 3)
    }
}
#[doc = "CFG0_intr_raw_status_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_raw_status_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_raw_status_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0IntrRawStatusProxySpec;
impl crate::RegisterSpec for Cfg0IntrRawStatusProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_intr_raw_status_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0IntrRawStatusProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_intr_raw_status_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0IntrRawStatusProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_intr_raw_status_PROXY to value 0"]
impl crate::Resettable for Cfg0IntrRawStatusProxySpec {
    const RESET_VALUE: u32 = 0;
}
