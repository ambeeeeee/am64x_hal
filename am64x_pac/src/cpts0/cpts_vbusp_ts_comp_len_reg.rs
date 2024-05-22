#[doc = "Register `CPTS_VBUSP_TS_COMP_LEN_REG` reader"]
pub type R = crate::R<CptsVbuspTsCompLenRegSpec>;
#[doc = "Register `CPTS_VBUSP_TS_COMP_LEN_REG` writer"]
pub type W = crate::W<CptsVbuspTsCompLenRegSpec>;
#[doc = "Field `TS_COMP_LENGTH` reader - 31:0\\]
Time stamp comparison length"]
pub type TsCompLengthR = crate::FieldReader<u32>;
#[doc = "Field `TS_COMP_LENGTH` writer - 31:0\\]
Time stamp comparison length"]
pub type TsCompLengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp comparison length"]
    #[inline(always)]
    pub fn ts_comp_length(&self) -> TsCompLengthR {
        TsCompLengthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Time stamp comparison length"]
    #[inline(always)]
    #[must_use]
    pub fn ts_comp_length(&mut self) -> TsCompLengthW<CptsVbuspTsCompLenRegSpec> {
        TsCompLengthW::new(self, 0)
    }
}
#[doc = "Time Stamp Comparison Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_comp_len_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_comp_len_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspTsCompLenRegSpec;
impl crate::RegisterSpec for CptsVbuspTsCompLenRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_ts_comp_len_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspTsCompLenRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_ts_comp_len_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspTsCompLenRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_TS_COMP_LEN_REG to value 0"]
impl crate::Resettable for CptsVbuspTsCompLenRegSpec {
    const RESET_VALUE: u32 = 0;
}
