#[doc = "Register `MEM_DIR67` reader"]
pub type R = crate::R<MemDir67Spec>;
#[doc = "Register `MEM_DIR67` writer"]
pub type W = crate::W<MemDir67Spec>;
#[doc = "Field `DIR6` reader - 15:0\\]
Direction of GPIO bank 6 bits, 0 = output, 1 = input"]
pub type Dir6R = crate::FieldReader<u16>;
#[doc = "Field `DIR6` writer - 15:0\\]
Direction of GPIO bank 6 bits, 0 = output, 1 = input"]
pub type Dir6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DIR7` reader - 31:16\\]
Direction of GPIO bank 7 bits, 0 = output, 1 = input"]
pub type Dir7R = crate::FieldReader<u16>;
#[doc = "Field `DIR7` writer - 31:16\\]
Direction of GPIO bank 7 bits, 0 = output, 1 = input"]
pub type Dir7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Direction of GPIO bank 6 bits, 0 = output, 1 = input"]
    #[inline(always)]
    pub fn dir6(&self) -> Dir6R {
        Dir6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Direction of GPIO bank 7 bits, 0 = output, 1 = input"]
    #[inline(always)]
    pub fn dir7(&self) -> Dir7R {
        Dir7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Direction of GPIO bank 6 bits, 0 = output, 1 = input"]
    #[inline(always)]
    #[must_use]
    pub fn dir6(&mut self) -> Dir6W<MemDir67Spec> {
        Dir6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Direction of GPIO bank 7 bits, 0 = output, 1 = input"]
    #[inline(always)]
    #[must_use]
    pub fn dir7(&mut self) -> Dir7W<MemDir67Spec> {
        Dir7W::new(self, 16)
    }
}
#[doc = "Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dir67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dir67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemDir67Spec;
impl crate::RegisterSpec for MemDir67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_dir67::R`](R) reader structure"]
impl crate::Readable for MemDir67Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_dir67::W`](W) writer structure"]
impl crate::Writable for MemDir67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_DIR67 to value 0x0006_5535"]
impl crate::Resettable for MemDir67Spec {
    const RESET_VALUE: u32 = 0x0006_5535;
}
