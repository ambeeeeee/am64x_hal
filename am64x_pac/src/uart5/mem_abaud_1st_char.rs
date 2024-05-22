#[doc = "Register `MEM_ABAUD_1ST_CHAR` reader"]
pub type R = crate::R<MemAbaud1stCharSpec>;
#[doc = "Register `MEM_ABAUD_1ST_CHAR` writer"]
pub type W = crate::W<MemAbaud1stCharSpec>;
impl W {}
#[doc = "Unused\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_abaud_1st_char::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_abaud_1st_char::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemAbaud1stCharSpec;
impl crate::RegisterSpec for MemAbaud1stCharSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_abaud_1st_char::R`](R) reader structure"]
impl crate::Readable for MemAbaud1stCharSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_abaud_1st_char::W`](W) writer structure"]
impl crate::Writable for MemAbaud1stCharSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ABAUD_1ST_CHAR to value 0"]
impl crate::Resettable for MemAbaud1stCharSpec {
    const RESET_VALUE: u32 = 0;
}
