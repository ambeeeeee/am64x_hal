#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__8_GICR_PROPBASER_upper` reader"]
pub type R = crate::R<GicRegsRedistributorControlLpi1_8GicrPropbaserUpperSpec>;
#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__8_GICR_PROPBASER_upper` writer"]
pub type W = crate::W<GicRegsRedistributorControlLpi1_8GicrPropbaserUpperSpec>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__8_GICR_PROPBASER_UPPER__0_16` reader - 15:0\\]
Physical Address \\[47:32\\]"]
pub type RedistributorControlLpi1_8GicrPropbaserUpper_0_16R = crate::FieldReader<u16>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__8_GICR_PROPBASER_UPPER__0_16` writer - 15:0\\]
Physical Address \\[47:32\\]"]
pub type RedistributorControlLpi1_8GicrPropbaserUpper_0_16W<'a, REG> =
    crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Physical Address \\[47:32\\]"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__8_gicr_propbaser_upper__0_16(
        &self,
    ) -> RedistributorControlLpi1_8GicrPropbaserUpper_0_16R {
        RedistributorControlLpi1_8GicrPropbaserUpper_0_16R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Physical Address \\[47:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__8_gicr_propbaser_upper__0_16(
        &mut self,
    ) -> RedistributorControlLpi1_8GicrPropbaserUpper_0_16W<
        GicRegsRedistributorControlLpi1_8GicrPropbaserUpperSpec,
    > {
        RedistributorControlLpi1_8GicrPropbaserUpper_0_16W::new(self, 0)
    }
}
#[doc = "GICR_PROPBASER_upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_redistributor_control_lpi_1__8_gicr_propbaser_upper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_redistributor_control_lpi_1__8_gicr_propbaser_upper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsRedistributorControlLpi1_8GicrPropbaserUpperSpec;
impl crate::RegisterSpec for GicRegsRedistributorControlLpi1_8GicrPropbaserUpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_redistributor_control_lpi_1__8_gicr_propbaser_upper::R`](R) reader structure"]
impl crate::Readable for GicRegsRedistributorControlLpi1_8GicrPropbaserUpperSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_redistributor_control_lpi_1__8_gicr_propbaser_upper::W`](W) writer structure"]
impl crate::Writable for GicRegsRedistributorControlLpi1_8GicrPropbaserUpperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Redistributor_control_LPI_1__8_GICR_PROPBASER_upper to value 0"]
impl crate::Resettable for GicRegsRedistributorControlLpi1_8GicrPropbaserUpperSpec {
    const RESET_VALUE: u32 = 0;
}
