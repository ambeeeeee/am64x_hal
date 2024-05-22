#[doc = "Register `GIC_REGS_Distributor__16_GICD_ISPENDR2` reader"]
pub type R = crate::R<GicRegsDistributor_16GicdIspendr2Spec>;
#[doc = "Register `GIC_REGS_Distributor__16_GICD_ISPENDR2` writer"]
pub type W = crate::W<GicRegsDistributor_16GicdIspendr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GICD_ISPENDR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_distributor__16_gicd_ispendr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_distributor__16_gicd_ispendr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsDistributor_16GicdIspendr2Spec;
impl crate::RegisterSpec for GicRegsDistributor_16GicdIspendr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_distributor__16_gicd_ispendr2::R`](R) reader structure"]
impl crate::Readable for GicRegsDistributor_16GicdIspendr2Spec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_distributor__16_gicd_ispendr2::W`](W) writer structure"]
impl crate::Writable for GicRegsDistributor_16GicdIspendr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Distributor__16_GICD_ISPENDR2 to value 0"]
impl crate::Resettable for GicRegsDistributor_16GicdIspendr2Spec {
    const RESET_VALUE: u32 = 0;
}
