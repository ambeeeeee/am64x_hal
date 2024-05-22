#[doc = "Register `CFG0_intr_enable_PROXY` reader"]
pub type R = crate::R<Cfg0IntrEnableProxySpec>;
#[doc = "Register `CFG0_intr_enable_PROXY` writer"]
pub type W = crate::W<Cfg0IntrEnableProxySpec>;
#[doc = "Field `PROT_ERR_EN_PROXY` reader - 0:0\\]
Protection violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type ProtErrEnProxyR = crate::BitReader;
#[doc = "Field `PROT_ERR_EN_PROXY` writer - 0:0\\]
Protection violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type ProtErrEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_ERR_EN_PROXY` reader - 1:1\\]
Addressing violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type AddrErrEnProxyR = crate::BitReader;
#[doc = "Field `ADDR_ERR_EN_PROXY` writer - 1:1\\]
Addressing violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type AddrErrEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KICK_ERR_EN_PROXY` reader - 2:2\\]
Kick access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type KickErrEnProxyR = crate::BitReader;
#[doc = "Field `KICK_ERR_EN_PROXY` writer - 2:2\\]
Kick access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type KickErrEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROXY_ERR_EN_PROXY` reader - 3:3\\]
Proxy0 access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type ProxyErrEnProxyR = crate::BitReader;
#[doc = "Field `PROXY_ERR_EN_PROXY` writer - 3:3\\]
Proxy0 access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type ProxyErrEnProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn prot_err_en_proxy(&self) -> ProtErrEnProxyR {
        ProtErrEnProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn addr_err_en_proxy(&self) -> AddrErrEnProxyR {
        AddrErrEnProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn kick_err_en_proxy(&self) -> KickErrEnProxyR {
        KickErrEnProxyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn proxy_err_en_proxy(&self) -> ProxyErrEnProxyR {
        ProxyErrEnProxyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn prot_err_en_proxy(&mut self) -> ProtErrEnProxyW<Cfg0IntrEnableProxySpec> {
        ProtErrEnProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn addr_err_en_proxy(&mut self) -> AddrErrEnProxyW<Cfg0IntrEnableProxySpec> {
        AddrErrEnProxyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn kick_err_en_proxy(&mut self) -> KickErrEnProxyW<Cfg0IntrEnableProxySpec> {
        KickErrEnProxyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn proxy_err_en_proxy(&mut self) -> ProxyErrEnProxyW<Cfg0IntrEnableProxySpec> {
        ProxyErrEnProxyW::new(self, 3)
    }
}
#[doc = "CFG0_intr_enable_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_enable_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_enable_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0IntrEnableProxySpec;
impl crate::RegisterSpec for Cfg0IntrEnableProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_intr_enable_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0IntrEnableProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_intr_enable_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0IntrEnableProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_intr_enable_PROXY to value 0"]
impl crate::Resettable for Cfg0IntrEnableProxySpec {
    const RESET_VALUE: u32 = 0;
}
