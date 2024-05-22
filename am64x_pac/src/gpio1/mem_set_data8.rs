#[doc = "Register `MEM_SET_DATA8` reader"]
pub type R = crate::R<MemSetData8Spec>;
#[doc = "Register `MEM_SET_DATA8` writer"]
pub type W = crate::W<MemSetData8Spec>;
#[doc = "Field `SET8` reader - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 8 bits Reading it returns the output drive state"]
pub type Set8R = crate::FieldReader<u16>;
#[doc = "Field `SET8` writer - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 8 bits Reading it returns the output drive state"]
pub type Set8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 8 bits Reading it returns the output drive state"]
    #[inline(always)]
    pub fn set8(&self) -> Set8R {
        Set8R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 8 bits Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn set8(&mut self) -> Set8W<MemSetData8Spec> {
        Set8W::new(self, 0)
    }
}
#[doc = "Set Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_data8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_data8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSetData8Spec;
impl crate::RegisterSpec for MemSetData8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_set_data8::R`](R) reader structure"]
impl crate::Readable for MemSetData8Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_set_data8::W`](W) writer structure"]
impl crate::Writable for MemSetData8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SET_DATA8 to value 0"]
impl crate::Resettable for MemSetData8Spec {
    const RESET_VALUE: u32 = 0;
}
