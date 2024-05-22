#[doc = "Register `MEM_DIR23` reader"]
pub type R = crate::R<MemDir23Spec>;
#[doc = "Register `MEM_DIR23` writer"]
pub type W = crate::W<MemDir23Spec>;
#[doc = "Field `DIR2` reader - 15:0\\]
Direction of GPIO bank 2 bits, 0 = output, 1 = input"]
pub type Dir2R = crate::FieldReader<u16>;
#[doc = "Field `DIR2` writer - 15:0\\]
Direction of GPIO bank 2 bits, 0 = output, 1 = input"]
pub type Dir2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DIR3` reader - 31:16\\]
Direction of GPIO bank 3 bits, 0 = output, 1 = input"]
pub type Dir3R = crate::FieldReader<u16>;
#[doc = "Field `DIR3` writer - 31:16\\]
Direction of GPIO bank 3 bits, 0 = output, 1 = input"]
pub type Dir3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Direction of GPIO bank 2 bits, 0 = output, 1 = input"]
    #[inline(always)]
    pub fn dir2(&self) -> Dir2R {
        Dir2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Direction of GPIO bank 3 bits, 0 = output, 1 = input"]
    #[inline(always)]
    pub fn dir3(&self) -> Dir3R {
        Dir3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Direction of GPIO bank 2 bits, 0 = output, 1 = input"]
    #[inline(always)]
    #[must_use]
    pub fn dir2(&mut self) -> Dir2W<MemDir23Spec> {
        Dir2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Direction of GPIO bank 3 bits, 0 = output, 1 = input"]
    #[inline(always)]
    #[must_use]
    pub fn dir3(&mut self) -> Dir3W<MemDir23Spec> {
        Dir3W::new(self, 16)
    }
}
#[doc = "Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dir23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dir23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemDir23Spec;
impl crate::RegisterSpec for MemDir23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_dir23::R`](R) reader structure"]
impl crate::Readable for MemDir23Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_dir23::W`](W) writer structure"]
impl crate::Writable for MemDir23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_DIR23 to value 0x0006_5535"]
impl crate::Resettable for MemDir23Spec {
    const RESET_VALUE: u32 = 0x0006_5535;
}
