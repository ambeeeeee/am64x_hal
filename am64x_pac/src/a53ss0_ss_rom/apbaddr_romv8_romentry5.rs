#[doc = "Register `APBADDR_ROMV8_ROMENTRY5` reader"]
pub type R = crate::R<ApbaddrRomv8Romentry5Spec>;
#[doc = "Register `APBADDR_ROMV8_ROMENTRY5` writer"]
pub type W = crate::W<ApbaddrRomv8Romentry5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ROM Table Entry Register 5 (CPU 1 CTI Component)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_romv8_romentry5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_romv8_romentry5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrRomv8Romentry5Spec;
impl crate::RegisterSpec for ApbaddrRomv8Romentry5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_romv8_romentry5::R`](R) reader structure"]
impl crate::Readable for ApbaddrRomv8Romentry5Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_romv8_romentry5::W`](W) writer structure"]
impl crate::Writable for ApbaddrRomv8Romentry5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ROMV8_ROMENTRY5 to value 0"]
impl crate::Resettable for ApbaddrRomv8Romentry5Spec {
    const RESET_VALUE: u32 = 0;
}
