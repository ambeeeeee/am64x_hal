#[doc = "Register `CFG0_intr_enable_clear_PROXY` reader"]
pub type R = crate::R<Cfg0IntrEnableClearProxySpec>;
#[doc = "Register `CFG0_intr_enable_clear_PROXY` writer"]
pub type W = crate::W<Cfg0IntrEnableClearProxySpec>;
#[doc = "Field `PROT_ERR_EN_CLR_PROXY` reader - 0:0\\]
Protection violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type ProtErrEnClrProxyR = crate::BitReader;
#[doc = "Field `PROT_ERR_EN_CLR_PROXY` writer - 0:0\\]
Protection violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type ProtErrEnClrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_ERR_EN_CLR_PROXY` reader - 1:1\\]
Addressing violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type AddrErrEnClrProxyR = crate::BitReader;
#[doc = "Field `ADDR_ERR_EN_CLR_PROXY` writer - 1:1\\]
Addressing violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type AddrErrEnClrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KICK_ERR_EN_CLR_PROXY` reader - 2:2\\]
Kick access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type KickErrEnClrProxyR = crate::BitReader;
#[doc = "Field `KICK_ERR_EN_CLR_PROXY` writer - 2:2\\]
Kick access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type KickErrEnClrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROXY_ERR_EN_CLR_PROXY` reader - 3:3\\]
Proxy0 access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type ProxyErrEnClrProxyR = crate::BitReader;
#[doc = "Field `PROXY_ERR_EN_CLR_PROXY` writer - 3:3\\]
Proxy0 access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type ProxyErrEnClrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn prot_err_en_clr_proxy(&self) -> ProtErrEnClrProxyR {
        ProtErrEnClrProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn addr_err_en_clr_proxy(&self) -> AddrErrEnClrProxyR {
        AddrErrEnClrProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn kick_err_en_clr_proxy(&self) -> KickErrEnClrProxyR {
        KickErrEnClrProxyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn proxy_err_en_clr_proxy(&self) -> ProxyErrEnClrProxyR {
        ProxyErrEnClrProxyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn prot_err_en_clr_proxy(&mut self) -> ProtErrEnClrProxyW<Cfg0IntrEnableClearProxySpec> {
        ProtErrEnClrProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn addr_err_en_clr_proxy(&mut self) -> AddrErrEnClrProxyW<Cfg0IntrEnableClearProxySpec> {
        AddrErrEnClrProxyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn kick_err_en_clr_proxy(&mut self) -> KickErrEnClrProxyW<Cfg0IntrEnableClearProxySpec> {
        KickErrEnClrProxyW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn proxy_err_en_clr_proxy(&mut self) -> ProxyErrEnClrProxyW<Cfg0IntrEnableClearProxySpec> {
        ProxyErrEnClrProxyW::new(self, 3)
    }
}
#[doc = "CFG0_intr_enable_clear_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_enable_clear_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_enable_clear_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0IntrEnableClearProxySpec;
impl crate::RegisterSpec for Cfg0IntrEnableClearProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_intr_enable_clear_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0IntrEnableClearProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_intr_enable_clear_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0IntrEnableClearProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_intr_enable_clear_PROXY to value 0"]
impl crate::Resettable for Cfg0IntrEnableClearProxySpec {
    const RESET_VALUE: u32 = 0;
}
