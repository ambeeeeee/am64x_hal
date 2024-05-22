#[doc = "Register `GIC_REGS_Distributor__37_GICD_IROUTER98_upper` reader"]
pub type R = crate::R<GicRegsDistributor_37GicdIrouter98UpperSpec>;
#[doc = "Register `GIC_REGS_Distributor__37_GICD_IROUTER98_upper` writer"]
pub type W = crate::W<GicRegsDistributor_37GicdIrouter98UpperSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GICD_IROUTER98_upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_distributor__37_gicd_irouter98_upper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_distributor__37_gicd_irouter98_upper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsDistributor_37GicdIrouter98UpperSpec;
impl crate::RegisterSpec for GicRegsDistributor_37GicdIrouter98UpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_distributor__37_gicd_irouter98_upper::R`](R) reader structure"]
impl crate::Readable for GicRegsDistributor_37GicdIrouter98UpperSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_distributor__37_gicd_irouter98_upper::W`](W) writer structure"]
impl crate::Writable for GicRegsDistributor_37GicdIrouter98UpperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Distributor__37_GICD_IROUTER98_upper to value 0"]
impl crate::Resettable for GicRegsDistributor_37GicdIrouter98UpperSpec {
    const RESET_VALUE: u32 = 0;
}
