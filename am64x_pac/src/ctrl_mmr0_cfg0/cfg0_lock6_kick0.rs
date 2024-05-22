#[doc = "Register `CFG0_LOCK6_KICK0` reader"]
pub type R = crate::R<Cfg0Lock6Kick0Spec>;
#[doc = "Register `CFG0_LOCK6_KICK0` writer"]
pub type W = crate::W<Cfg0Lock6Kick0Spec>;
#[doc = "Field `LOCK6_KICK0` reader - 31:0\\]
- KICK0 component"]
pub type Lock6Kick0R = crate::FieldReader<u32>;
#[doc = "Field `LOCK6_KICK0` writer - 31:0\\]
- KICK0 component"]
pub type Lock6Kick0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
- KICK0 component"]
    #[inline(always)]
    pub fn lock6_kick0(&self) -> Lock6Kick0R {
        Lock6Kick0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
- KICK0 component"]
    #[inline(always)]
    #[must_use]
    pub fn lock6_kick0(&mut self) -> Lock6Kick0W<Cfg0Lock6Kick0Spec> {
        Lock6Kick0W::new(self, 0)
    }
}
#[doc = "CFG0_LOCK6_KICK0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_lock6_kick0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_lock6_kick0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Lock6Kick0Spec;
impl crate::RegisterSpec for Cfg0Lock6Kick0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_lock6_kick0::R`](R) reader structure"]
impl crate::Readable for Cfg0Lock6Kick0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_lock6_kick0::W`](W) writer structure"]
impl crate::Writable for Cfg0Lock6Kick0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_LOCK6_KICK0 to value 0"]
impl crate::Resettable for Cfg0Lock6Kick0Spec {
    const RESET_VALUE: u32 = 0;
}
