#[doc = "Register `GIC_REGS_Distributor__37_GICD_IROUTER223_lower` reader"]
pub type R = crate::R<GicRegsDistributor_37GicdIrouter223LowerSpec>;
#[doc = "Register `GIC_REGS_Distributor__37_GICD_IROUTER223_lower` writer"]
pub type W = crate::W<GicRegsDistributor_37GicdIrouter223LowerSpec>;
#[doc = "Field `DISTRIBUTOR__37_GICD_IROUTER223_LOWER__0_8` reader - 7:0\\]
A0"]
pub type Distributor_37GicdIrouter223Lower_0_8R = crate::FieldReader;
#[doc = "Field `DISTRIBUTOR__37_GICD_IROUTER223_LOWER__0_8` writer - 7:0\\]
A0"]
pub type Distributor_37GicdIrouter223Lower_0_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DISTRIBUTOR__37_GICD_IROUTER223_LOWER__8_8` reader - 15:8\\]
A1"]
pub type Distributor_37GicdIrouter223Lower_8_8R = crate::FieldReader;
#[doc = "Field `DISTRIBUTOR__37_GICD_IROUTER223_LOWER__8_8` writer - 15:8\\]
A1"]
pub type Distributor_37GicdIrouter223Lower_8_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DISTRIBUTOR__37_GICD_IROUTER223_LOWER__31_1` reader - 31:31\\]
IRM"]
pub type Distributor_37GicdIrouter223Lower_31_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__37_GICD_IROUTER223_LOWER__31_1` writer - 31:31\\]
IRM"]
pub type Distributor_37GicdIrouter223Lower_31_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
A0"]
    #[inline(always)]
    pub fn distributor__37_gicd_irouter223_lower__0_8(
        &self,
    ) -> Distributor_37GicdIrouter223Lower_0_8R {
        Distributor_37GicdIrouter223Lower_0_8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
A1"]
    #[inline(always)]
    pub fn distributor__37_gicd_irouter223_lower__8_8(
        &self,
    ) -> Distributor_37GicdIrouter223Lower_8_8R {
        Distributor_37GicdIrouter223Lower_8_8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
IRM"]
    #[inline(always)]
    pub fn distributor__37_gicd_irouter223_lower__31_1(
        &self,
    ) -> Distributor_37GicdIrouter223Lower_31_1R {
        Distributor_37GicdIrouter223Lower_31_1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
A0"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__37_gicd_irouter223_lower__0_8(
        &mut self,
    ) -> Distributor_37GicdIrouter223Lower_0_8W<GicRegsDistributor_37GicdIrouter223LowerSpec> {
        Distributor_37GicdIrouter223Lower_0_8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
A1"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__37_gicd_irouter223_lower__8_8(
        &mut self,
    ) -> Distributor_37GicdIrouter223Lower_8_8W<GicRegsDistributor_37GicdIrouter223LowerSpec> {
        Distributor_37GicdIrouter223Lower_8_8W::new(self, 8)
    }
    #[doc = "Bit 31 - 31:31\\]
IRM"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__37_gicd_irouter223_lower__31_1(
        &mut self,
    ) -> Distributor_37GicdIrouter223Lower_31_1W<GicRegsDistributor_37GicdIrouter223LowerSpec> {
        Distributor_37GicdIrouter223Lower_31_1W::new(self, 31)
    }
}
#[doc = "GICD_IROUTER223_lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_distributor__37_gicd_irouter223_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_distributor__37_gicd_irouter223_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsDistributor_37GicdIrouter223LowerSpec;
impl crate::RegisterSpec for GicRegsDistributor_37GicdIrouter223LowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_distributor__37_gicd_irouter223_lower::R`](R) reader structure"]
impl crate::Readable for GicRegsDistributor_37GicdIrouter223LowerSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_distributor__37_gicd_irouter223_lower::W`](W) writer structure"]
impl crate::Writable for GicRegsDistributor_37GicdIrouter223LowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Distributor__37_GICD_IROUTER223_lower to value 0"]
impl crate::Resettable for GicRegsDistributor_37GicdIrouter223LowerSpec {
    const RESET_VALUE: u32 = 0;
}
