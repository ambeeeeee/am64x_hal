#[doc = "Register `PR1_RAT_SLICE1__CFG__MMRS_trans_l` reader"]
pub type R = crate::R<Pr1RatSlice1_Cfg_MmrsTransLSpec>;
#[doc = "Register `PR1_RAT_SLICE1__CFG__MMRS_trans_l` writer"]
pub type W = crate::W<Pr1RatSlice1_Cfg_MmrsTransLSpec>;
#[doc = "Field `LOWER` reader - 31:0\\]
Translated Lower Address Bits for the Region. It must be aligned to the programmed size."]
pub type LowerR = crate::FieldReader<u32>;
#[doc = "Field `LOWER` writer - 31:0\\]
Translated Lower Address Bits for the Region. It must be aligned to the programmed size."]
pub type LowerW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Translated Lower Address Bits for the Region. It must be aligned to the programmed size."]
    #[inline(always)]
    pub fn lower(&self) -> LowerR {
        LowerR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Translated Lower Address Bits for the Region. It must be aligned to the programmed size."]
    #[inline(always)]
    #[must_use]
    pub fn lower(&mut self) -> LowerW<Pr1RatSlice1_Cfg_MmrsTransLSpec> {
        LowerW::new(self, 0)
    }
}
#[doc = "The Translated Lower Address Bits for Region a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_trans_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_trans_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1RatSlice1_Cfg_MmrsTransLSpec;
impl crate::RegisterSpec for Pr1RatSlice1_Cfg_MmrsTransLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_rat_slice1__cfg__mmrs_trans_l::R`](R) reader structure"]
impl crate::Readable for Pr1RatSlice1_Cfg_MmrsTransLSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_rat_slice1__cfg__mmrs_trans_l::W`](W) writer structure"]
impl crate::Writable for Pr1RatSlice1_Cfg_MmrsTransLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_RAT_SLICE1__CFG__MMRS_trans_l to value 0"]
impl crate::Resettable for Pr1RatSlice1_Cfg_MmrsTransLSpec {
    const RESET_VALUE: u32 = 0;
}
