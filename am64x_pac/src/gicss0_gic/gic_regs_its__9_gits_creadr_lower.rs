#[doc = "Register `GIC_REGS_ITS__9_GITS_CREADR_lower` reader"]
pub type R = crate::R<GicRegsIts_9GitsCreadrLowerSpec>;
#[doc = "Register `GIC_REGS_ITS__9_GITS_CREADR_lower` writer"]
pub type W = crate::W<GicRegsIts_9GitsCreadrLowerSpec>;
#[doc = "Field `ITS__9_GITS_CREADR_LOWER__5_15` reader - 19:5\\]
Offset"]
pub type Its_9GitsCreadrLower_5_15R = crate::FieldReader<u16>;
#[doc = "Field `ITS__9_GITS_CREADR_LOWER__5_15` writer - 19:5\\]
Offset"]
pub type Its_9GitsCreadrLower_5_15W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 5:19 - 19:5\\]
Offset"]
    #[inline(always)]
    pub fn its__9_gits_creadr_lower__5_15(&self) -> Its_9GitsCreadrLower_5_15R {
        Its_9GitsCreadrLower_5_15R::new(((self.bits >> 5) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 5:19 - 19:5\\]
Offset"]
    #[inline(always)]
    #[must_use]
    pub fn its__9_gits_creadr_lower__5_15(
        &mut self,
    ) -> Its_9GitsCreadrLower_5_15W<GicRegsIts_9GitsCreadrLowerSpec> {
        Its_9GitsCreadrLower_5_15W::new(self, 5)
    }
}
#[doc = "GITS_CREADR_lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__9_gits_creadr_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__9_gits_creadr_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_9GitsCreadrLowerSpec;
impl crate::RegisterSpec for GicRegsIts_9GitsCreadrLowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__9_gits_creadr_lower::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_9GitsCreadrLowerSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__9_gits_creadr_lower::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_9GitsCreadrLowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__9_GITS_CREADR_lower to value 0"]
impl crate::Resettable for GicRegsIts_9GitsCreadrLowerSpec {
    const RESET_VALUE: u32 = 0;
}
