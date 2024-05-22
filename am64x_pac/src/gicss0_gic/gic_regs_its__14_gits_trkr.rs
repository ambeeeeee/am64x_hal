#[doc = "Register `GIC_REGS_ITS__14_GITS_TRKR` reader"]
pub type R = crate::R<GicRegsIts_14GitsTrkrSpec>;
#[doc = "Register `GIC_REGS_ITS__14_GITS_TRKR` writer"]
pub type W = crate::W<GicRegsIts_14GitsTrkrSpec>;
#[doc = "Field `ITS__14_GITS_TRKR__0_1` reader - 0:0\\]
DID out of range"]
pub type Its_14GitsTrkr_0_1R = crate::BitReader;
#[doc = "Field `ITS__14_GITS_TRKR__0_1` writer - 0:0\\]
DID out of range"]
pub type Its_14GitsTrkr_0_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS__14_GITS_TRKR__1_1` reader - 1:1\\]
DID unmapped"]
pub type Its_14GitsTrkr_1_1R = crate::BitReader;
#[doc = "Field `ITS__14_GITS_TRKR__1_1` writer - 1:1\\]
DID unmapped"]
pub type Its_14GitsTrkr_1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS__14_GITS_TRKR__2_1` reader - 2:2\\]
Input ID out of range"]
pub type Its_14GitsTrkr_2_1R = crate::BitReader;
#[doc = "Field `ITS__14_GITS_TRKR__2_1` writer - 2:2\\]
Input ID out of range"]
pub type Its_14GitsTrkr_2_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS__14_GITS_TRKR__3_1` reader - 3:3\\]
DID/ID not mapped"]
pub type Its_14GitsTrkr_3_1R = crate::BitReader;
#[doc = "Field `ITS__14_GITS_TRKR__3_1` writer - 3:3\\]
DID/ID not mapped"]
pub type Its_14GitsTrkr_3_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS__14_GITS_TRKR__4_1` reader - 4:4\\]
Target CPU out of range"]
pub type Its_14GitsTrkr_4_1R = crate::BitReader;
#[doc = "Field `ITS__14_GITS_TRKR__4_1` writer - 4:4\\]
Target CPU out of range"]
pub type Its_14GitsTrkr_4_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITS__14_GITS_TRKR__5_1` reader - 5:5\\]
Translated ID out of range"]
pub type Its_14GitsTrkr_5_1R = crate::BitReader;
#[doc = "Field `ITS__14_GITS_TRKR__5_1` writer - 5:5\\]
Translated ID out of range"]
pub type Its_14GitsTrkr_5_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DID out of range"]
    #[inline(always)]
    pub fn its__14_gits_trkr__0_1(&self) -> Its_14GitsTrkr_0_1R {
        Its_14GitsTrkr_0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DID unmapped"]
    #[inline(always)]
    pub fn its__14_gits_trkr__1_1(&self) -> Its_14GitsTrkr_1_1R {
        Its_14GitsTrkr_1_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Input ID out of range"]
    #[inline(always)]
    pub fn its__14_gits_trkr__2_1(&self) -> Its_14GitsTrkr_2_1R {
        Its_14GitsTrkr_2_1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
DID/ID not mapped"]
    #[inline(always)]
    pub fn its__14_gits_trkr__3_1(&self) -> Its_14GitsTrkr_3_1R {
        Its_14GitsTrkr_3_1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Target CPU out of range"]
    #[inline(always)]
    pub fn its__14_gits_trkr__4_1(&self) -> Its_14GitsTrkr_4_1R {
        Its_14GitsTrkr_4_1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Translated ID out of range"]
    #[inline(always)]
    pub fn its__14_gits_trkr__5_1(&self) -> Its_14GitsTrkr_5_1R {
        Its_14GitsTrkr_5_1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DID out of range"]
    #[inline(always)]
    #[must_use]
    pub fn its__14_gits_trkr__0_1(&mut self) -> Its_14GitsTrkr_0_1W<GicRegsIts_14GitsTrkrSpec> {
        Its_14GitsTrkr_0_1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DID unmapped"]
    #[inline(always)]
    #[must_use]
    pub fn its__14_gits_trkr__1_1(&mut self) -> Its_14GitsTrkr_1_1W<GicRegsIts_14GitsTrkrSpec> {
        Its_14GitsTrkr_1_1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Input ID out of range"]
    #[inline(always)]
    #[must_use]
    pub fn its__14_gits_trkr__2_1(&mut self) -> Its_14GitsTrkr_2_1W<GicRegsIts_14GitsTrkrSpec> {
        Its_14GitsTrkr_2_1W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
DID/ID not mapped"]
    #[inline(always)]
    #[must_use]
    pub fn its__14_gits_trkr__3_1(&mut self) -> Its_14GitsTrkr_3_1W<GicRegsIts_14GitsTrkrSpec> {
        Its_14GitsTrkr_3_1W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Target CPU out of range"]
    #[inline(always)]
    #[must_use]
    pub fn its__14_gits_trkr__4_1(&mut self) -> Its_14GitsTrkr_4_1W<GicRegsIts_14GitsTrkrSpec> {
        Its_14GitsTrkr_4_1W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Translated ID out of range"]
    #[inline(always)]
    #[must_use]
    pub fn its__14_gits_trkr__5_1(&mut self) -> Its_14GitsTrkr_5_1W<GicRegsIts_14GitsTrkrSpec> {
        Its_14GitsTrkr_5_1W::new(self, 5)
    }
}
#[doc = "GITS_TRKR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__14_gits_trkr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__14_gits_trkr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_14GitsTrkrSpec;
impl crate::RegisterSpec for GicRegsIts_14GitsTrkrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__14_gits_trkr::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_14GitsTrkrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__14_gits_trkr::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_14GitsTrkrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__14_GITS_TRKR to value 0"]
impl crate::Resettable for GicRegsIts_14GitsTrkrSpec {
    const RESET_VALUE: u32 = 0;
}
