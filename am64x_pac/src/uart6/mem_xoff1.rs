#[doc = "Register `MEM_XOFF1` reader"]
pub type R = crate::R<MemXoff1Spec>;
#[doc = "Register `MEM_XOFF1` writer"]
pub type W = crate::W<MemXoff1Spec>;
#[doc = "Field `XOFF_WORD1` reader - 7:0\\]
Used to store the 8-bit XOFF1 character in used in UART modes."]
pub type XoffWord1R = crate::FieldReader;
#[doc = "Field `XOFF_WORD1` writer - 7:0\\]
Used to store the 8-bit XOFF1 character in used in UART modes."]
pub type XoffWord1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Used to store the 8-bit XOFF1 character in used in UART modes."]
    #[inline(always)]
    pub fn xoff_word1(&self) -> XoffWord1R {
        XoffWord1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Used to store the 8-bit XOFF1 character in used in UART modes."]
    #[inline(always)]
    #[must_use]
    pub fn xoff_word1(&mut self) -> XoffWord1W<MemXoff1Spec> {
        XoffWord1W::new(self, 0)
    }
}
#[doc = "XOFF1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_xoff1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_xoff1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemXoff1Spec;
impl crate::RegisterSpec for MemXoff1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_xoff1::R`](R) reader structure"]
impl crate::Readable for MemXoff1Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_xoff1::W`](W) writer structure"]
impl crate::Writable for MemXoff1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_XOFF1 to value 0"]
impl crate::Resettable for MemXoff1Spec {
    const RESET_VALUE: u32 = 0;
}
