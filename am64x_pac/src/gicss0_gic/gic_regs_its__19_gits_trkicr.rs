#[doc = "Register `GIC_REGS_ITS__19_GITS_TRKICR` reader"]
pub type R = crate::R<GicRegsIts_19GitsTrkicrSpec>;
#[doc = "Register `GIC_REGS_ITS__19_GITS_TRKICR` writer"]
pub type W = crate::W<GicRegsIts_19GitsTrkicrSpec>;
#[doc = "Field `ITS__19_GITS_TRKICR__0_16` reader - 15:0\\]
ITE cache misses"]
pub type Its_19GitsTrkicr_0_16R = crate::FieldReader<u16>;
#[doc = "Field `ITS__19_GITS_TRKICR__0_16` writer - 15:0\\]
ITE cache misses"]
pub type Its_19GitsTrkicr_0_16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ITS__19_GITS_TRKICR__16_16` reader - 31:16\\]
ITE cache hits"]
pub type Its_19GitsTrkicr_16_16R = crate::FieldReader<u16>;
#[doc = "Field `ITS__19_GITS_TRKICR__16_16` writer - 31:16\\]
ITE cache hits"]
pub type Its_19GitsTrkicr_16_16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
ITE cache misses"]
    #[inline(always)]
    pub fn its__19_gits_trkicr__0_16(&self) -> Its_19GitsTrkicr_0_16R {
        Its_19GitsTrkicr_0_16R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ITE cache hits"]
    #[inline(always)]
    pub fn its__19_gits_trkicr__16_16(&self) -> Its_19GitsTrkicr_16_16R {
        Its_19GitsTrkicr_16_16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
ITE cache misses"]
    #[inline(always)]
    #[must_use]
    pub fn its__19_gits_trkicr__0_16(
        &mut self,
    ) -> Its_19GitsTrkicr_0_16W<GicRegsIts_19GitsTrkicrSpec> {
        Its_19GitsTrkicr_0_16W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ITE cache hits"]
    #[inline(always)]
    #[must_use]
    pub fn its__19_gits_trkicr__16_16(
        &mut self,
    ) -> Its_19GitsTrkicr_16_16W<GicRegsIts_19GitsTrkicrSpec> {
        Its_19GitsTrkicr_16_16W::new(self, 16)
    }
}
#[doc = "GITS_TRKICR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__19_gits_trkicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__19_gits_trkicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_19GitsTrkicrSpec;
impl crate::RegisterSpec for GicRegsIts_19GitsTrkicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__19_gits_trkicr::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_19GitsTrkicrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__19_gits_trkicr::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_19GitsTrkicrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__19_GITS_TRKICR to value 0"]
impl crate::Resettable for GicRegsIts_19GitsTrkicrSpec {
    const RESET_VALUE: u32 = 0;
}
