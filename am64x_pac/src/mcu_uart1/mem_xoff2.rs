#[doc = "Register `MEM_XOFF2` reader"]
pub type R = crate::R<MemXoff2Spec>;
#[doc = "Register `MEM_XOFF2` writer"]
pub type W = crate::W<MemXoff2Spec>;
#[doc = "Field `XOFF_WORD2` reader - 7:0\\]
Used to store the 8-bit XOFF2 character in used in UART modes."]
pub type XoffWord2R = crate::FieldReader;
#[doc = "Field `XOFF_WORD2` writer - 7:0\\]
Used to store the 8-bit XOFF2 character in used in UART modes."]
pub type XoffWord2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Used to store the 8-bit XOFF2 character in used in UART modes."]
    #[inline(always)]
    pub fn xoff_word2(&self) -> XoffWord2R {
        XoffWord2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Used to store the 8-bit XOFF2 character in used in UART modes."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_word2(&mut self) -> XoffWord2W<MemXoff2Spec> {
        XoffWord2W::new(self, 0)
    }
}
#[doc = "XOFF2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_xoff2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_xoff2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemXoff2Spec;
impl crate::RegisterSpec for MemXoff2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_xoff2::R`](R) reader structure"]
impl crate::Readable for MemXoff2Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_xoff2::W`](W) writer structure"]
impl crate::Writable for MemXoff2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_XOFF2 to value 0"]
impl crate::Resettable for MemXoff2Spec {
    const RESET_VALUE: u32 = 0;
}
