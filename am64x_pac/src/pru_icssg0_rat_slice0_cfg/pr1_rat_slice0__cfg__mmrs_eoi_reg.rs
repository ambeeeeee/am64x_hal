#[doc = "Register `PR1_RAT_SLICE0__CFG__MMRS_eoi_reg` reader"]
pub type R = crate::R<Pr1RatSlice0_Cfg_MmrsEoiRegSpec>;
#[doc = "Register `PR1_RAT_SLICE0__CFG__MMRS_eoi_reg` writer"]
pub type W = crate::W<Pr1RatSlice0_Cfg_MmrsEoiRegSpec>;
#[doc = "Field `EOI_WR` reader - 15:0\\]
EOI Register"]
pub type EoiWrR = crate::FieldReader<u16>;
#[doc = "Field `EOI_WR` writer - 15:0\\]
EOI Register"]
pub type EoiWrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
EOI Register"]
    #[inline(always)]
    pub fn eoi_wr(&self) -> EoiWrR {
        EoiWrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
EOI Register"]
    #[inline(always)]
    #[must_use]
    pub fn eoi_wr(&mut self) -> EoiWrW<Pr1RatSlice0_Cfg_MmrsEoiRegSpec> {
        EoiWrW::new(self, 0)
    }
}
#[doc = "EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice0__cfg__mmrs_eoi_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice0__cfg__mmrs_eoi_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1RatSlice0_Cfg_MmrsEoiRegSpec;
impl crate::RegisterSpec for Pr1RatSlice0_Cfg_MmrsEoiRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_rat_slice0__cfg__mmrs_eoi_reg::R`](R) reader structure"]
impl crate::Readable for Pr1RatSlice0_Cfg_MmrsEoiRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_rat_slice0__cfg__mmrs_eoi_reg::W`](W) writer structure"]
impl crate::Writable for Pr1RatSlice0_Cfg_MmrsEoiRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_RAT_SLICE0__CFG__MMRS_eoi_reg to value 0"]
impl crate::Resettable for Pr1RatSlice0_Cfg_MmrsEoiRegSpec {
    const RESET_VALUE: u32 = 0;
}
