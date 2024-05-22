#[doc = "Register `GIC_REGS_Distributor__24_GICD_IPRIORITYR58` reader"]
pub type R = crate::R<GicRegsDistributor_24GicdIpriorityr58Spec>;
#[doc = "Register `GIC_REGS_Distributor__24_GICD_IPRIORITYR58` writer"]
pub type W = crate::W<GicRegsDistributor_24GicdIpriorityr58Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GICD_IPRIORITYR58\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_distributor__24_gicd_ipriorityr58::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_distributor__24_gicd_ipriorityr58::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsDistributor_24GicdIpriorityr58Spec;
impl crate::RegisterSpec for GicRegsDistributor_24GicdIpriorityr58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_distributor__24_gicd_ipriorityr58::R`](R) reader structure"]
impl crate::Readable for GicRegsDistributor_24GicdIpriorityr58Spec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_distributor__24_gicd_ipriorityr58::W`](W) writer structure"]
impl crate::Writable for GicRegsDistributor_24GicdIpriorityr58Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Distributor__24_GICD_IPRIORITYR58 to value 0"]
impl crate::Resettable for GicRegsDistributor_24GicdIpriorityr58Spec {
    const RESET_VALUE: u32 = 0;
}
