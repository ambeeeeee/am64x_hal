#[doc = "Register `GIC_REGS_Distributor__38_GICD_ESTATUSR` reader"]
pub type R = crate::R<GicRegsDistributor_38GicdEstatusrSpec>;
#[doc = "Register `GIC_REGS_Distributor__38_GICD_ESTATUSR` writer"]
pub type W = crate::W<GicRegsDistributor_38GicdEstatusrSpec>;
#[doc = "Field `DISTRIBUTOR__38_GICD_ESTATUSR__31_1` reader - 31:31\\]
SWRP"]
pub type Distributor_38GicdEstatusr_31_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__38_GICD_ESTATUSR__31_1` writer - 31:31\\]
SWRP"]
pub type Distributor_38GicdEstatusr_31_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - 31:31\\]
SWRP"]
    #[inline(always)]
    pub fn distributor__38_gicd_estatusr__31_1(&self) -> Distributor_38GicdEstatusr_31_1R {
        Distributor_38GicdEstatusr_31_1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
SWRP"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__38_gicd_estatusr__31_1(
        &mut self,
    ) -> Distributor_38GicdEstatusr_31_1W<GicRegsDistributor_38GicdEstatusrSpec> {
        Distributor_38GicdEstatusr_31_1W::new(self, 31)
    }
}
#[doc = "GICD_ESTATUSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_distributor__38_gicd_estatusr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_distributor__38_gicd_estatusr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsDistributor_38GicdEstatusrSpec;
impl crate::RegisterSpec for GicRegsDistributor_38GicdEstatusrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_distributor__38_gicd_estatusr::R`](R) reader structure"]
impl crate::Readable for GicRegsDistributor_38GicdEstatusrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_distributor__38_gicd_estatusr::W`](W) writer structure"]
impl crate::Writable for GicRegsDistributor_38GicdEstatusrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Distributor__38_GICD_ESTATUSR to value 0"]
impl crate::Resettable for GicRegsDistributor_38GicdEstatusrSpec {
    const RESET_VALUE: u32 = 0;
}
