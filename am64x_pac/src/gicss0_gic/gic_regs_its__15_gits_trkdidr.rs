#[doc = "Register `GIC_REGS_ITS__15_GITS_TRKDIDR` reader"]
pub type R = crate::R<GicRegsIts_15GitsTrkdidrSpec>;
#[doc = "Register `GIC_REGS_ITS__15_GITS_TRKDIDR` writer"]
pub type W = crate::W<GicRegsIts_15GitsTrkdidrSpec>;
#[doc = "Field `ITS__15_GITS_TRKDIDR__0_20` reader - 19:0\\]
DID"]
pub type Its_15GitsTrkdidr_0_20R = crate::FieldReader<u32>;
#[doc = "Field `ITS__15_GITS_TRKDIDR__0_20` writer - 19:0\\]
DID"]
pub type Its_15GitsTrkdidr_0_20W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
DID"]
    #[inline(always)]
    pub fn its__15_gits_trkdidr__0_20(&self) -> Its_15GitsTrkdidr_0_20R {
        Its_15GitsTrkdidr_0_20R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
DID"]
    #[inline(always)]
    #[must_use]
    pub fn its__15_gits_trkdidr__0_20(
        &mut self,
    ) -> Its_15GitsTrkdidr_0_20W<GicRegsIts_15GitsTrkdidrSpec> {
        Its_15GitsTrkdidr_0_20W::new(self, 0)
    }
}
#[doc = "GITS_TRKDIDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__15_gits_trkdidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__15_gits_trkdidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_15GitsTrkdidrSpec;
impl crate::RegisterSpec for GicRegsIts_15GitsTrkdidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__15_gits_trkdidr::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_15GitsTrkdidrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__15_gits_trkdidr::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_15GitsTrkdidrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__15_GITS_TRKDIDR to value 0"]
impl crate::Resettable for GicRegsIts_15GitsTrkdidrSpec {
    const RESET_VALUE: u32 = 0;
}
