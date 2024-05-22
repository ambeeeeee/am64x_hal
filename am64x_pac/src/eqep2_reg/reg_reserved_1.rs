#[doc = "Register `REG_Reserved_1` reader"]
pub type R = crate::R<RegReserved1Spec>;
#[doc = "Register `REG_Reserved_1` writer"]
pub type W = crate::W<RegReserved1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "REG_Reserved_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_reserved_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_reserved_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegReserved1Spec;
impl crate::RegisterSpec for RegReserved1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`reg_reserved_1::R`](R) reader structure"]
impl crate::Readable for RegReserved1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_reserved_1::W`](W) writer structure"]
impl crate::Writable for RegReserved1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets REG_Reserved_1 to value 0"]
impl crate::Resettable for RegReserved1Spec {
    const RESET_VALUE: u16 = 0;
}
