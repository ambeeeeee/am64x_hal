#[doc = "Register `MEM_SET_DATA23` reader"]
pub type R = crate::R<MemSetData23Spec>;
#[doc = "Register `MEM_SET_DATA23` writer"]
pub type W = crate::W<MemSetData23Spec>;
#[doc = "Field `SET2` reader - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 2 bits Reading it returns the output drive state"]
pub type Set2R = crate::FieldReader<u16>;
#[doc = "Field `SET2` writer - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 2 bits Reading it returns the output drive state"]
pub type Set2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SET3` reader - 31:16\\]
Writing 1 sets the output drive state of GPIO bank 3 bits Reading it returns the output drive state"]
pub type Set3R = crate::FieldReader<u16>;
#[doc = "Field `SET3` writer - 31:16\\]
Writing 1 sets the output drive state of GPIO bank 3 bits Reading it returns the output drive state"]
pub type Set3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 2 bits Reading it returns the output drive state"]
    #[inline(always)]
    pub fn set2(&self) -> Set2R {
        Set2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 sets the output drive state of GPIO bank 3 bits Reading it returns the output drive state"]
    #[inline(always)]
    pub fn set3(&self) -> Set3R {
        Set3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 2 bits Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn set2(&mut self) -> Set2W<MemSetData23Spec> {
        Set2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 sets the output drive state of GPIO bank 3 bits Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn set3(&mut self) -> Set3W<MemSetData23Spec> {
        Set3W::new(self, 16)
    }
}
#[doc = "Set Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_data23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_data23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSetData23Spec;
impl crate::RegisterSpec for MemSetData23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_set_data23::R`](R) reader structure"]
impl crate::Readable for MemSetData23Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_set_data23::W`](W) writer structure"]
impl crate::Writable for MemSetData23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SET_DATA23 to value 0"]
impl crate::Resettable for MemSetData23Spec {
    const RESET_VALUE: u32 = 0;
}
