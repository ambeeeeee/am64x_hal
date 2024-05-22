#[doc = "Register `GIC_REGS_Redistributor_control_LPI_0__2_GICR_CTLR` reader"]
pub type R = crate::R<GicRegsRedistributorControlLpi0_2GicrCtlrSpec>;
#[doc = "Register `GIC_REGS_Redistributor_control_LPI_0__2_GICR_CTLR` writer"]
pub type W = crate::W<GicRegsRedistributorControlLpi0_2GicrCtlrSpec>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__2_GICR_CTLR__0_1` reader - 0:0\\]
Enable LPIs"]
pub type RedistributorControlLpi0_2GicrCtlr_0_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__2_GICR_CTLR__0_1` writer - 0:0\\]
Enable LPIs"]
pub type RedistributorControlLpi0_2GicrCtlr_0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__2_GICR_CTLR__3_1` reader - 3:3\\]
Register Write Pending"]
pub type RedistributorControlLpi0_2GicrCtlr_3_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__2_GICR_CTLR__3_1` writer - 3:3\\]
Register Write Pending"]
pub type RedistributorControlLpi0_2GicrCtlr_3_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__2_GICR_CTLR__31_1` reader - 31:31\\]
Upstream Write Pending"]
pub type RedistributorControlLpi0_2GicrCtlr_31_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__2_GICR_CTLR__31_1` writer - 31:31\\]
Upstream Write Pending"]
pub type RedistributorControlLpi0_2GicrCtlr_31_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable LPIs"]
    #[inline(always)]
    pub fn redistributor_control_lpi_0__2_gicr_ctlr__0_1(
        &self,
    ) -> RedistributorControlLpi0_2GicrCtlr_0_1R {
        RedistributorControlLpi0_2GicrCtlr_0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Register Write Pending"]
    #[inline(always)]
    pub fn redistributor_control_lpi_0__2_gicr_ctlr__3_1(
        &self,
    ) -> RedistributorControlLpi0_2GicrCtlr_3_1R {
        RedistributorControlLpi0_2GicrCtlr_3_1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Upstream Write Pending"]
    #[inline(always)]
    pub fn redistributor_control_lpi_0__2_gicr_ctlr__31_1(
        &self,
    ) -> RedistributorControlLpi0_2GicrCtlr_31_1R {
        RedistributorControlLpi0_2GicrCtlr_31_1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable LPIs"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_0__2_gicr_ctlr__0_1(
        &mut self,
    ) -> RedistributorControlLpi0_2GicrCtlr_0_1W<GicRegsRedistributorControlLpi0_2GicrCtlrSpec>
    {
        RedistributorControlLpi0_2GicrCtlr_0_1W::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Register Write Pending"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_0__2_gicr_ctlr__3_1(
        &mut self,
    ) -> RedistributorControlLpi0_2GicrCtlr_3_1W<GicRegsRedistributorControlLpi0_2GicrCtlrSpec>
    {
        RedistributorControlLpi0_2GicrCtlr_3_1W::new(self, 3)
    }
    #[doc = "Bit 31 - 31:31\\]
Upstream Write Pending"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_0__2_gicr_ctlr__31_1(
        &mut self,
    ) -> RedistributorControlLpi0_2GicrCtlr_31_1W<GicRegsRedistributorControlLpi0_2GicrCtlrSpec>
    {
        RedistributorControlLpi0_2GicrCtlr_31_1W::new(self, 31)
    }
}
#[doc = "GICR_CTLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_redistributor_control_lpi_0__2_gicr_ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_redistributor_control_lpi_0__2_gicr_ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsRedistributorControlLpi0_2GicrCtlrSpec;
impl crate::RegisterSpec for GicRegsRedistributorControlLpi0_2GicrCtlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_redistributor_control_lpi_0__2_gicr_ctlr::R`](R) reader structure"]
impl crate::Readable for GicRegsRedistributorControlLpi0_2GicrCtlrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_redistributor_control_lpi_0__2_gicr_ctlr::W`](W) writer structure"]
impl crate::Writable for GicRegsRedistributorControlLpi0_2GicrCtlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Redistributor_control_LPI_0__2_GICR_CTLR to value 0"]
impl crate::Resettable for GicRegsRedistributorControlLpi0_2GicrCtlrSpec {
    const RESET_VALUE: u32 = 0;
}
