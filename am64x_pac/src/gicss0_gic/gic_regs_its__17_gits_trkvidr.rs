#[doc = "Register `GIC_REGS_ITS__17_GITS_TRKVIDR` reader"]
pub type R = crate::R<GicRegsIts_17GitsTrkvidrSpec>;
#[doc = "Register `GIC_REGS_ITS__17_GITS_TRKVIDR` writer"]
pub type W = crate::W<GicRegsIts_17GitsTrkvidrSpec>;
#[doc = "Field `ITS__17_GITS_TRKVIDR__0_16` reader - 15:0\\]
Input ID"]
pub type Its_17GitsTrkvidr_0_16R = crate::FieldReader<u16>;
#[doc = "Field `ITS__17_GITS_TRKVIDR__0_16` writer - 15:0\\]
Input ID"]
pub type Its_17GitsTrkvidr_0_16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Input ID"]
    #[inline(always)]
    pub fn its__17_gits_trkvidr__0_16(&self) -> Its_17GitsTrkvidr_0_16R {
        Its_17GitsTrkvidr_0_16R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Input ID"]
    #[inline(always)]
    #[must_use]
    pub fn its__17_gits_trkvidr__0_16(
        &mut self,
    ) -> Its_17GitsTrkvidr_0_16W<GicRegsIts_17GitsTrkvidrSpec> {
        Its_17GitsTrkvidr_0_16W::new(self, 0)
    }
}
#[doc = "GITS_TRKVIDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__17_gits_trkvidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__17_gits_trkvidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_17GitsTrkvidrSpec;
impl crate::RegisterSpec for GicRegsIts_17GitsTrkvidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__17_gits_trkvidr::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_17GitsTrkvidrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__17_gits_trkvidr::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_17GitsTrkvidrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__17_GITS_TRKVIDR to value 0"]
impl crate::Resettable for GicRegsIts_17GitsTrkvidrSpec {
    const RESET_VALUE: u32 = 0;
}
