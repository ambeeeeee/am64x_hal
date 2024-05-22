#[doc = "Register `GIC_REGS_ITS__1_GITS_CTLR` reader"]
pub type R = crate::R<GicRegsIts_1GitsCtlrSpec>;
#[doc = "Register `GIC_REGS_ITS__1_GITS_CTLR` writer"]
pub type W = crate::W<GicRegsIts_1GitsCtlrSpec>;
#[doc = "Field `ITS__1_GITS_CTLR__0_1` reader - 0:0\\]
Enabled"]
pub type Its_1GitsCtlr_0_1R = crate::BitReader;
#[doc = "Field `ITS__1_GITS_CTLR__0_1` writer - 0:0\\]
Enabled"]
pub type Its_1GitsCtlr_0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS__1_GITS_CTLR__31_1` reader - 31:31\\]
Quiescent"]
pub type Its_1GitsCtlr_31_1R = crate::BitReader;
#[doc = "Field `ITS__1_GITS_CTLR__31_1` writer - 31:31\\]
Quiescent"]
pub type Its_1GitsCtlr_31_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enabled"]
    #[inline(always)]
    pub fn its__1_gits_ctlr__0_1(&self) -> Its_1GitsCtlr_0_1R {
        Its_1GitsCtlr_0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Quiescent"]
    #[inline(always)]
    pub fn its__1_gits_ctlr__31_1(&self) -> Its_1GitsCtlr_31_1R {
        Its_1GitsCtlr_31_1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn its__1_gits_ctlr__0_1(&mut self) -> Its_1GitsCtlr_0_1W<GicRegsIts_1GitsCtlrSpec> {
        Its_1GitsCtlr_0_1W::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Quiescent"]
    #[inline(always)]
    #[must_use]
    pub fn its__1_gits_ctlr__31_1(&mut self) -> Its_1GitsCtlr_31_1W<GicRegsIts_1GitsCtlrSpec> {
        Its_1GitsCtlr_31_1W::new(self, 31)
    }
}
#[doc = "GITS_CTLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__1_gits_ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__1_gits_ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_1GitsCtlrSpec;
impl crate::RegisterSpec for GicRegsIts_1GitsCtlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__1_gits_ctlr::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_1GitsCtlrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__1_gits_ctlr::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_1GitsCtlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__1_GITS_CTLR to value 0x8000_0000"]
impl crate::Resettable for GicRegsIts_1GitsCtlrSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
