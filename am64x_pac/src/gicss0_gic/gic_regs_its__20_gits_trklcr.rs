#[doc = "Register `GIC_REGS_ITS__20_GITS_TRKLCR` reader"]
pub type R = crate::R<GicRegsIts_20GitsTrklcrSpec>;
#[doc = "Register `GIC_REGS_ITS__20_GITS_TRKLCR` writer"]
pub type W = crate::W<GicRegsIts_20GitsTrklcrSpec>;
#[doc = "Field `ITS__20_GITS_TRKLCR__0_16` reader - 15:0\\]
LPI cache misses"]
pub type Its_20GitsTrklcr_0_16R = crate::FieldReader<u16>;
#[doc = "Field `ITS__20_GITS_TRKLCR__0_16` writer - 15:0\\]
LPI cache misses"]
pub type Its_20GitsTrklcr_0_16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ITS__20_GITS_TRKLCR__16_16` reader - 31:16\\]
LPI cache hits"]
pub type Its_20GitsTrklcr_16_16R = crate::FieldReader<u16>;
#[doc = "Field `ITS__20_GITS_TRKLCR__16_16` writer - 31:16\\]
LPI cache hits"]
pub type Its_20GitsTrklcr_16_16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
LPI cache misses"]
    #[inline(always)]
    pub fn its__20_gits_trklcr__0_16(&self) -> Its_20GitsTrklcr_0_16R {
        Its_20GitsTrklcr_0_16R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
LPI cache hits"]
    #[inline(always)]
    pub fn its__20_gits_trklcr__16_16(&self) -> Its_20GitsTrklcr_16_16R {
        Its_20GitsTrklcr_16_16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
LPI cache misses"]
    #[inline(always)]
    #[must_use]
    pub fn its__20_gits_trklcr__0_16(
        &mut self,
    ) -> Its_20GitsTrklcr_0_16W<GicRegsIts_20GitsTrklcrSpec> {
        Its_20GitsTrklcr_0_16W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
LPI cache hits"]
    #[inline(always)]
    #[must_use]
    pub fn its__20_gits_trklcr__16_16(
        &mut self,
    ) -> Its_20GitsTrklcr_16_16W<GicRegsIts_20GitsTrklcrSpec> {
        Its_20GitsTrklcr_16_16W::new(self, 16)
    }
}
#[doc = "GITS_TRKLCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__20_gits_trklcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__20_gits_trklcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_20GitsTrklcrSpec;
impl crate::RegisterSpec for GicRegsIts_20GitsTrklcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__20_gits_trklcr::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_20GitsTrklcrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__20_gits_trklcr::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_20GitsTrklcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__20_GITS_TRKLCR to value 0"]
impl crate::Resettable for GicRegsIts_20GitsTrklcrSpec {
    const RESET_VALUE: u32 = 0;
}
