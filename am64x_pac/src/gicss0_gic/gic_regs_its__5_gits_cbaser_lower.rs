#[doc = "Register `GIC_REGS_ITS__5_GITS_CBASER_lower` reader"]
pub type R = crate::R<GicRegsIts_5GitsCbaserLowerSpec>;
#[doc = "Register `GIC_REGS_ITS__5_GITS_CBASER_lower` writer"]
pub type W = crate::W<GicRegsIts_5GitsCbaserLowerSpec>;
#[doc = "Field `ITS__5_GITS_CBASER_LOWER__0_8` reader - 7:0\\]
Size"]
pub type Its_5GitsCbaserLower_0_8R = crate::FieldReader;
#[doc = "Field `ITS__5_GITS_CBASER_LOWER__0_8` writer - 7:0\\]
Size"]
pub type Its_5GitsCbaserLower_0_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ITS__5_GITS_CBASER_LOWER__12_20` reader - 31:12\\]
Physical Address \\[31:12\\]"]
pub type Its_5GitsCbaserLower_12_20R = crate::FieldReader<u32>;
#[doc = "Field `ITS__5_GITS_CBASER_LOWER__12_20` writer - 31:12\\]
Physical Address \\[31:12\\]"]
pub type Its_5GitsCbaserLower_12_20W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Size"]
    #[inline(always)]
    pub fn its__5_gits_cbaser_lower__0_8(&self) -> Its_5GitsCbaserLower_0_8R {
        Its_5GitsCbaserLower_0_8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Physical Address \\[31:12\\]"]
    #[inline(always)]
    pub fn its__5_gits_cbaser_lower__12_20(&self) -> Its_5GitsCbaserLower_12_20R {
        Its_5GitsCbaserLower_12_20R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Size"]
    #[inline(always)]
    #[must_use]
    pub fn its__5_gits_cbaser_lower__0_8(
        &mut self,
    ) -> Its_5GitsCbaserLower_0_8W<GicRegsIts_5GitsCbaserLowerSpec> {
        Its_5GitsCbaserLower_0_8W::new(self, 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Physical Address \\[31:12\\]"]
    #[inline(always)]
    #[must_use]
    pub fn its__5_gits_cbaser_lower__12_20(
        &mut self,
    ) -> Its_5GitsCbaserLower_12_20W<GicRegsIts_5GitsCbaserLowerSpec> {
        Its_5GitsCbaserLower_12_20W::new(self, 12)
    }
}
#[doc = "GITS_CBASER_lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__5_gits_cbaser_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__5_gits_cbaser_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_5GitsCbaserLowerSpec;
impl crate::RegisterSpec for GicRegsIts_5GitsCbaserLowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__5_gits_cbaser_lower::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_5GitsCbaserLowerSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__5_gits_cbaser_lower::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_5GitsCbaserLowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__5_GITS_CBASER_lower to value 0"]
impl crate::Resettable for GicRegsIts_5GitsCbaserLowerSpec {
    const RESET_VALUE: u32 = 0;
}
