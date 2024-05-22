#[doc = "Register `GIC_REGS_Redistributor_SGI_PPI_0__13_GICR_MISCSTATUSR` reader"]
pub type R = crate::R<GicRegsRedistributorSgiPpi0_13GicrMiscstatusrSpec>;
#[doc = "Register `GIC_REGS_Redistributor_SGI_PPI_0__13_GICR_MISCSTATUSR` writer"]
pub type W = crate::W<GicRegsRedistributorSgiPpi0_13GicrMiscstatusrSpec>;
#[doc = "Field `REDISTRIBUTOR_SGI_PPI_0__13_GICR_MISCSTATUSR__0_1` reader - 0:0\\]
EnableGrp0"]
pub type RedistributorSgiPpi0_13GicrMiscstatusr_0_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_SGI_PPI_0__13_GICR_MISCSTATUSR__0_1` writer - 0:0\\]
EnableGrp0"]
pub type RedistributorSgiPpi0_13GicrMiscstatusr_0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDISTRIBUTOR_SGI_PPI_0__13_GICR_MISCSTATUSR__1_1` reader - 1:1\\]
EnableGrp1_NS"]
pub type RedistributorSgiPpi0_13GicrMiscstatusr_1_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_SGI_PPI_0__13_GICR_MISCSTATUSR__1_1` writer - 1:1\\]
EnableGrp1_NS"]
pub type RedistributorSgiPpi0_13GicrMiscstatusr_1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDISTRIBUTOR_SGI_PPI_0__13_GICR_MISCSTATUSR__2_1` reader - 2:2\\]
EnableGrp1_S"]
pub type RedistributorSgiPpi0_13GicrMiscstatusr_2_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_SGI_PPI_0__13_GICR_MISCSTATUSR__2_1` writer - 2:2\\]
EnableGrp1_S"]
pub type RedistributorSgiPpi0_13GicrMiscstatusr_2_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDISTRIBUTOR_SGI_PPI_0__13_GICR_MISCSTATUSR__31_1` reader - 31:31\\]
cpu_active"]
pub type RedistributorSgiPpi0_13GicrMiscstatusr_31_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_SGI_PPI_0__13_GICR_MISCSTATUSR__31_1` writer - 31:31\\]
cpu_active"]
pub type RedistributorSgiPpi0_13GicrMiscstatusr_31_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
EnableGrp0"]
    #[inline(always)]
    pub fn redistributor_sgi_ppi_0__13_gicr_miscstatusr__0_1(
        &self,
    ) -> RedistributorSgiPpi0_13GicrMiscstatusr_0_1R {
        RedistributorSgiPpi0_13GicrMiscstatusr_0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
EnableGrp1_NS"]
    #[inline(always)]
    pub fn redistributor_sgi_ppi_0__13_gicr_miscstatusr__1_1(
        &self,
    ) -> RedistributorSgiPpi0_13GicrMiscstatusr_1_1R {
        RedistributorSgiPpi0_13GicrMiscstatusr_1_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
EnableGrp1_S"]
    #[inline(always)]
    pub fn redistributor_sgi_ppi_0__13_gicr_miscstatusr__2_1(
        &self,
    ) -> RedistributorSgiPpi0_13GicrMiscstatusr_2_1R {
        RedistributorSgiPpi0_13GicrMiscstatusr_2_1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
cpu_active"]
    #[inline(always)]
    pub fn redistributor_sgi_ppi_0__13_gicr_miscstatusr__31_1(
        &self,
    ) -> RedistributorSgiPpi0_13GicrMiscstatusr_31_1R {
        RedistributorSgiPpi0_13GicrMiscstatusr_31_1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
EnableGrp0"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_sgi_ppi_0__13_gicr_miscstatusr__0_1(
        &mut self,
    ) -> RedistributorSgiPpi0_13GicrMiscstatusr_0_1W<
        GicRegsRedistributorSgiPpi0_13GicrMiscstatusrSpec,
    > {
        RedistributorSgiPpi0_13GicrMiscstatusr_0_1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
EnableGrp1_NS"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_sgi_ppi_0__13_gicr_miscstatusr__1_1(
        &mut self,
    ) -> RedistributorSgiPpi0_13GicrMiscstatusr_1_1W<
        GicRegsRedistributorSgiPpi0_13GicrMiscstatusrSpec,
    > {
        RedistributorSgiPpi0_13GicrMiscstatusr_1_1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
EnableGrp1_S"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_sgi_ppi_0__13_gicr_miscstatusr__2_1(
        &mut self,
    ) -> RedistributorSgiPpi0_13GicrMiscstatusr_2_1W<
        GicRegsRedistributorSgiPpi0_13GicrMiscstatusrSpec,
    > {
        RedistributorSgiPpi0_13GicrMiscstatusr_2_1W::new(self, 2)
    }
    #[doc = "Bit 31 - 31:31\\]
cpu_active"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_sgi_ppi_0__13_gicr_miscstatusr__31_1(
        &mut self,
    ) -> RedistributorSgiPpi0_13GicrMiscstatusr_31_1W<
        GicRegsRedistributorSgiPpi0_13GicrMiscstatusrSpec,
    > {
        RedistributorSgiPpi0_13GicrMiscstatusr_31_1W::new(self, 31)
    }
}
#[doc = "GICR_MISCSTATUSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_redistributor_sgi_ppi_0__13_gicr_miscstatusr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_redistributor_sgi_ppi_0__13_gicr_miscstatusr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsRedistributorSgiPpi0_13GicrMiscstatusrSpec;
impl crate::RegisterSpec for GicRegsRedistributorSgiPpi0_13GicrMiscstatusrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_redistributor_sgi_ppi_0__13_gicr_miscstatusr::R`](R) reader structure"]
impl crate::Readable for GicRegsRedistributorSgiPpi0_13GicrMiscstatusrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_redistributor_sgi_ppi_0__13_gicr_miscstatusr::W`](W) writer structure"]
impl crate::Writable for GicRegsRedistributorSgiPpi0_13GicrMiscstatusrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Redistributor_SGI_PPI_0__13_GICR_MISCSTATUSR to value 0"]
impl crate::Resettable for GicRegsRedistributorSgiPpi0_13GicrMiscstatusrSpec {
    const RESET_VALUE: u32 = 0;
}
