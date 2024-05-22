#[doc = "Register `GIC_REGS_Distributor__1_GICD_CTLR` reader"]
pub type R = crate::R<GicRegsDistributor_1GicdCtlrSpec>;
#[doc = "Register `GIC_REGS_Distributor__1_GICD_CTLR` writer"]
pub type W = crate::W<GicRegsDistributor_1GicdCtlrSpec>;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__0_1` reader - 0:0\\]
NS: EnableGrp1 S: EnableGrp0"]
pub type Distributor_1GicdCtlr_0_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__0_1` writer - 0:0\\]
NS: EnableGrp1 S: EnableGrp0"]
pub type Distributor_1GicdCtlr_0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__1_1` reader - 1:1\\]
NS: EnableGrp1A S: EnableGrp1_NS"]
pub type Distributor_1GicdCtlr_1_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__1_1` writer - 1:1\\]
NS: EnableGrp1A S: EnableGrp1_NS"]
pub type Distributor_1GicdCtlr_1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__2_1` reader - 2:2\\]
S: EnableGrp1_S"]
pub type Distributor_1GicdCtlr_2_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__2_1` writer - 2:2\\]
S: EnableGrp1_S"]
pub type Distributor_1GicdCtlr_2_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__4_1` reader - 4:4\\]
NS: ARE_NS S: ARE_S"]
pub type Distributor_1GicdCtlr_4_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__4_1` writer - 4:4\\]
NS: ARE_NS S: ARE_S"]
pub type Distributor_1GicdCtlr_4_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__5_1` reader - 5:5\\]
S: ARE_NS"]
pub type Distributor_1GicdCtlr_5_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__5_1` writer - 5:5\\]
S: ARE_NS"]
pub type Distributor_1GicdCtlr_5_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__6_1` reader - 6:6\\]
S: DS"]
pub type Distributor_1GicdCtlr_6_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__6_1` writer - 6:6\\]
S: DS"]
pub type Distributor_1GicdCtlr_6_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__31_1` reader - 31:31\\]
Register Write Pending"]
pub type Distributor_1GicdCtlr_31_1R = crate::BitReader;
#[doc = "Field `DISTRIBUTOR__1_GICD_CTLR__31_1` writer - 31:31\\]
Register Write Pending"]
pub type Distributor_1GicdCtlr_31_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
NS: EnableGrp1 S: EnableGrp0"]
    #[inline(always)]
    pub fn distributor__1_gicd_ctlr__0_1(&self) -> Distributor_1GicdCtlr_0_1R {
        Distributor_1GicdCtlr_0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
NS: EnableGrp1A S: EnableGrp1_NS"]
    #[inline(always)]
    pub fn distributor__1_gicd_ctlr__1_1(&self) -> Distributor_1GicdCtlr_1_1R {
        Distributor_1GicdCtlr_1_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
S: EnableGrp1_S"]
    #[inline(always)]
    pub fn distributor__1_gicd_ctlr__2_1(&self) -> Distributor_1GicdCtlr_2_1R {
        Distributor_1GicdCtlr_2_1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
NS: ARE_NS S: ARE_S"]
    #[inline(always)]
    pub fn distributor__1_gicd_ctlr__4_1(&self) -> Distributor_1GicdCtlr_4_1R {
        Distributor_1GicdCtlr_4_1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
S: ARE_NS"]
    #[inline(always)]
    pub fn distributor__1_gicd_ctlr__5_1(&self) -> Distributor_1GicdCtlr_5_1R {
        Distributor_1GicdCtlr_5_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
S: DS"]
    #[inline(always)]
    pub fn distributor__1_gicd_ctlr__6_1(&self) -> Distributor_1GicdCtlr_6_1R {
        Distributor_1GicdCtlr_6_1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Register Write Pending"]
    #[inline(always)]
    pub fn distributor__1_gicd_ctlr__31_1(&self) -> Distributor_1GicdCtlr_31_1R {
        Distributor_1GicdCtlr_31_1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
NS: EnableGrp1 S: EnableGrp0"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__1_gicd_ctlr__0_1(
        &mut self,
    ) -> Distributor_1GicdCtlr_0_1W<GicRegsDistributor_1GicdCtlrSpec> {
        Distributor_1GicdCtlr_0_1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
NS: EnableGrp1A S: EnableGrp1_NS"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__1_gicd_ctlr__1_1(
        &mut self,
    ) -> Distributor_1GicdCtlr_1_1W<GicRegsDistributor_1GicdCtlrSpec> {
        Distributor_1GicdCtlr_1_1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
S: EnableGrp1_S"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__1_gicd_ctlr__2_1(
        &mut self,
    ) -> Distributor_1GicdCtlr_2_1W<GicRegsDistributor_1GicdCtlrSpec> {
        Distributor_1GicdCtlr_2_1W::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
NS: ARE_NS S: ARE_S"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__1_gicd_ctlr__4_1(
        &mut self,
    ) -> Distributor_1GicdCtlr_4_1W<GicRegsDistributor_1GicdCtlrSpec> {
        Distributor_1GicdCtlr_4_1W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
S: ARE_NS"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__1_gicd_ctlr__5_1(
        &mut self,
    ) -> Distributor_1GicdCtlr_5_1W<GicRegsDistributor_1GicdCtlrSpec> {
        Distributor_1GicdCtlr_5_1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
S: DS"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__1_gicd_ctlr__6_1(
        &mut self,
    ) -> Distributor_1GicdCtlr_6_1W<GicRegsDistributor_1GicdCtlrSpec> {
        Distributor_1GicdCtlr_6_1W::new(self, 6)
    }
    #[doc = "Bit 31 - 31:31\\]
Register Write Pending"]
    #[inline(always)]
    #[must_use]
    pub fn distributor__1_gicd_ctlr__31_1(
        &mut self,
    ) -> Distributor_1GicdCtlr_31_1W<GicRegsDistributor_1GicdCtlrSpec> {
        Distributor_1GicdCtlr_31_1W::new(self, 31)
    }
}
#[doc = "GICD_CTLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_distributor__1_gicd_ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_distributor__1_gicd_ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsDistributor_1GicdCtlrSpec;
impl crate::RegisterSpec for GicRegsDistributor_1GicdCtlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_distributor__1_gicd_ctlr::R`](R) reader structure"]
impl crate::Readable for GicRegsDistributor_1GicdCtlrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_distributor__1_gicd_ctlr::W`](W) writer structure"]
impl crate::Writable for GicRegsDistributor_1GicdCtlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Distributor__1_GICD_CTLR to value 0x30"]
impl crate::Resettable for GicRegsDistributor_1GicdCtlrSpec {
    const RESET_VALUE: u32 = 0x30;
}
