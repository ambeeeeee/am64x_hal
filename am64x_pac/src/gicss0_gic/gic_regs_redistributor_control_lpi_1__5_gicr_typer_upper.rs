#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__5_GICR_TYPER_upper` reader"]
pub type R = crate::R<GicRegsRedistributorControlLpi1_5GicrTyperUpperSpec>;
#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__5_GICR_TYPER_upper` writer"]
pub type W = crate::W<GicRegsRedistributorControlLpi1_5GicrTyperUpperSpec>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__5_GICR_TYPER_UPPER__0_8` reader - 7:0\\]
A0"]
pub type RedistributorControlLpi1_5GicrTyperUpper_0_8R = crate::FieldReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__5_GICR_TYPER_UPPER__0_8` writer - 7:0\\]
A0"]
pub type RedistributorControlLpi1_5GicrTyperUpper_0_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__5_GICR_TYPER_UPPER__8_8` reader - 15:8\\]
A1"]
pub type RedistributorControlLpi1_5GicrTyperUpper_8_8R = crate::FieldReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__5_GICR_TYPER_UPPER__8_8` writer - 15:8\\]
A1"]
pub type RedistributorControlLpi1_5GicrTyperUpper_8_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__5_GICR_TYPER_UPPER__16_8` reader - 23:16\\]
A2"]
pub type RedistributorControlLpi1_5GicrTyperUpper_16_8R = crate::FieldReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__5_GICR_TYPER_UPPER__16_8` writer - 23:16\\]
A2"]
pub type RedistributorControlLpi1_5GicrTyperUpper_16_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__5_GICR_TYPER_UPPER__24_8` reader - 31:24\\]
A3"]
pub type RedistributorControlLpi1_5GicrTyperUpper_24_8R = crate::FieldReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__5_GICR_TYPER_UPPER__24_8` writer - 31:24\\]
A3"]
pub type RedistributorControlLpi1_5GicrTyperUpper_24_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
A0"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__5_gicr_typer_upper__0_8(
        &self,
    ) -> RedistributorControlLpi1_5GicrTyperUpper_0_8R {
        RedistributorControlLpi1_5GicrTyperUpper_0_8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
A1"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__5_gicr_typer_upper__8_8(
        &self,
    ) -> RedistributorControlLpi1_5GicrTyperUpper_8_8R {
        RedistributorControlLpi1_5GicrTyperUpper_8_8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
A2"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__5_gicr_typer_upper__16_8(
        &self,
    ) -> RedistributorControlLpi1_5GicrTyperUpper_16_8R {
        RedistributorControlLpi1_5GicrTyperUpper_16_8R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
A3"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__5_gicr_typer_upper__24_8(
        &self,
    ) -> RedistributorControlLpi1_5GicrTyperUpper_24_8R {
        RedistributorControlLpi1_5GicrTyperUpper_24_8R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
A0"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__5_gicr_typer_upper__0_8(
        &mut self,
    ) -> RedistributorControlLpi1_5GicrTyperUpper_0_8W<
        GicRegsRedistributorControlLpi1_5GicrTyperUpperSpec,
    > {
        RedistributorControlLpi1_5GicrTyperUpper_0_8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
A1"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__5_gicr_typer_upper__8_8(
        &mut self,
    ) -> RedistributorControlLpi1_5GicrTyperUpper_8_8W<
        GicRegsRedistributorControlLpi1_5GicrTyperUpperSpec,
    > {
        RedistributorControlLpi1_5GicrTyperUpper_8_8W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
A2"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__5_gicr_typer_upper__16_8(
        &mut self,
    ) -> RedistributorControlLpi1_5GicrTyperUpper_16_8W<
        GicRegsRedistributorControlLpi1_5GicrTyperUpperSpec,
    > {
        RedistributorControlLpi1_5GicrTyperUpper_16_8W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
A3"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__5_gicr_typer_upper__24_8(
        &mut self,
    ) -> RedistributorControlLpi1_5GicrTyperUpper_24_8W<
        GicRegsRedistributorControlLpi1_5GicrTyperUpperSpec,
    > {
        RedistributorControlLpi1_5GicrTyperUpper_24_8W::new(self, 24)
    }
}
#[doc = "GICR_TYPER_upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_redistributor_control_lpi_1__5_gicr_typer_upper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_redistributor_control_lpi_1__5_gicr_typer_upper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsRedistributorControlLpi1_5GicrTyperUpperSpec;
impl crate::RegisterSpec for GicRegsRedistributorControlLpi1_5GicrTyperUpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_redistributor_control_lpi_1__5_gicr_typer_upper::R`](R) reader structure"]
impl crate::Readable for GicRegsRedistributorControlLpi1_5GicrTyperUpperSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_redistributor_control_lpi_1__5_gicr_typer_upper::W`](W) writer structure"]
impl crate::Writable for GicRegsRedistributorControlLpi1_5GicrTyperUpperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Redistributor_control_LPI_1__5_GICR_TYPER_upper to value 0"]
impl crate::Resettable for GicRegsRedistributorControlLpi1_5GicrTyperUpperSpec {
    const RESET_VALUE: u32 = 0;
}
