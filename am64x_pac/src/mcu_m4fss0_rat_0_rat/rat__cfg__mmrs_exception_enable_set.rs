#[doc = "Register `RAT__CFG__MMRS_exception_enable_set` reader"]
pub type R = crate::R<Rat_Cfg_MmrsExceptionEnableSetSpec>;
#[doc = "Register `RAT__CFG__MMRS_exception_enable_set` writer"]
pub type W = crate::W<Rat_Cfg_MmrsExceptionEnableSetSpec>;
#[doc = "Field `ENABLE_SET` reader - 0:0\\]
Write a 1 to set the exception interrupt enable signal."]
pub type EnableSetR = crate::BitReader;
#[doc = "Field `ENABLE_SET` writer - 0:0\\]
Write a 1 to set the exception interrupt enable signal."]
pub type EnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write a 1 to set the exception interrupt enable signal."]
    #[inline(always)]
    pub fn enable_set(&self) -> EnableSetR {
        EnableSetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write a 1 to set the exception interrupt enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn enable_set(&mut self) -> EnableSetW<Rat_Cfg_MmrsExceptionEnableSetSpec> {
        EnableSetW::new(self, 0)
    }
}
#[doc = "The Exception Logging Interrupt Enable Set Register allows to set the interrupt enable signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_enable_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_enable_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rat_Cfg_MmrsExceptionEnableSetSpec;
impl crate::RegisterSpec for Rat_Cfg_MmrsExceptionEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rat__cfg__mmrs_exception_enable_set::R`](R) reader structure"]
impl crate::Readable for Rat_Cfg_MmrsExceptionEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`rat__cfg__mmrs_exception_enable_set::W`](W) writer structure"]
impl crate::Writable for Rat_Cfg_MmrsExceptionEnableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAT__CFG__MMRS_exception_enable_set to value 0"]
impl crate::Resettable for Rat_Cfg_MmrsExceptionEnableSetSpec {
    const RESET_VALUE: u32 = 0;
}
