#[doc = "Register `PR1_RAT_SLICE0__CFG__MMRS_base` reader"]
pub type R = crate::R<Pr1RatSlice0_Cfg_MmrsBaseSpec>;
#[doc = "Register `PR1_RAT_SLICE0__CFG__MMRS_base` writer"]
pub type W = crate::W<Pr1RatSlice0_Cfg_MmrsBaseSpec>;
#[doc = "Field `BASE` reader - 31:0\\]
Base Address for the Region. It must be aligned to the programmed size."]
pub type BaseR = crate::FieldReader<u32>;
#[doc = "Field `BASE` writer - 31:0\\]
Base Address for the Region. It must be aligned to the programmed size."]
pub type BaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Base Address for the Region. It must be aligned to the programmed size."]
    #[inline(always)]
    pub fn base(&self) -> BaseR {
        BaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Base Address for the Region. It must be aligned to the programmed size."]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BaseW<Pr1RatSlice0_Cfg_MmrsBaseSpec> {
        BaseW::new(self, 0)
    }
}
#[doc = "The Base Address for Region a. This is the source address for matching to a region.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice0__cfg__mmrs_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice0__cfg__mmrs_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1RatSlice0_Cfg_MmrsBaseSpec;
impl crate::RegisterSpec for Pr1RatSlice0_Cfg_MmrsBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_rat_slice0__cfg__mmrs_base::R`](R) reader structure"]
impl crate::Readable for Pr1RatSlice0_Cfg_MmrsBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_rat_slice0__cfg__mmrs_base::W`](W) writer structure"]
impl crate::Writable for Pr1RatSlice0_Cfg_MmrsBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_RAT_SLICE0__CFG__MMRS_base to value 0"]
impl crate::Resettable for Pr1RatSlice0_Cfg_MmrsBaseSpec {
    const RESET_VALUE: u32 = 0;
}
