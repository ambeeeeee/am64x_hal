#[doc = "Register `GIC_REGS_Distributor__33_GICD_NSACR9` reader"]
pub type R = crate::R<GicRegsDistributor_33GicdNsacr9Spec>;
#[doc = "Register `GIC_REGS_Distributor__33_GICD_NSACR9` writer"]
pub type W = crate::W<GicRegsDistributor_33GicdNsacr9Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GICD_NSACR9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_distributor__33_gicd_nsacr9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_distributor__33_gicd_nsacr9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsDistributor_33GicdNsacr9Spec;
impl crate::RegisterSpec for GicRegsDistributor_33GicdNsacr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_distributor__33_gicd_nsacr9::R`](R) reader structure"]
impl crate::Readable for GicRegsDistributor_33GicdNsacr9Spec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_distributor__33_gicd_nsacr9::W`](W) writer structure"]
impl crate::Writable for GicRegsDistributor_33GicdNsacr9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Distributor__33_GICD_NSACR9 to value 0"]
impl crate::Resettable for GicRegsDistributor_33GicdNsacr9Spec {
    const RESET_VALUE: u32 = 0;
}
