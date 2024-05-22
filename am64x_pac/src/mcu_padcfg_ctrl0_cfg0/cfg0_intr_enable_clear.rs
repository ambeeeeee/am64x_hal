#[doc = "Register `CFG0_intr_enable_clear` reader"]
pub type R = crate::R<Cfg0IntrEnableClearSpec>;
#[doc = "Register `CFG0_intr_enable_clear` writer"]
pub type W = crate::W<Cfg0IntrEnableClearSpec>;
#[doc = "Field `PROT_ERR_EN_CLR` reader - 0:0\\]
Protection violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type ProtErrEnClrR = crate::BitReader;
#[doc = "Field `PROT_ERR_EN_CLR` writer - 0:0\\]
Protection violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type ProtErrEnClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_ERR_EN_CLR` reader - 1:1\\]
Addressing violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type AddrErrEnClrR = crate::BitReader;
#[doc = "Field `ADDR_ERR_EN_CLR` writer - 1:1\\]
Addressing violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type AddrErrEnClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KICK_ERR_EN_CLR` reader - 2:2\\]
Kick access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type KickErrEnClrR = crate::BitReader;
#[doc = "Field `KICK_ERR_EN_CLR` writer - 2:2\\]
Kick access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type KickErrEnClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROXY_ERR_EN_CLR` reader - 3:3\\]
Proxy0 access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type ProxyErrEnClrR = crate::BitReader;
#[doc = "Field `PROXY_ERR_EN_CLR` writer - 3:3\\]
Proxy0 access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
pub type ProxyErrEnClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn prot_err_en_clr(&self) -> ProtErrEnClrR {
        ProtErrEnClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn addr_err_en_clr(&self) -> AddrErrEnClrR {
        AddrErrEnClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn kick_err_en_clr(&self) -> KickErrEnClrR {
        KickErrEnClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn proxy_err_en_clr(&self) -> ProxyErrEnClrR {
        ProxyErrEnClrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn prot_err_en_clr(&mut self) -> ProtErrEnClrW<Cfg0IntrEnableClearSpec> {
        ProtErrEnClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn addr_err_en_clr(&mut self) -> AddrErrEnClrW<Cfg0IntrEnableClearSpec> {
        AddrErrEnClrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn kick_err_en_clr(&mut self) -> KickErrEnClrW<Cfg0IntrEnableClearSpec> {
        KickErrEnClrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error enable clear. Write a 1 to clear the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn proxy_err_en_clr(&mut self) -> ProxyErrEnClrW<Cfg0IntrEnableClearSpec> {
        ProxyErrEnClrW::new(self, 3)
    }
}
#[doc = "CFG0_intr_enable_clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_enable_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_enable_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0IntrEnableClearSpec;
impl crate::RegisterSpec for Cfg0IntrEnableClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_intr_enable_clear::R`](R) reader structure"]
impl crate::Readable for Cfg0IntrEnableClearSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_intr_enable_clear::W`](W) writer structure"]
impl crate::Writable for Cfg0IntrEnableClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_intr_enable_clear to value 0"]
impl crate::Resettable for Cfg0IntrEnableClearSpec {
    const RESET_VALUE: u32 = 0;
}
