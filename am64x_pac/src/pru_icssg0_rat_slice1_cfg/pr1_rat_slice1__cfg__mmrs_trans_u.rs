#[doc = "Register `PR1_RAT_SLICE1__CFG__MMRS_trans_u` reader"]
pub type R = crate::R<Pr1RatSlice1_Cfg_MmrsTransUSpec>;
#[doc = "Register `PR1_RAT_SLICE1__CFG__MMRS_trans_u` writer"]
pub type W = crate::W<Pr1RatSlice1_Cfg_MmrsTransUSpec>;
#[doc = "Field `UPPER` reader - 15:0\\]
Translated Upper Address Bits for the Region"]
pub type UpperR = crate::FieldReader<u16>;
#[doc = "Field `UPPER` writer - 15:0\\]
Translated Upper Address Bits for the Region"]
pub type UpperW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Translated Upper Address Bits for the Region"]
    #[inline(always)]
    pub fn upper(&self) -> UpperR {
        UpperR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Translated Upper Address Bits for the Region"]
    #[inline(always)]
    #[must_use]
    pub fn upper(&mut self) -> UpperW<Pr1RatSlice1_Cfg_MmrsTransUSpec> {
        UpperW::new(self, 0)
    }
}
#[doc = "The Translated Upper Address Bits for Region a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_trans_u::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_trans_u::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1RatSlice1_Cfg_MmrsTransUSpec;
impl crate::RegisterSpec for Pr1RatSlice1_Cfg_MmrsTransUSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_rat_slice1__cfg__mmrs_trans_u::R`](R) reader structure"]
impl crate::Readable for Pr1RatSlice1_Cfg_MmrsTransUSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_rat_slice1__cfg__mmrs_trans_u::W`](W) writer structure"]
impl crate::Writable for Pr1RatSlice1_Cfg_MmrsTransUSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_RAT_SLICE1__CFG__MMRS_trans_u to value 0"]
impl crate::Resettable for Pr1RatSlice1_Cfg_MmrsTransUSpec {
    const RESET_VALUE: u32 = 0;
}
