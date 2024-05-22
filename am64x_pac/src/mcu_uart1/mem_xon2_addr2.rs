#[doc = "Register `MEM_XON2_ADDR2` reader"]
pub type R = crate::R<MemXon2Addr2Spec>;
#[doc = "Register `MEM_XON2_ADDR2` writer"]
pub type W = crate::W<MemXon2Addr2Spec>;
#[doc = "Field `XON_WORD2` reader - 7:0\\]
Used to store the 8-bit XON2 character in UART modes and ADDR2 address 2 for IrDA modes."]
pub type XonWord2R = crate::FieldReader;
#[doc = "Field `XON_WORD2` writer - 7:0\\]
Used to store the 8-bit XON2 character in UART modes and ADDR2 address 2 for IrDA modes."]
pub type XonWord2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Used to store the 8-bit XON2 character in UART modes and ADDR2 address 2 for IrDA modes."]
    #[inline(always)]
    pub fn xon_word2(&self) -> XonWord2R {
        XonWord2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Used to store the 8-bit XON2 character in UART modes and ADDR2 address 2 for IrDA modes."]
    #[inline(always)]
    #[must_use]
    pub fn xon_word2(&mut self) -> XonWord2W<MemXon2Addr2Spec> {
        XonWord2W::new(self, 0)
    }
}
#[doc = "XON2/ADDR2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_xon2_addr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_xon2_addr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemXon2Addr2Spec;
impl crate::RegisterSpec for MemXon2Addr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_xon2_addr2::R`](R) reader structure"]
impl crate::Readable for MemXon2Addr2Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_xon2_addr2::W`](W) writer structure"]
impl crate::Writable for MemXon2Addr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_XON2_ADDR2 to value 0"]
impl crate::Resettable for MemXon2Addr2Spec {
    const RESET_VALUE: u32 = 0;
}
