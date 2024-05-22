#[doc = "Register `CFG0_LOCK5_KICK1_PROXY` reader"]
pub type R = crate::R<Cfg0Lock5Kick1ProxySpec>;
#[doc = "Register `CFG0_LOCK5_KICK1_PROXY` writer"]
pub type W = crate::W<Cfg0Lock5Kick1ProxySpec>;
#[doc = "Field `LOCK5_KICK1_PROXY` reader - 31:0\\]
- KICK1 component"]
pub type Lock5Kick1ProxyR = crate::FieldReader<u32>;
#[doc = "Field `LOCK5_KICK1_PROXY` writer - 31:0\\]
- KICK1 component"]
pub type Lock5Kick1ProxyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
- KICK1 component"]
    #[inline(always)]
    pub fn lock5_kick1_proxy(&self) -> Lock5Kick1ProxyR {
        Lock5Kick1ProxyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
- KICK1 component"]
    #[inline(always)]
    #[must_use]
    pub fn lock5_kick1_proxy(&mut self) -> Lock5Kick1ProxyW<Cfg0Lock5Kick1ProxySpec> {
        Lock5Kick1ProxyW::new(self, 0)
    }
}
#[doc = "CFG0_LOCK5_KICK1_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock5_kick1_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock5_kick1_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Lock5Kick1ProxySpec;
impl crate::RegisterSpec for Cfg0Lock5Kick1ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_lock5_kick1_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Lock5Kick1ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_lock5_kick1_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Lock5Kick1ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_LOCK5_KICK1_PROXY to value 0"]
impl crate::Resettable for Cfg0Lock5Kick1ProxySpec {
    const RESET_VALUE: u32 = 0;
}
