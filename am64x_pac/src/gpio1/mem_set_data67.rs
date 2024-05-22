#[doc = "Register `MEM_SET_DATA67` reader"]
pub type R = crate::R<MemSetData67Spec>;
#[doc = "Register `MEM_SET_DATA67` writer"]
pub type W = crate::W<MemSetData67Spec>;
#[doc = "Field `SET6` reader - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 6 bits Reading it returns the output drive state"]
pub type Set6R = crate::FieldReader<u16>;
#[doc = "Field `SET6` writer - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 6 bits Reading it returns the output drive state"]
pub type Set6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SET7` reader - 31:16\\]
Writing 1 sets the output drive state of GPIO bank 7 bits Reading it returns the output drive state"]
pub type Set7R = crate::FieldReader<u16>;
#[doc = "Field `SET7` writer - 31:16\\]
Writing 1 sets the output drive state of GPIO bank 7 bits Reading it returns the output drive state"]
pub type Set7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 6 bits Reading it returns the output drive state"]
    #[inline(always)]
    pub fn set6(&self) -> Set6R {
        Set6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 sets the output drive state of GPIO bank 7 bits Reading it returns the output drive state"]
    #[inline(always)]
    pub fn set7(&self) -> Set7R {
        Set7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 6 bits Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn set6(&mut self) -> Set6W<MemSetData67Spec> {
        Set6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 sets the output drive state of GPIO bank 7 bits Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn set7(&mut self) -> Set7W<MemSetData67Spec> {
        Set7W::new(self, 16)
    }
}
#[doc = "Set Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_data67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_data67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSetData67Spec;
impl crate::RegisterSpec for MemSetData67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_set_data67::R`](R) reader structure"]
impl crate::Readable for MemSetData67Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_set_data67::W`](W) writer structure"]
impl crate::Writable for MemSetData67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SET_DATA67 to value 0"]
impl crate::Resettable for MemSetData67Spec {
    const RESET_VALUE: u32 = 0;
}
