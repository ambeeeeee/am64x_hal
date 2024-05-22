#[doc = "Register `GIC_REGS_Redistributor_control_LPI_0__16_GICR_PIDR1` reader"]
pub type R = crate::R<GicRegsRedistributorControlLpi0_16GicrPidr1Spec>;
#[doc = "Register `GIC_REGS_Redistributor_control_LPI_0__16_GICR_PIDR1` writer"]
pub type W = crate::W<GicRegsRedistributorControlLpi0_16GicrPidr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GICR_PIDR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_redistributor_control_lpi_0__16_gicr_pidr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_redistributor_control_lpi_0__16_gicr_pidr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsRedistributorControlLpi0_16GicrPidr1Spec;
impl crate::RegisterSpec for GicRegsRedistributorControlLpi0_16GicrPidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_redistributor_control_lpi_0__16_gicr_pidr1::R`](R) reader structure"]
impl crate::Readable for GicRegsRedistributorControlLpi0_16GicrPidr1Spec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_redistributor_control_lpi_0__16_gicr_pidr1::W`](W) writer structure"]
impl crate::Writable for GicRegsRedistributorControlLpi0_16GicrPidr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Redistributor_control_LPI_0__16_GICR_PIDR1 to value 0"]
impl crate::Resettable for GicRegsRedistributorControlLpi0_16GicrPidr1Spec {
    const RESET_VALUE: u32 = 0;
}
