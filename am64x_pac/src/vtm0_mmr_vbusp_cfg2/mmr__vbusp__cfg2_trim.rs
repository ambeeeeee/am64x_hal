#[doc = "Register `MMR__VBUSP__CFG2_TRIM` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg2TrimSpec>;
#[doc = "Register `MMR__VBUSP__CFG2_TRIM` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg2TrimSpec>;
#[doc = "Field `TRIMG` reader - 4:0\\]
Trim gain bits in the temp sensor. Reset value is from e-fuse at POR, efuse_tmpsens\\[a\\]_trimg."]
pub type TrimgR = crate::FieldReader;
#[doc = "Field `TRIMG` writer - 4:0\\]
Trim gain bits in the temp sensor. Reset value is from e-fuse at POR, efuse_tmpsens\\[a\\]_trimg."]
pub type TrimgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMO` reader - 13:8\\]
Trim offset bits in the temp sensor. Reset value is from e-fuse at POR, efuse_tmpsens\\[a\\]_trimo."]
pub type TrimoR = crate::FieldReader;
#[doc = "Field `TRIMO` writer - 13:8\\]
Trim offset bits in the temp sensor. Reset value is from e-fuse at POR, efuse_tmpsens\\[a\\]_trimo."]
pub type TrimoW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Trim gain bits in the temp sensor. Reset value is from e-fuse at POR, efuse_tmpsens\\[a\\]_trimg."]
    #[inline(always)]
    pub fn trimg(&self) -> TrimgR {
        TrimgR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Trim offset bits in the temp sensor. Reset value is from e-fuse at POR, efuse_tmpsens\\[a\\]_trimo."]
    #[inline(always)]
    pub fn trimo(&self) -> TrimoR {
        TrimoR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Trim gain bits in the temp sensor. Reset value is from e-fuse at POR, efuse_tmpsens\\[a\\]_trimg."]
    #[inline(always)]
    #[must_use]
    pub fn trimg(&mut self) -> TrimgW<Mmr_Vbusp_Cfg2TrimSpec> {
        TrimgW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Trim offset bits in the temp sensor. Reset value is from e-fuse at POR, efuse_tmpsens\\[a\\]_trimo."]
    #[inline(always)]
    #[must_use]
    pub fn trimo(&mut self) -> TrimoW<Mmr_Vbusp_Cfg2TrimSpec> {
        TrimoW::new(self, 8)
    }
}
#[doc = "Temperature Sensor Band-gap trim values register for sensor a. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg2_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg2_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg2TrimSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg2TrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg2_trim::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg2TrimSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg2_trim::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg2TrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG2_TRIM to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg2TrimSpec {
    const RESET_VALUE: u32 = 0;
}
