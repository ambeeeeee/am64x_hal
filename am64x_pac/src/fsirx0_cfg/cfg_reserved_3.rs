#[doc = "Register `CFG_Reserved_3` reader"]
pub type R = crate::R<CfgReserved3Spec>;
#[doc = "Register `CFG_Reserved_3` writer"]
pub type W = crate::W<CfgReserved3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFG_Reserved_3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgReserved3Spec;
impl crate::RegisterSpec for CfgReserved3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_reserved_3::R`](R) reader structure"]
impl crate::Readable for CfgReserved3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_reserved_3::W`](W) writer structure"]
impl crate::Writable for CfgReserved3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_Reserved_3 to value 0"]
impl crate::Resettable for CfgReserved3Spec {
    const RESET_VALUE: u16 = 0;
}
