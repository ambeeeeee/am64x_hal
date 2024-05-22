#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__10_GICR_PENDBASER_upper` reader"]
pub type R = crate::R<GicRegsRedistributorControlLpi1_10GicrPendbaserUpperSpec>;
#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__10_GICR_PENDBASER_upper` writer"]
pub type W = crate::W<GicRegsRedistributorControlLpi1_10GicrPendbaserUpperSpec>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__10_GICR_PENDBASER_UPPER__0_16` reader - 15:0\\]
Physical Address \\[47:32\\]"]
pub type RedistributorControlLpi1_10GicrPendbaserUpper_0_16R = crate::FieldReader<u16>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__10_GICR_PENDBASER_UPPER__0_16` writer - 15:0\\]
Physical Address \\[47:32\\]"]
pub type RedistributorControlLpi1_10GicrPendbaserUpper_0_16W<'a, REG> =
    crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__10_GICR_PENDBASER_UPPER__30_1` reader - 30:30\\]
Pending Table Zero"]
pub type RedistributorControlLpi1_10GicrPendbaserUpper_30_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__10_GICR_PENDBASER_UPPER__30_1` writer - 30:30\\]
Pending Table Zero"]
pub type RedistributorControlLpi1_10GicrPendbaserUpper_30_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Physical Address \\[47:32\\]"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__10_gicr_pendbaser_upper__0_16(
        &self,
    ) -> RedistributorControlLpi1_10GicrPendbaserUpper_0_16R {
        RedistributorControlLpi1_10GicrPendbaserUpper_0_16R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - 30:30\\]
Pending Table Zero"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__10_gicr_pendbaser_upper__30_1(
        &self,
    ) -> RedistributorControlLpi1_10GicrPendbaserUpper_30_1R {
        RedistributorControlLpi1_10GicrPendbaserUpper_30_1R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Physical Address \\[47:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__10_gicr_pendbaser_upper__0_16(
        &mut self,
    ) -> RedistributorControlLpi1_10GicrPendbaserUpper_0_16W<
        GicRegsRedistributorControlLpi1_10GicrPendbaserUpperSpec,
    > {
        RedistributorControlLpi1_10GicrPendbaserUpper_0_16W::new(self, 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Pending Table Zero"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__10_gicr_pendbaser_upper__30_1(
        &mut self,
    ) -> RedistributorControlLpi1_10GicrPendbaserUpper_30_1W<
        GicRegsRedistributorControlLpi1_10GicrPendbaserUpperSpec,
    > {
        RedistributorControlLpi1_10GicrPendbaserUpper_30_1W::new(self, 30)
    }
}
#[doc = "GICR_PENDBASER_upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_redistributor_control_lpi_1__10_gicr_pendbaser_upper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_redistributor_control_lpi_1__10_gicr_pendbaser_upper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsRedistributorControlLpi1_10GicrPendbaserUpperSpec;
impl crate::RegisterSpec for GicRegsRedistributorControlLpi1_10GicrPendbaserUpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_redistributor_control_lpi_1__10_gicr_pendbaser_upper::R`](R) reader structure"]
impl crate::Readable for GicRegsRedistributorControlLpi1_10GicrPendbaserUpperSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_redistributor_control_lpi_1__10_gicr_pendbaser_upper::W`](W) writer structure"]
impl crate::Writable for GicRegsRedistributorControlLpi1_10GicrPendbaserUpperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Redistributor_control_LPI_1__10_GICR_PENDBASER_upper to value 0"]
impl crate::Resettable for GicRegsRedistributorControlLpi1_10GicrPendbaserUpperSpec {
    const RESET_VALUE: u32 = 0;
}
