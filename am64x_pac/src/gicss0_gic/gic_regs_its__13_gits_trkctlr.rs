#[doc = "Register `GIC_REGS_ITS__13_GITS_TRKCTLR` reader"]
pub type R = crate::R<GicRegsIts_13GitsTrkctlrSpec>;
#[doc = "Register `GIC_REGS_ITS__13_GITS_TRKCTLR` writer"]
pub type W = crate::W<GicRegsIts_13GitsTrkctlrSpec>;
#[doc = "Field `ITS__13_GITS_TRKCTLR__0_1` reader - 0:0\\]
Cache count reset"]
pub type Its_13GitsTrkctlr_0_1R = crate::BitReader;
#[doc = "Field `ITS__13_GITS_TRKCTLR__0_1` writer - 0:0\\]
Cache count reset"]
pub type Its_13GitsTrkctlr_0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS__13_GITS_TRKCTLR__1_1` reader - 1:1\\]
ITS track"]
pub type Its_13GitsTrkctlr_1_1R = crate::BitReader;
#[doc = "Field `ITS__13_GITS_TRKCTLR__1_1` writer - 1:1\\]
ITS track"]
pub type Its_13GitsTrkctlr_1_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Cache count reset"]
    #[inline(always)]
    pub fn its__13_gits_trkctlr__0_1(&self) -> Its_13GitsTrkctlr_0_1R {
        Its_13GitsTrkctlr_0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
ITS track"]
    #[inline(always)]
    pub fn its__13_gits_trkctlr__1_1(&self) -> Its_13GitsTrkctlr_1_1R {
        Its_13GitsTrkctlr_1_1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Cache count reset"]
    #[inline(always)]
    #[must_use]
    pub fn its__13_gits_trkctlr__0_1(
        &mut self,
    ) -> Its_13GitsTrkctlr_0_1W<GicRegsIts_13GitsTrkctlrSpec> {
        Its_13GitsTrkctlr_0_1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
ITS track"]
    #[inline(always)]
    #[must_use]
    pub fn its__13_gits_trkctlr__1_1(
        &mut self,
    ) -> Its_13GitsTrkctlr_1_1W<GicRegsIts_13GitsTrkctlrSpec> {
        Its_13GitsTrkctlr_1_1W::new(self, 1)
    }
}
#[doc = "GITS_TRKCTLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__13_gits_trkctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__13_gits_trkctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_13GitsTrkctlrSpec;
impl crate::RegisterSpec for GicRegsIts_13GitsTrkctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__13_gits_trkctlr::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_13GitsTrkctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__13_gits_trkctlr::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_13GitsTrkctlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__13_GITS_TRKCTLR to value 0"]
impl crate::Resettable for GicRegsIts_13GitsTrkctlrSpec {
    const RESET_VALUE: u32 = 0;
}
