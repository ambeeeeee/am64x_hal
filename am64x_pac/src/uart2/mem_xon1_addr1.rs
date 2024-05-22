#[doc = "Register `MEM_XON1_ADDR1` reader"]
pub type R = crate::R<MemXon1Addr1Spec>;
#[doc = "Register `MEM_XON1_ADDR1` writer"]
pub type W = crate::W<MemXon1Addr1Spec>;
#[doc = "Field `XON_WORD1` reader - 7:0\\]
Used to store the 8-bit XON1 character in UART modes and ADDR1 address 1 for IrDA modes."]
pub type XonWord1R = crate::FieldReader;
#[doc = "Field `XON_WORD1` writer - 7:0\\]
Used to store the 8-bit XON1 character in UART modes and ADDR1 address 1 for IrDA modes."]
pub type XonWord1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Used to store the 8-bit XON1 character in UART modes and ADDR1 address 1 for IrDA modes."]
    #[inline(always)]
    pub fn xon_word1(&self) -> XonWord1R {
        XonWord1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Used to store the 8-bit XON1 character in UART modes and ADDR1 address 1 for IrDA modes."]
    #[inline(always)]
    #[must_use]
    pub fn xon_word1(&mut self) -> XonWord1W<MemXon1Addr1Spec> {
        XonWord1W::new(self, 0)
    }
}
#[doc = "XON1/ADDR1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_xon1_addr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_xon1_addr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemXon1Addr1Spec;
impl crate::RegisterSpec for MemXon1Addr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_xon1_addr1::R`](R) reader structure"]
impl crate::Readable for MemXon1Addr1Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_xon1_addr1::W`](W) writer structure"]
impl crate::Writable for MemXon1Addr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_XON1_ADDR1 to value 0"]
impl crate::Resettable for MemXon1Addr1Spec {
    const RESET_VALUE: u32 = 0;
}
