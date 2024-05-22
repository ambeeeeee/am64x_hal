#[doc = "Register `MEM_DIR01` reader"]
pub type R = crate::R<MemDir01Spec>;
#[doc = "Register `MEM_DIR01` writer"]
pub type W = crate::W<MemDir01Spec>;
#[doc = "Field `DIR0` reader - 15:0\\]
Direction of GPIO bank 0 bits, 0 = output, 1 = input"]
pub type Dir0R = crate::FieldReader<u16>;
#[doc = "Field `DIR0` writer - 15:0\\]
Direction of GPIO bank 0 bits, 0 = output, 1 = input"]
pub type Dir0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DIR1` reader - 31:16\\]
Direction of GPIO bank 1 bits, 0 = output, 1 = input"]
pub type Dir1R = crate::FieldReader<u16>;
#[doc = "Field `DIR1` writer - 31:16\\]
Direction of GPIO bank 1 bits, 0 = output, 1 = input"]
pub type Dir1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Direction of GPIO bank 0 bits, 0 = output, 1 = input"]
    #[inline(always)]
    pub fn dir0(&self) -> Dir0R {
        Dir0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Direction of GPIO bank 1 bits, 0 = output, 1 = input"]
    #[inline(always)]
    pub fn dir1(&self) -> Dir1R {
        Dir1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Direction of GPIO bank 0 bits, 0 = output, 1 = input"]
    #[inline(always)]
    #[must_use]
    pub fn dir0(&mut self) -> Dir0W<MemDir01Spec> {
        Dir0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Direction of GPIO bank 1 bits, 0 = output, 1 = input"]
    #[inline(always)]
    #[must_use]
    pub fn dir1(&mut self) -> Dir1W<MemDir01Spec> {
        Dir1W::new(self, 16)
    }
}
#[doc = "Direction Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dir01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dir01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemDir01Spec;
impl crate::RegisterSpec for MemDir01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_dir01::R`](R) reader structure"]
impl crate::Readable for MemDir01Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_dir01::W`](W) writer structure"]
impl crate::Writable for MemDir01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_DIR01 to value 0x0006_5535"]
impl crate::Resettable for MemDir01Spec {
    const RESET_VALUE: u32 = 0x0006_5535;
}
