#[doc = "Register `GIC_REGS_Distributor__39_GICD_ERRTESTR` reader"]
pub type R = crate::R<GicRegsDistributor_39GicdErrtestrSpec>;
#[doc = "Register `GIC_REGS_Distributor__39_GICD_ERRTESTR` writer"]
pub type W = crate::W<GicRegsDistributor_39GicdErrtestrSpec>;
#[doc = "Field `DISTRIBUTOR__39_GICD_ERRTESTR__0_1` reader - 0:0\\]
ECC_fatal"]
pub type Distributor_39GicdErrtestr_0_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__39_GICD_ERRTESTR__0_1` writer - 0:0\\]
ECC_fatal"]
pub type Distributor_39GicdErrtestr_0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISTRIBUTOR__39_GICD_ERRTESTR__1_1` reader - 1:1\\]
AXIM_err"]
pub type Distributor_39GicdErrtestr_1_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__39_GICD_ERRTESTR__1_1` writer - 1:1\\]
AXIM_err"]
pub type Distributor_39GicdErrtestr_1_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
ECC_fatal"]
    #[inline(always)]
    pub fn distributor__39_gicd_errtestr__0_1(&self) -> Distributor_39GicdErrtestr_0_1R {
        Distributor_39GicdErrtestr_0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AXIM_err"]
    #[inline(always)]
    pub fn distributor__39_gicd_errtestr__1_1(&self) -> Distributor_39GicdErrtestr_1_1R {
        Distributor_39GicdErrtestr_1_1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
ECC_fatal"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__39_gicd_errtestr__0_1(
        &mut self,
    ) -> Distributor_39GicdErrtestr_0_1W<GicRegsDistributor_39GicdErrtestrSpec> {
        Distributor_39GicdErrtestr_0_1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AXIM_err"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__39_gicd_errtestr__1_1(
        &mut self,
    ) -> Distributor_39GicdErrtestr_1_1W<GicRegsDistributor_39GicdErrtestrSpec> {
        Distributor_39GicdErrtestr_1_1W::new(self, 1)
    }
}
#[doc = "GICD_ERRTESTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_distributor__39_gicd_errtestr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_distributor__39_gicd_errtestr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsDistributor_39GicdErrtestrSpec;
impl crate::RegisterSpec for GicRegsDistributor_39GicdErrtestrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_distributor__39_gicd_errtestr::R`](R) reader structure"]
impl crate::Readable for GicRegsDistributor_39GicdErrtestrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_distributor__39_gicd_errtestr::W`](W) writer structure"]
impl crate::Writable for GicRegsDistributor_39GicdErrtestrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Distributor__39_GICD_ERRTESTR to value 0"]
impl crate::Resettable for GicRegsDistributor_39GicdErrtestrSpec {
    const RESET_VALUE: u32 = 0;
}
