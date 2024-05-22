#[doc = "Register `GIC_REGS_ITS__12_GITS_BASER0_upper` reader"]
pub type R = crate::R<GicRegsIts_12GitsBaser0UpperSpec>;
#[doc = "Register `GIC_REGS_ITS__12_GITS_BASER0_upper` writer"]
pub type W = crate::W<GicRegsIts_12GitsBaser0UpperSpec>;
#[doc = "Field `ITS__12_GITS_BASER0_UPPER__0_16` reader - 15:0\\]
Physical Address \\[47:32\\]"]
pub type Its_12GitsBaser0Upper_0_16R = crate::FieldReader<u16>;
#[doc = "Field `ITS__12_GITS_BASER0_UPPER__0_16` writer - 15:0\\]
Physical Address \\[47:32\\]"]
pub type Its_12GitsBaser0Upper_0_16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ITS__12_GITS_BASER0_UPPER__16_8` reader - 23:16\\]
Entry Size"]
pub type Its_12GitsBaser0Upper_16_8R = crate::FieldReader;
#[doc = "Field `ITS__12_GITS_BASER0_UPPER__16_8` writer - 23:16\\]
Entry Size"]
pub type Its_12GitsBaser0Upper_16_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ITS__12_GITS_BASER0_UPPER__24_3` reader - 26:24\\]
Type"]
pub type Its_12GitsBaser0Upper_24_3R = crate::FieldReader;
#[doc = "Field `ITS__12_GITS_BASER0_UPPER__24_3` writer - 26:24\\]
Type"]
pub type Its_12GitsBaser0Upper_24_3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Physical Address \\[47:32\\]"]
    #[inline(always)]
    pub fn its__12_gits_baser0_upper__0_16(&self) -> Its_12GitsBaser0Upper_0_16R {
        Its_12GitsBaser0Upper_0_16R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Entry Size"]
    #[inline(always)]
    pub fn its__12_gits_baser0_upper__16_8(&self) -> Its_12GitsBaser0Upper_16_8R {
        Its_12GitsBaser0Upper_16_8R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Type"]
    #[inline(always)]
    pub fn its__12_gits_baser0_upper__24_3(&self) -> Its_12GitsBaser0Upper_24_3R {
        Its_12GitsBaser0Upper_24_3R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Physical Address \\[47:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn its__12_gits_baser0_upper__0_16(
        &mut self,
    ) -> Its_12GitsBaser0Upper_0_16W<GicRegsIts_12GitsBaser0UpperSpec> {
        Its_12GitsBaser0Upper_0_16W::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Entry Size"]
    #[inline(always)]
    #[must_use]
    pub fn its__12_gits_baser0_upper__16_8(
        &mut self,
    ) -> Its_12GitsBaser0Upper_16_8W<GicRegsIts_12GitsBaser0UpperSpec> {
        Its_12GitsBaser0Upper_16_8W::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Type"]
    #[inline(always)]
    #[must_use]
    pub fn its__12_gits_baser0_upper__24_3(
        &mut self,
    ) -> Its_12GitsBaser0Upper_24_3W<GicRegsIts_12GitsBaser0UpperSpec> {
        Its_12GitsBaser0Upper_24_3W::new(self, 24)
    }
}
#[doc = "GITS_BASER0_upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__12_gits_baser0_upper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__12_gits_baser0_upper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_12GitsBaser0UpperSpec;
impl crate::RegisterSpec for GicRegsIts_12GitsBaser0UpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__12_gits_baser0_upper::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_12GitsBaser0UpperSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__12_gits_baser0_upper::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_12GitsBaser0UpperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__12_GITS_BASER0_upper to value 0x0107_0000"]
impl crate::Resettable for GicRegsIts_12GitsBaser0UpperSpec {
    const RESET_VALUE: u32 = 0x0107_0000;
}
