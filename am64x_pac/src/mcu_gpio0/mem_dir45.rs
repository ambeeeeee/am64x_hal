#[doc = "Register `MEM_DIR45` reader"]
pub type R = crate::R<MemDir45Spec>;
#[doc = "Register `MEM_DIR45` writer"]
pub type W = crate::W<MemDir45Spec>;
#[doc = "Field `DIR4` reader - 15:0\\]
Direction of GPIO bank 4 bits, 0 = output, 1 = input"]
pub type Dir4R = crate::FieldReader<u16>;
#[doc = "Field `DIR4` writer - 15:0\\]
Direction of GPIO bank 4 bits, 0 = output, 1 = input"]
pub type Dir4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DIR5` reader - 31:16\\]
Direction of GPIO bank 5 bits, 0 = output, 1 = input"]
pub type Dir5R = crate::FieldReader<u16>;
#[doc = "Field `DIR5` writer - 31:16\\]
Direction of GPIO bank 5 bits, 0 = output, 1 = input"]
pub type Dir5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Direction of GPIO bank 4 bits, 0 = output, 1 = input"]
    #[inline(always)]
    pub fn dir4(&self) -> Dir4R {
        Dir4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Direction of GPIO bank 5 bits, 0 = output, 1 = input"]
    #[inline(always)]
    pub fn dir5(&self) -> Dir5R {
        Dir5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Direction of GPIO bank 4 bits, 0 = output, 1 = input"]
    #[inline(always)]
    #[must_use]
    pub fn dir4(&mut self) -> Dir4W<MemDir45Spec> {
        Dir4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Direction of GPIO bank 5 bits, 0 = output, 1 = input"]
    #[inline(always)]
    #[must_use]
    pub fn dir5(&mut self) -> Dir5W<MemDir45Spec> {
        Dir5W::new(self, 16)
    }
}
#[doc = "Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dir45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dir45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemDir45Spec;
impl crate::RegisterSpec for MemDir45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_dir45::R`](R) reader structure"]
impl crate::Readable for MemDir45Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_dir45::W`](W) writer structure"]
impl crate::Writable for MemDir45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_DIR45 to value 0x0006_5535"]
impl crate::Resettable for MemDir45Spec {
    const RESET_VALUE: u32 = 0x0006_5535;
}
