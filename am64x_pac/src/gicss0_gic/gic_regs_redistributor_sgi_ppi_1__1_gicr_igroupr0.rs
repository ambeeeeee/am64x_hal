#[doc = "Register `GIC_REGS_Redistributor_SGI_PPI_1__1_GICR_IGROUPR0` reader"]
pub type R = crate::R<GicRegsRedistributorSgiPpi1_1GicrIgroupr0Spec>;
#[doc = "Register `GIC_REGS_Redistributor_SGI_PPI_1__1_GICR_IGROUPR0` writer"]
pub type W = crate::W<GicRegsRedistributorSgiPpi1_1GicrIgroupr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GICR_IGROUPR0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_redistributor_sgi_ppi_1__1_gicr_igroupr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_redistributor_sgi_ppi_1__1_gicr_igroupr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsRedistributorSgiPpi1_1GicrIgroupr0Spec;
impl crate::RegisterSpec for GicRegsRedistributorSgiPpi1_1GicrIgroupr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_redistributor_sgi_ppi_1__1_gicr_igroupr0::R`](R) reader structure"]
impl crate::Readable for GicRegsRedistributorSgiPpi1_1GicrIgroupr0Spec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_redistributor_sgi_ppi_1__1_gicr_igroupr0::W`](W) writer structure"]
impl crate::Writable for GicRegsRedistributorSgiPpi1_1GicrIgroupr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Redistributor_SGI_PPI_1__1_GICR_IGROUPR0 to value 0"]
impl crate::Resettable for GicRegsRedistributorSgiPpi1_1GicrIgroupr0Spec {
    const RESET_VALUE: u32 = 0;
}
