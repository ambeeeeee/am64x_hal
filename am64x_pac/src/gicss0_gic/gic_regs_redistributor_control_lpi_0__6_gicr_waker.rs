#[doc = "Register `GIC_REGS_Redistributor_control_LPI_0__6_GICR_WAKER` reader"]
pub type R = crate::R<GicRegsRedistributorControlLpi0_6GicrWakerSpec>;
#[doc = "Register `GIC_REGS_Redistributor_control_LPI_0__6_GICR_WAKER` writer"]
pub type W = crate::W<GicRegsRedistributorControlLpi0_6GicrWakerSpec>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__6_GICR_WAKER__0_1` reader - 0:0\\]
Sleep"]
pub type RedistributorControlLpi0_6GicrWaker_0_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__6_GICR_WAKER__0_1` writer - 0:0\\]
Sleep"]
pub type RedistributorControlLpi0_6GicrWaker_0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__6_GICR_WAKER__1_1` reader - 1:1\\]
ProcessorSleep"]
pub type RedistributorControlLpi0_6GicrWaker_1_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__6_GICR_WAKER__1_1` writer - 1:1\\]
ProcessorSleep"]
pub type RedistributorControlLpi0_6GicrWaker_1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__6_GICR_WAKER__2_1` reader - 2:2\\]
ChildrenAsleep"]
pub type RedistributorControlLpi0_6GicrWaker_2_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__6_GICR_WAKER__2_1` writer - 2:2\\]
ChildrenAsleep"]
pub type RedistributorControlLpi0_6GicrWaker_2_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__6_GICR_WAKER__31_1` reader - 31:31\\]
Quiescent"]
pub type RedistributorControlLpi0_6GicrWaker_31_1R = crate::BitReader;
#[doc = "Field `REDISTRIBUTOR_CONTROL_LPI_0__6_GICR_WAKER__31_1` writer - 31:31\\]
Quiescent"]
pub type RedistributorControlLpi0_6GicrWaker_31_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sleep"]
    #[inline(always)]
    pub fn redistributor_control_lpi_0__6_gicr_waker__0_1(
        &self,
    ) -> RedistributorControlLpi0_6GicrWaker_0_1R {
        RedistributorControlLpi0_6GicrWaker_0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
ProcessorSleep"]
    #[inline(always)]
    pub fn redistributor_control_lpi_0__6_gicr_waker__1_1(
        &self,
    ) -> RedistributorControlLpi0_6GicrWaker_1_1R {
        RedistributorControlLpi0_6GicrWaker_1_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
ChildrenAsleep"]
    #[inline(always)]
    pub fn redistributor_control_lpi_0__6_gicr_waker__2_1(
        &self,
    ) -> RedistributorControlLpi0_6GicrWaker_2_1R {
        RedistributorControlLpi0_6GicrWaker_2_1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Quiescent"]
    #[inline(always)]
    pub fn redistributor_control_lpi_0__6_gicr_waker__31_1(
        &self,
    ) -> RedistributorControlLpi0_6GicrWaker_31_1R {
        RedistributorControlLpi0_6GicrWaker_31_1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_0__6_gicr_waker__0_1(
        &mut self,
    ) -> RedistributorControlLpi0_6GicrWaker_0_1W<GicRegsRedistributorControlLpi0_6GicrWakerSpec>
    {
        RedistributorControlLpi0_6GicrWaker_0_1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
ProcessorSleep"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_0__6_gicr_waker__1_1(
        &mut self,
    ) -> RedistributorControlLpi0_6GicrWaker_1_1W<GicRegsRedistributorControlLpi0_6GicrWakerSpec>
    {
        RedistributorControlLpi0_6GicrWaker_1_1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
ChildrenAsleep"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_0__6_gicr_waker__2_1(
        &mut self,
    ) -> RedistributorControlLpi0_6GicrWaker_2_1W<GicRegsRedistributorControlLpi0_6GicrWakerSpec>
    {
        RedistributorControlLpi0_6GicrWaker_2_1W::new(self, 2)
    }
    #[doc = "Bit 31 - 31:31\\]
Quiescent"]
    #[inline(always)]
    #[must_use]
    pub fn redistributor_control_lpi_0__6_gicr_waker__31_1(
        &mut self,
    ) -> RedistributorControlLpi0_6GicrWaker_31_1W<GicRegsRedistributorControlLpi0_6GicrWakerSpec>
    {
        RedistributorControlLpi0_6GicrWaker_31_1W::new(self, 31)
    }
}
#[doc = "GICR_WAKER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_redistributor_control_lpi_0__6_gicr_waker::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_redistributor_control_lpi_0__6_gicr_waker::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsRedistributorControlLpi0_6GicrWakerSpec;
impl crate::RegisterSpec for GicRegsRedistributorControlLpi0_6GicrWakerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_redistributor_control_lpi_0__6_gicr_waker::R`](R) reader structure"]
impl crate::Readable for GicRegsRedistributorControlLpi0_6GicrWakerSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_redistributor_control_lpi_0__6_gicr_waker::W`](W) writer structure"]
impl crate::Writable for GicRegsRedistributorControlLpi0_6GicrWakerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Redistributor_control_LPI_0__6_GICR_WAKER to value 0x06"]
impl crate::Resettable for GicRegsRedistributorControlLpi0_6GicrWakerSpec {
    const RESET_VALUE: u32 = 0x06;
}
