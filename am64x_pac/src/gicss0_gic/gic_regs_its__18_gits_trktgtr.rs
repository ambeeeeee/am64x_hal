#[doc = "Register `GIC_REGS_ITS__18_GITS_TRKTGTR` reader"]
pub type R = crate::R<GicRegsIts_18GitsTrktgtrSpec>;
#[doc = "Register `GIC_REGS_ITS__18_GITS_TRKTGTR` writer"]
pub type W = crate::W<GicRegsIts_18GitsTrktgtrSpec>;
#[doc = "Field `ITS__18_GITS_TRKTGTR__0_7` reader - 6:0\\]
Target CPU"]
pub type Its_18GitsTrktgtr_0_7R = crate::FieldReader;
#[doc = "Field `ITS__18_GITS_TRKTGTR__0_7` writer - 6:0\\]
Target CPU"]
pub type Its_18GitsTrktgtr_0_7W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Target CPU"]
    #[inline(always)]
    pub fn its__18_gits_trktgtr__0_7(&self) -> Its_18GitsTrktgtr_0_7R {
        Its_18GitsTrktgtr_0_7R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Target CPU"]
    #[inline(always)]
    #[must_use]
    pub fn its__18_gits_trktgtr__0_7(
        &mut self,
    ) -> Its_18GitsTrktgtr_0_7W<GicRegsIts_18GitsTrktgtrSpec> {
        Its_18GitsTrktgtr_0_7W::new(self, 0)
    }
}
#[doc = "GITS_TRKTGTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__18_gits_trktgtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__18_gits_trktgtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_18GitsTrktgtrSpec;
impl crate::RegisterSpec for GicRegsIts_18GitsTrktgtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__18_gits_trktgtr::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_18GitsTrktgtrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__18_gits_trktgtr::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_18GitsTrktgtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__18_GITS_TRKTGTR to value 0"]
impl crate::Resettable for GicRegsIts_18GitsTrktgtrSpec {
    const RESET_VALUE: u32 = 0;
}
