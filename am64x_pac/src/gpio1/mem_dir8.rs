#[doc = "Register `MEM_DIR8` reader"]
pub type R = crate::R<MemDir8Spec>;
#[doc = "Register `MEM_DIR8` writer"]
pub type W = crate::W<MemDir8Spec>;
#[doc = "Field `DIR8` reader - 15:0\\]
Direction of GPIO bank 8 bits, 0 = output, 1 = input"]
pub type Dir8R = crate::FieldReader<u16>;
#[doc = "Field `DIR8` writer - 15:0\\]
Direction of GPIO bank 8 bits, 0 = output, 1 = input"]
pub type Dir8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Direction of GPIO bank 8 bits, 0 = output, 1 = input"]
    #[inline(always)]
    pub fn dir8(&self) -> Dir8R {
        Dir8R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Direction of GPIO bank 8 bits, 0 = output, 1 = input"]
    #[inline(always)]
    #[must_use]
    pub fn dir8(&mut self) -> Dir8W<MemDir8Spec> {
        Dir8W::new(self, 0)
    }
}
#[doc = "Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dir8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dir8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemDir8Spec;
impl crate::RegisterSpec for MemDir8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_dir8::R`](R) reader structure"]
impl crate::Readable for MemDir8Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_dir8::W`](W) writer structure"]
impl crate::Writable for MemDir8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_DIR8 to value 0x0006_5535"]
impl crate::Resettable for MemDir8Spec {
    const RESET_VALUE: u32 = 0x0006_5535;
}
