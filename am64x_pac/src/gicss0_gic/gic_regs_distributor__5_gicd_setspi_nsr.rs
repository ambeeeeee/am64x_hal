#[doc = "Register `GIC_REGS_Distributor__5_GICD_SETSPI_NSR` reader"]
pub type R = crate::R<GicRegsDistributor_5GicdSetspiNsrSpec>;
#[doc = "Register `GIC_REGS_Distributor__5_GICD_SETSPI_NSR` writer"]
pub type W = crate::W<GicRegsDistributor_5GicdSetspiNsrSpec>;
#[doc = "Field `DISTRIBUTOR__5_GICD_SETSPI_NSR__0_10` reader - 9:0\\]
SPI ID"]
pub type Distributor_5GicdSetspiNsr_0_10R = crate::FieldReader<u16>;
#[doc = "Field `DISTRIBUTOR__5_GICD_SETSPI_NSR__0_10` writer - 9:0\\]
SPI ID"]
pub type Distributor_5GicdSetspiNsr_0_10W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
SPI ID"]
    #[inline(always)]
    pub fn distributor__5_gicd_setspi_nsr__0_10(&self) -> Distributor_5GicdSetspiNsr_0_10R {
        Distributor_5GicdSetspiNsr_0_10R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
SPI ID"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__5_gicd_setspi_nsr__0_10(
        &mut self,
    ) -> Distributor_5GicdSetspiNsr_0_10W<GicRegsDistributor_5GicdSetspiNsrSpec> {
        Distributor_5GicdSetspiNsr_0_10W::new(self, 0)
    }
}
#[doc = "GICD_SETSPI_NSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_distributor__5_gicd_setspi_nsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_distributor__5_gicd_setspi_nsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsDistributor_5GicdSetspiNsrSpec;
impl crate::RegisterSpec for GicRegsDistributor_5GicdSetspiNsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_distributor__5_gicd_setspi_nsr::R`](R) reader structure"]
impl crate::Readable for GicRegsDistributor_5GicdSetspiNsrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_distributor__5_gicd_setspi_nsr::W`](W) writer structure"]
impl crate::Writable for GicRegsDistributor_5GicdSetspiNsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Distributor__5_GICD_SETSPI_NSR to value 0"]
impl crate::Resettable for GicRegsDistributor_5GicdSetspiNsrSpec {
    const RESET_VALUE: u32 = 0;
}
