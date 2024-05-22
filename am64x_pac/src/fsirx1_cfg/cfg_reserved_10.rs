#[doc = "Register `CFG_Reserved_10` reader"]
pub type R = crate::R<CfgReserved10Spec>;
#[doc = "Register `CFG_Reserved_10` writer"]
pub type W = crate::W<CfgReserved10Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFG_Reserved_10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_reserved_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_reserved_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgReserved10Spec;
impl crate::RegisterSpec for CfgReserved10Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_reserved_10::R`](R) reader structure"]
impl crate::Readable for CfgReserved10Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_reserved_10::W`](W) writer structure"]
impl crate::Writable for CfgReserved10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_Reserved_10 to value 0"]
impl crate::Resettable for CfgReserved10Spec {
    const RESET_VALUE: u16 = 0;
}
