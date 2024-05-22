#[doc = "Register `MEM_BAUD_2ND_CHAR` reader"]
pub type R = crate::R<MemBaud2ndCharSpec>;
#[doc = "Register `MEM_BAUD_2ND_CHAR` writer"]
pub type W = crate::W<MemBaud2ndCharSpec>;
impl W {}
#[doc = "Unused\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_baud_2nd_char::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_baud_2nd_char::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemBaud2ndCharSpec;
impl crate::RegisterSpec for MemBaud2ndCharSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_baud_2nd_char::R`](R) reader structure"]
impl crate::Readable for MemBaud2ndCharSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_baud_2nd_char::W`](W) writer structure"]
impl crate::Writable for MemBaud2ndCharSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_BAUD_2ND_CHAR to value 0"]
impl crate::Resettable for MemBaud2ndCharSpec {
    const RESET_VALUE: u32 = 0;
}
