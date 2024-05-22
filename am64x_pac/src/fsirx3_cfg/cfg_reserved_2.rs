#[doc = "Register `CFG_Reserved_2` reader"]
pub type R = crate::R<CfgReserved2Spec>;
#[doc = "Register `CFG_Reserved_2` writer"]
pub type W = crate::W<CfgReserved2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFG_Reserved_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgReserved2Spec;
impl crate::RegisterSpec for CfgReserved2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_reserved_2::R`](R) reader structure"]
impl crate::Readable for CfgReserved2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_reserved_2::W`](W) writer structure"]
impl crate::Writable for CfgReserved2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_Reserved_2 to value 0"]
impl crate::Resettable for CfgReserved2Spec {
    const RESET_VALUE: u16 = 0;
}
