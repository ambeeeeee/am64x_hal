#[doc = "Register `GIC_REGS_Redistributor_SGI_PPI_0__14_GICR_PPISR` reader"]
pub type R = crate::R<GicRegsRedistributorSgiPpi0_14GicrPpisrSpec>;
#[doc = "Register `GIC_REGS_Redistributor_SGI_PPI_0__14_GICR_PPISR` writer"]
pub type W = crate::W<GicRegsRedistributorSgiPpi0_14GicrPpisrSpec>;
#[doc = "Field `REDISTRIBUTOR_SGI_PPI_0__14_GICR_PPISR__16_16` reader - 31:16\\]
PPI status"]
pub type RedistributorSgiPpi0_14GicrPpisr_16_16R = crate::FieldReader<u16>;
#[doc = "Field `REDISTRIBUTOR_SGI_PPI_0__14_GICR_PPISR__16_16` writer - 31:16\\]
PPI status"]
pub type RedistributorSgiPpi0_14GicrPpisr_16_16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
PPI status"]
    #[inline(always)]
    pub fn redistributor_sgi_ppi_0__14_gicr_ppisr__16_16(
        &self,
    ) -> RedistributorSgiPpi0_14GicrPpisr_16_16R {
        RedistributorSgiPpi0_14GicrPpisr_16_16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
PPI status"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_sgi_ppi_0__14_gicr_ppisr__16_16(
        &mut self,
    ) -> RedistributorSgiPpi0_14GicrPpisr_16_16W<GicRegsRedistributorSgiPpi0_14GicrPpisrSpec> {
        RedistributorSgiPpi0_14GicrPpisr_16_16W::new(self, 16)
    }
}
#[doc = "GICR_PPISR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_redistributor_sgi_ppi_0__14_gicr_ppisr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_redistributor_sgi_ppi_0__14_gicr_ppisr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsRedistributorSgiPpi0_14GicrPpisrSpec;
impl crate::RegisterSpec for GicRegsRedistributorSgiPpi0_14GicrPpisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_redistributor_sgi_ppi_0__14_gicr_ppisr::R`](R) reader structure"]
impl crate::Readable for GicRegsRedistributorSgiPpi0_14GicrPpisrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_redistributor_sgi_ppi_0__14_gicr_ppisr::W`](W) writer structure"]
impl crate::Writable for GicRegsRedistributorSgiPpi0_14GicrPpisrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Redistributor_SGI_PPI_0__14_GICR_PPISR to value 0"]
impl crate::Resettable for GicRegsRedistributorSgiPpi0_14GicrPpisrSpec {
    const RESET_VALUE: u32 = 0;
}
