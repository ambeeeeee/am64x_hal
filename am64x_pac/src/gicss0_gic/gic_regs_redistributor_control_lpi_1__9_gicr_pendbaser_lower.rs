#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__9_GICR_PENDBASER_lower` reader"]
pub type R = crate::R<GicRegsRedistributorControlLpi1_9GicrPendbaserLowerSpec>;
#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__9_GICR_PENDBASER_lower` writer"]
pub type W = crate::W<GicRegsRedistributorControlLpi1_9GicrPendbaserLowerSpec>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__9_GICR_PENDBASER_LOWER__7_3` reader - 9:7\\]
Cacheability"]
pub type RedistributorControlLpi1_9GicrPendbaserLower_7_3R = crate::FieldReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__9_GICR_PENDBASER_LOWER__7_3` writer - 9:7\\]
Cacheability"]
pub type RedistributorControlLpi1_9GicrPendbaserLower_7_3W<'a, REG> =
    crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__9_GICR_PENDBASER_LOWER__16_16` reader - 31:16\\]
Physical Address \\[31:16\\]"]
pub type RedistributorControlLpi1_9GicrPendbaserLower_16_16R = crate::FieldReader<u16>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__9_GICR_PENDBASER_LOWER__16_16` writer - 31:16\\]
Physical Address \\[31:16\\]"]
pub type RedistributorControlLpi1_9GicrPendbaserLower_16_16W<'a, REG> =
    crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 7:9 - 9:7\\]
Cacheability"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__9_gicr_pendbaser_lower__7_3(
        &self,
    ) -> RedistributorControlLpi1_9GicrPendbaserLower_7_3R {
        RedistributorControlLpi1_9GicrPendbaserLower_7_3R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Physical Address \\[31:16\\]"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__9_gicr_pendbaser_lower__16_16(
        &self,
    ) -> RedistributorControlLpi1_9GicrPendbaserLower_16_16R {
        RedistributorControlLpi1_9GicrPendbaserLower_16_16R::new(
            ((self.bits >> 16) & 0xffff) as u16,
        )
    }
}
impl W {
    #[doc = "Bits 7:9 - 9:7\\]
Cacheability"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__9_gicr_pendbaser_lower__7_3(
        &mut self,
    ) -> RedistributorControlLpi1_9GicrPendbaserLower_7_3W<
        GicRegsRedistributorControlLpi1_9GicrPendbaserLowerSpec,
    > {
        RedistributorControlLpi1_9GicrPendbaserLower_7_3W::new(self, 7)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Physical Address \\[31:16\\]"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__9_gicr_pendbaser_lower__16_16(
        &mut self,
    ) -> RedistributorControlLpi1_9GicrPendbaserLower_16_16W<
        GicRegsRedistributorControlLpi1_9GicrPendbaserLowerSpec,
    > {
        RedistributorControlLpi1_9GicrPendbaserLower_16_16W::new(self, 16)
    }
}
#[doc = "GICR_PENDBASER_lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_redistributor_control_lpi_1__9_gicr_pendbaser_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_redistributor_control_lpi_1__9_gicr_pendbaser_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsRedistributorControlLpi1_9GicrPendbaserLowerSpec;
impl crate::RegisterSpec for GicRegsRedistributorControlLpi1_9GicrPendbaserLowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_redistributor_control_lpi_1__9_gicr_pendbaser_lower::R`](R) reader structure"]
impl crate::Readable for GicRegsRedistributorControlLpi1_9GicrPendbaserLowerSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_redistributor_control_lpi_1__9_gicr_pendbaser_lower::W`](W) writer structure"]
impl crate::Writable for GicRegsRedistributorControlLpi1_9GicrPendbaserLowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Redistributor_control_LPI_1__9_GICR_PENDBASER_lower to value 0"]
impl crate::Resettable for GicRegsRedistributorControlLpi1_9GicrPendbaserLowerSpec {
    const RESET_VALUE: u32 = 0;
}
