#[doc = "Register `MEM_SET_DATA45` reader"]
pub type R = crate::R<MemSetData45Spec>;
#[doc = "Register `MEM_SET_DATA45` writer"]
pub type W = crate::W<MemSetData45Spec>;
#[doc = "Field `SET4` reader - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 4 bits Reading it returns the output drive state"]
pub type Set4R = crate::FieldReader<u16>;
#[doc = "Field `SET4` writer - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 4 bits Reading it returns the output drive state"]
pub type Set4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SET5` reader - 31:16\\]
Writing 1 sets the output drive state of GPIO bank 5 bits Reading it returns the output drive state"]
pub type Set5R = crate::FieldReader<u16>;
#[doc = "Field `SET5` writer - 31:16\\]
Writing 1 sets the output drive state of GPIO bank 5 bits Reading it returns the output drive state"]
pub type Set5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 4 bits Reading it returns the output drive state"]
    #[inline(always)]
    pub fn set4(&self) -> Set4R {
        Set4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 sets the output drive state of GPIO bank 5 bits Reading it returns the output drive state"]
    #[inline(always)]
    pub fn set5(&self) -> Set5R {
        Set5R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Writing 1 sets the output drive state of GPIO bank 4 bits Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn set4(&mut self) -> Set4W<MemSetData45Spec> {
        Set4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Writing 1 sets the output drive state of GPIO bank 5 bits Reading it returns the output drive state"]
    #[inline(always)]
    #[must_use]
    pub fn set5(&mut self) -> Set5W<MemSetData45Spec> {
        Set5W::new(self, 16)
    }
}
#[doc = "Set Output Drive State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_set_data45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_set_data45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSetData45Spec;
impl crate::RegisterSpec for MemSetData45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_set_data45::R`](R) reader structure"]
impl crate::Readable for MemSetData45Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_set_data45::W`](W) writer structure"]
impl crate::Writable for MemSetData45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SET_DATA45 to value 0"]
impl crate::Resettable for MemSetData45Spec {
    const RESET_VALUE: u32 = 0;
}
