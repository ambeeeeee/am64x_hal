#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__3_GICR_IIDR` reader"]
pub type R = crate::R<GicRegsRedistributorControlLpi1_3GicrIidrSpec>;
#[doc = "Register `GIC_REGS_Redistributor_control_LPI_1__3_GICR_IIDR` writer"]
pub type W = crate::W<GicRegsRedistributorControlLpi1_3GicrIidrSpec>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__3_GICR_IIDR__0_12` reader - 11:0\\]
Implementer"]
pub type RedistributorControlLpi1_3GicrIidr_0_12R = crate::FieldReader<u16>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__3_GICR_IIDR__0_12` writer - 11:0\\]
Implementer"]
pub type RedistributorControlLpi1_3GicrIidr_0_12W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__3_GICR_IIDR__12_4` reader - 15:12\\]
Revision"]
pub type RedistributorControlLpi1_3GicrIidr_12_4R = crate::FieldReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__3_GICR_IIDR__12_4` writer - 15:12\\]
Revision"]
pub type RedistributorControlLpi1_3GicrIidr_12_4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__3_GICR_IIDR__16_4` reader - 19:16\\]
Variant"]
pub type RedistributorControlLpi1_3GicrIidr_16_4R = crate::FieldReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__3_GICR_IIDR__16_4` writer - 19:16\\]
Variant"]
pub type RedistributorControlLpi1_3GicrIidr_16_4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__3_GICR_IIDR__24_8` reader - 31:24\\]
ProductID"]
pub type RedistributorControlLpi1_3GicrIidr_24_8R = crate::FieldReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_1__3_GICR_IIDR__24_8` writer - 31:24\\]
ProductID"]
pub type RedistributorControlLpi1_3GicrIidr_24_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Implementer"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__3_gicr_iidr__0_12(
        &self,
    ) -> RedistributorControlLpi1_3GicrIidr_0_12R {
        RedistributorControlLpi1_3GicrIidr_0_12R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Revision"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__3_gicr_iidr__12_4(
        &self,
    ) -> RedistributorControlLpi1_3GicrIidr_12_4R {
        RedistributorControlLpi1_3GicrIidr_12_4R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Variant"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__3_gicr_iidr__16_4(
        &self,
    ) -> RedistributorControlLpi1_3GicrIidr_16_4R {
        RedistributorControlLpi1_3GicrIidr_16_4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
ProductID"]
    #[inline(always)]
    pub fn redistributor_control_lpi_1__3_gicr_iidr__24_8(
        &self,
    ) -> RedistributorControlLpi1_3GicrIidr_24_8R {
        RedistributorControlLpi1_3GicrIidr_24_8R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Implementer"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__3_gicr_iidr__0_12(
        &mut self,
    ) -> RedistributorControlLpi1_3GicrIidr_0_12W<GicRegsRedistributorControlLpi1_3GicrIidrSpec>
    {
        RedistributorControlLpi1_3GicrIidr_0_12W::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Revision"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__3_gicr_iidr__12_4(
        &mut self,
    ) -> RedistributorControlLpi1_3GicrIidr_12_4W<GicRegsRedistributorControlLpi1_3GicrIidrSpec>
    {
        RedistributorControlLpi1_3GicrIidr_12_4W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Variant"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__3_gicr_iidr__16_4(
        &mut self,
    ) -> RedistributorControlLpi1_3GicrIidr_16_4W<GicRegsRedistributorControlLpi1_3GicrIidrSpec>
    {
        RedistributorControlLpi1_3GicrIidr_16_4W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
ProductID"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_1__3_gicr_iidr__24_8(
        &mut self,
    ) -> RedistributorControlLpi1_3GicrIidr_24_8W<GicRegsRedistributorControlLpi1_3GicrIidrSpec>
    {
        RedistributorControlLpi1_3GicrIidr_24_8W::new(self, 24)
    }
}
#[doc = "GICR_IIDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_redistributor_control_lpi_1__3_gicr_iidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_redistributor_control_lpi_1__3_gicr_iidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsRedistributorControlLpi1_3GicrIidrSpec;
impl crate::RegisterSpec for GicRegsRedistributorControlLpi1_3GicrIidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_redistributor_control_lpi_1__3_gicr_iidr::R`](R) reader structure"]
impl crate::Readable for GicRegsRedistributorControlLpi1_3GicrIidrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_redistributor_control_lpi_1__3_gicr_iidr::W`](W) writer structure"]
impl crate::Writable for GicRegsRedistributorControlLpi1_3GicrIidrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Redistributor_control_LPI_1__3_GICR_IIDR to value 0x0001_1083"]
impl crate::Resettable for GicRegsRedistributorControlLpi1_3GicrIidrSpec {
    const RESET_VALUE: u32 = 0x0001_1083;
}
