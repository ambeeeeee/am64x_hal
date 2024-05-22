#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__4_GICR_TYPER_lower` reader"]
pub type R = crate::R<GicRegsRedistributorControlLpi1_4GicrTyperLowerSpec>;
#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__4_GICR_TYPER_lower` writer"]
pub type W = crate::W<GicRegsRedistributorControlLpi1_4GicrTyperLowerSpec>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__4_GICR_TYPER_LOWER__0_1` reader - 0:0\\]
PLPIS"]
pub type RedistributorControlLpi1_4GicrTyperLower_0_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__4_GICR_TYPER_LOWER__0_1` writer - 0:0\\]
PLPIS"]
pub type RedistributorControlLpi1_4GicrTyperLower_0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__4_GICR_TYPER_LOWER__1_1` reader - 1:1\\]
VLPIS"]
pub type RedistributorControlLpi1_4GicrTyperLower_1_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__4_GICR_TYPER_LOWER__1_1` writer - 1:1\\]
VLPIS"]
pub type RedistributorControlLpi1_4GicrTyperLower_1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__4_GICR_TYPER_LOWER__3_1` reader - 3:3\\]
Distributed"]
pub type RedistributorControlLpi1_4GicrTyperLower_3_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__4_GICR_TYPER_LOWER__3_1` writer - 3:3\\]
Distributed"]
pub type RedistributorControlLpi1_4GicrTyperLower_3_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__4_GICR_TYPER_LOWER__4_1` reader - 4:4\\]
Last"]
pub type RedistributorControlLpi1_4GicrTyperLower_4_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__4_GICR_TYPER_LOWER__4_1` writer - 4:4\\]
Last"]
pub type RedistributorControlLpi1_4GicrTyperLower_4_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__4_GICR_TYPER_LOWER__8_16` reader - 23:8\\]
Processor Number"]
pub type RedistributorControlLpi1_4GicrTyperLower_8_16R = crate::FieldReader<u16>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__4_GICR_TYPER_LOWER__8_16` writer - 23:8\\]
Processor Number"]
pub type RedistributorControlLpi1_4GicrTyperLower_8_16W<'a, REG> =
    crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
PLPIS"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__4_gicr_typer_lower__0_1(
        &self,
    ) -> RedistributorControlLpi1_4GicrTyperLower_0_1R {
        RedistributorControlLpi1_4GicrTyperLower_0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
VLPIS"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__4_gicr_typer_lower__1_1(
        &self,
    ) -> RedistributorControlLpi1_4GicrTyperLower_1_1R {
        RedistributorControlLpi1_4GicrTyperLower_1_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Distributed"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__4_gicr_typer_lower__3_1(
        &self,
    ) -> RedistributorControlLpi1_4GicrTyperLower_3_1R {
        RedistributorControlLpi1_4GicrTyperLower_3_1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Last"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__4_gicr_typer_lower__4_1(
        &self,
    ) -> RedistributorControlLpi1_4GicrTyperLower_4_1R {
        RedistributorControlLpi1_4GicrTyperLower_4_1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Processor Number"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__4_gicr_typer_lower__8_16(
        &self,
    ) -> RedistributorControlLpi1_4GicrTyperLower_8_16R {
        RedistributorControlLpi1_4GicrTyperLower_8_16R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
PLPIS"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__4_gicr_typer_lower__0_1(
        &mut self,
    ) -> RedistributorControlLpi1_4GicrTyperLower_0_1W<
        GicRegsRedistributorControlLpi1_4GicrTyperLowerSpec,
    > {
        RedistributorControlLpi1_4GicrTyperLower_0_1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
VLPIS"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__4_gicr_typer_lower__1_1(
        &mut self,
    ) -> RedistributorControlLpi1_4GicrTyperLower_1_1W<
        GicRegsRedistributorControlLpi1_4GicrTyperLowerSpec,
    > {
        RedistributorControlLpi1_4GicrTyperLower_1_1W::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
Distributed"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__4_gicr_typer_lower__3_1(
        &mut self,
    ) -> RedistributorControlLpi1_4GicrTyperLower_3_1W<
        GicRegsRedistributorControlLpi1_4GicrTyperLowerSpec,
    > {
        RedistributorControlLpi1_4GicrTyperLower_3_1W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Last"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__4_gicr_typer_lower__4_1(
        &mut self,
    ) -> RedistributorControlLpi1_4GicrTyperLower_4_1W<
        GicRegsRedistributorControlLpi1_4GicrTyperLowerSpec,
    > {
        RedistributorControlLpi1_4GicrTyperLower_4_1W::new(self, 4)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Processor Number"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__4_gicr_typer_lower__8_16(
        &mut self,
    ) -> RedistributorControlLpi1_4GicrTyperLower_8_16W<
        GicRegsRedistributorControlLpi1_4GicrTyperLowerSpec,
    > {
        RedistributorControlLpi1_4GicrTyperLower_8_16W::new(self, 8)
    }
}
#[doc = "GICR_TYPER_lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_redistributor_control_lpi_1__4_gicr_typer_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_redistributor_control_lpi_1__4_gicr_typer_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsRedistributorControlLpi1_4GicrTyperLowerSpec;
impl crate::RegisterSpec for GicRegsRedistributorControlLpi1_4GicrTyperLowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_redistributor_control_lpi_1__4_gicr_typer_lower::R`](R) reader structure"]
impl crate::Readable for GicRegsRedistributorControlLpi1_4GicrTyperLowerSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_redistributor_control_lpi_1__4_gicr_typer_lower::W`](W) writer structure"]
impl crate::Writable for GicRegsRedistributorControlLpi1_4GicrTyperLowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Redistributor_control_LPI_1__4_GICR_TYPER_lower to value 0x01"]
impl crate::Resettable for GicRegsRedistributorControlLpi1_4GicrTyperLowerSpec {
    const RESET_VALUE: u32 = 0x01;
}
