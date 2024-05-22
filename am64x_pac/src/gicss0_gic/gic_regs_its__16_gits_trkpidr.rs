#[doc = "Register `GIC_REGS_ITS__16_GITS_TRKPIDR` reader"]
pub type R = crate::R<GicRegsIts_16GitsTrkpidrSpec>;
#[doc = "Register `GIC_REGS_ITS__16_GITS_TRKPIDR` writer"]
pub type W = crate::W<GicRegsIts_16GitsTrkpidrSpec>;
#[doc = "Field `ITS__16_GITS_TRKPIDR__0_16` reader - 15:0\\]
Translated ID"]
pub type Its_16GitsTrkpidr_0_16R = crate::FieldReader<u16>;
#[doc = "Field `ITS__16_GITS_TRKPIDR__0_16` writer - 15:0\\]
Translated ID"]
pub type Its_16GitsTrkpidr_0_16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Translated ID"]
    #[inline(always)]
    pub fn its__16_gits_trkpidr__0_16(&self) -> Its_16GitsTrkpidr_0_16R {
        Its_16GitsTrkpidr_0_16R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Translated ID"]
    #[inline(always)]
    #[must_use]
    pub fn its__16_gits_trkpidr__0_16(
        &mut self,
    ) -> Its_16GitsTrkpidr_0_16W<GicRegsIts_16GitsTrkpidrSpec> {
        Its_16GitsTrkpidr_0_16W::new(self, 0)
    }
}
#[doc = "GITS_TRKPIDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__16_gits_trkpidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__16_gits_trkpidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_16GitsTrkpidrSpec;
impl crate::RegisterSpec for GicRegsIts_16GitsTrkpidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__16_gits_trkpidr::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_16GitsTrkpidrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__16_gits_trkpidr::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_16GitsTrkpidrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__16_GITS_TRKPIDR to value 0"]
impl crate::Resettable for GicRegsIts_16GitsTrkpidrSpec {
    const RESET_VALUE: u32 = 0;
}
