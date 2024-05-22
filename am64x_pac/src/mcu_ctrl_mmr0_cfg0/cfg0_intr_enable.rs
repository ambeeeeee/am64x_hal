#[doc = "Register `CFG0_intr_enable` reader"]
pub type R = crate::R<Cfg0IntrEnableSpec>;
#[doc = "Register `CFG0_intr_enable` writer"]
pub type W = crate::W<Cfg0IntrEnableSpec>;
#[doc = "Field `PROT_ERR_EN` reader - 0:0\\]
Protection violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type ProtErrEnR = crate::BitReader;
#[doc = "Field `PROT_ERR_EN` writer - 0:0\\]
Protection violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type ProtErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_ERR_EN` reader - 1:1\\]
Addressing violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type AddrErrEnR = crate::BitReader;
#[doc = "Field `ADDR_ERR_EN` writer - 1:1\\]
Addressing violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type AddrErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KICK_ERR_EN` reader - 2:2\\]
Kick access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type KickErrEnR = crate::BitReader;
#[doc = "Field `KICK_ERR_EN` writer - 2:2\\]
Kick access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type KickErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROXY_ERR_EN` reader - 3:3\\]
Proxy0 access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type ProxyErrEnR = crate::BitReader;
#[doc = "Field `PROXY_ERR_EN` writer - 3:3\\]
Proxy0 access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
pub type ProxyErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn prot_err_en(&self) -> ProtErrEnR {
        ProtErrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn addr_err_en(&self) -> AddrErrEnR {
        AddrErrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn kick_err_en(&self) -> KickErrEnR {
        KickErrEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn proxy_err_en(&self) -> ProxyErrEnR {
        ProxyErrEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Protection violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn prot_err_en(&mut self) -> ProtErrEnW<Cfg0IntrEnableSpec> {
        ProtErrEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Addressing violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn addr_err_en(&mut self) -> AddrErrEnW<Cfg0IntrEnableSpec> {
        AddrErrEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Kick access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn kick_err_en(&mut self) -> KickErrEnW<Cfg0IntrEnableSpec> {
        KickErrEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Proxy0 access violation error enable. Write a 1 to set the enable. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn proxy_err_en(&mut self) -> ProxyErrEnW<Cfg0IntrEnableSpec> {
        ProxyErrEnW::new(self, 3)
    }
}
#[doc = "CFG0_intr_enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_intr_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_intr_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0IntrEnableSpec;
impl crate::RegisterSpec for Cfg0IntrEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_intr_enable::R`](R) reader structure"]
impl crate::Readable for Cfg0IntrEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_intr_enable::W`](W) writer structure"]
impl crate::Writable for Cfg0IntrEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_intr_enable to value 0"]
impl crate::Resettable for Cfg0IntrEnableSpec {
    const RESET_VALUE: u32 = 0;
}
