#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__7_GICR_PROPBASER_lower` reader"]
pub type R = crate::R<GicRegsRedistributorControlLpi1_7GicrPropbaserLowerSpec>;
#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__7_GICR_PROPBASER_lower` writer"]
pub type W = crate::W<GicRegsRedistributorControlLpi1_7GicrPropbaserLowerSpec>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__7_GICR_PROPBASER_LOWER__0_5` reader - 4:0\\]
Idbits"]
pub type RedistributorControlLpi1_7GicrPropbaserLower_0_5R = crate::FieldReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__7_GICR_PROPBASER_LOWER__0_5` writer - 4:0\\]
Idbits"]
pub type RedistributorControlLpi1_7GicrPropbaserLower_0_5W<'a, REG> =
    crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__7_GICR_PROPBASER_LOWER__7_3` reader - 9:7\\]
Cacheability"]
pub type RedistributorControlLpi1_7GicrPropbaserLower_7_3R = crate::FieldReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__7_GICR_PROPBASER_LOWER__7_3` writer - 9:7\\]
Cacheability"]
pub type RedistributorControlLpi1_7GicrPropbaserLower_7_3W<'a, REG> =
    crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__7_GICR_PROPBASER_LOWER__12_20` reader - 31:12\\]
Physical Address \\[31:12\\]"]
pub type RedistributorControlLpi1_7GicrPropbaserLower_12_20R = crate::FieldReader<u32>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__7_GICR_PROPBASER_LOWER__12_20` writer - 31:12\\]
Physical Address \\[31:12\\]"]
pub type RedistributorControlLpi1_7GicrPropbaserLower_12_20W<'a, REG> =
    crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Idbits"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__7_gicr_propbaser_lower__0_5(
        &self,
    ) -> RedistributorControlLpi1_7GicrPropbaserLower_0_5R {
        RedistributorControlLpi1_7GicrPropbaserLower_0_5R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 7:9 - 9:7\\]
Cacheability"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__7_gicr_propbaser_lower__7_3(
        &self,
    ) -> RedistributorControlLpi1_7GicrPropbaserLower_7_3R {
        RedistributorControlLpi1_7GicrPropbaserLower_7_3R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Physical Address \\[31:12\\]"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__7_gicr_propbaser_lower__12_20(
        &self,
    ) -> RedistributorControlLpi1_7GicrPropbaserLower_12_20R {
        RedistributorControlLpi1_7GicrPropbaserLower_12_20R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Idbits"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__7_gicr_propbaser_lower__0_5(
        &mut self,
    ) -> RedistributorControlLpi1_7GicrPropbaserLower_0_5W<
        GicRegsRedistributorControlLpi1_7GicrPropbaserLowerSpec,
    > {
        RedistributorControlLpi1_7GicrPropbaserLower_0_5W::new(self, 0)
    }
    #[doc = "Bits 7:9 - 9:7\\]
Cacheability"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__7_gicr_propbaser_lower__7_3(
        &mut self,
    ) -> RedistributorControlLpi1_7GicrPropbaserLower_7_3W<
        GicRegsRedistributorControlLpi1_7GicrPropbaserLowerSpec,
    > {
        RedistributorControlLpi1_7GicrPropbaserLower_7_3W::new(self, 7)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Physical Address \\[31:12\\]"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__7_gicr_propbaser_lower__12_20(
        &mut self,
    ) -> RedistributorControlLpi1_7GicrPropbaserLower_12_20W<
        GicRegsRedistributorControlLpi1_7GicrPropbaserLowerSpec,
    > {
        RedistributorControlLpi1_7GicrPropbaserLower_12_20W::new(self, 12)
    }
}
#[doc = "GICR_PROPBASER_lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_redistributor_control_lpi_1__7_gicr_propbaser_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_redistributor_control_lpi_1__7_gicr_propbaser_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsRedistributorControlLpi1_7GicrPropbaserLowerSpec;
impl crate::RegisterSpec for GicRegsRedistributorControlLpi1_7GicrPropbaserLowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_redistributor_control_lpi_1__7_gicr_propbaser_lower::R`](R) reader structure"]
impl crate::Readable for GicRegsRedistributorControlLpi1_7GicrPropbaserLowerSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_redistributor_control_lpi_1__7_gicr_propbaser_lower::W`](W) writer structure"]
impl crate::Writable for GicRegsRedistributorControlLpi1_7GicrPropbaserLowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Redistributor_control_LPI_1__7_GICR_PROPBASER_lower to value 0"]
impl crate::Resettable for GicRegsRedistributorControlLpi1_7GicrPropbaserLowerSpec {
    const RESET_VALUE: u32 = 0;
}
