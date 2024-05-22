#[doc = "Register `APBADDR_CTI_CPU1_CTIPIDR5` reader"]
pub type R = crate::R<ApbaddrCtiCpu1Ctipidr5Spec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTIPIDR5` writer"]
pub type W = crate::W<ApbaddrCtiCpu1Ctipidr5Spec>;
impl W {}
#[doc = "CTI Peripheral Identification Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctipidr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctipidr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1Ctipidr5Spec;
impl crate::RegisterSpec for ApbaddrCtiCpu1Ctipidr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctipidr5::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1Ctipidr5Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctipidr5::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1Ctipidr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTIPIDR5 to value 0"]
impl crate::Resettable for ApbaddrCtiCpu1Ctipidr5Spec {
    const RESET_VALUE: u32 = 0;
}
