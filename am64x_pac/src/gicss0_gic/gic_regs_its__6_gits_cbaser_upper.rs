#[doc = "Register `GIC_REGS_ITS__6_GITS_CBASER_upper` reader"]
pub type R = crate::R<GicRegsIts_6GitsCbaserUpperSpec>;
#[doc = "Register `GIC_REGS_ITS__6_GITS_CBASER_upper` writer"]
pub type W = crate::W<GicRegsIts_6GitsCbaserUpperSpec>;
#[doc = "Field `ITS__6_GITS_CBASER_UPPER__0_16` reader - 15:0\\]
Physical Address \\[47:32\\]"]
pub type Its_6GitsCbaserUpper_0_16R = crate::FieldReader<u16>;
#[doc = "Field `ITS__6_GITS_CBASER_UPPER__0_16` writer - 15:0\\]
Physical Address \\[47:32\\]"]
pub type Its_6GitsCbaserUpper_0_16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ITS__6_GITS_CBASER_UPPER__27_3` reader - 29:27\\]
Cacheability"]
pub type Its_6GitsCbaserUpper_27_3R = crate::FieldReader;
#[doc = "Field `ITS__6_GITS_CBASER_UPPER__27_3` writer - 29:27\\]
Cacheability"]
pub type Its_6GitsCbaserUpper_27_3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ITS__6_GITS_CBASER_UPPER__31_1` reader - 31:31\\]
Valid"]
pub type Its_6GitsCbaserUpper_31_1R = crate::BitReader;
#[doc = "Field `ITS__6_GITS_CBASER_UPPER__31_1` writer - 31:31\\]
Valid"]
pub type Its_6GitsCbaserUpper_31_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Physical Address \\[47:32\\]"]
    #[inline(always)]
    pub fn its__6_gits_cbaser_upper__0_16(&self) -> Its_6GitsCbaserUpper_0_16R {
        Its_6GitsCbaserUpper_0_16R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 27:29 - 29:27\\]
Cacheability"]
    #[inline(always)]
    pub fn its__6_gits_cbaser_upper__27_3(&self) -> Its_6GitsCbaserUpper_27_3R {
        Its_6GitsCbaserUpper_27_3R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Valid"]
    #[inline(always)]
    pub fn its__6_gits_cbaser_upper__31_1(&self) -> Its_6GitsCbaserUpper_31_1R {
        Its_6GitsCbaserUpper_31_1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Physical Address \\[47:32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn its__6_gits_cbaser_upper__0_16(
        &mut self,
    ) -> Its_6GitsCbaserUpper_0_16W<GicRegsIts_6GitsCbaserUpperSpec> {
        Its_6GitsCbaserUpper_0_16W::new(self, 0)
    }
    #[doc = "Bits 27:29 - 29:27\\]
Cacheability"]
    #[inline(always)]
    #[must_use]
    pub fn its__6_gits_cbaser_upper__27_3(
        &mut self,
    ) -> Its_6GitsCbaserUpper_27_3W<GicRegsIts_6GitsCbaserUpperSpec> {
        Its_6GitsCbaserUpper_27_3W::new(self, 27)
    }
    #[doc = "Bit 31 - 31:31\\]
Valid"]
    #[inline(always)]
    #[must_use]
    pub fn its__6_gits_cbaser_upper__31_1(
        &mut self,
    ) -> Its_6GitsCbaserUpper_31_1W<GicRegsIts_6GitsCbaserUpperSpec> {
        Its_6GitsCbaserUpper_31_1W::new(self, 31)
    }
}
#[doc = "GITS_CBASER_upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__6_gits_cbaser_upper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__6_gits_cbaser_upper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_6GitsCbaserUpperSpec;
impl crate::RegisterSpec for GicRegsIts_6GitsCbaserUpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__6_gits_cbaser_upper::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_6GitsCbaserUpperSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__6_gits_cbaser_upper::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_6GitsCbaserUpperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__6_GITS_CBASER_upper to value 0"]
impl crate::Resettable for GicRegsIts_6GitsCbaserUpperSpec {
    const RESET_VALUE: u32 = 0;
}
