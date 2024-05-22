#[doc = "Register `CPTS_VBUSP_TS_COMP_NUDGE_REG` reader"]
pub type R = crate::R<CptsVbuspTsCompNudgeRegSpec>;
#[doc = "Register `CPTS_VBUSP_TS_COMP_NUDGE_REG` writer"]
pub type W = crate::W<CptsVbuspTsCompNudgeRegSpec>;
#[doc = "Field `NUDGE` reader - 7:0\\]
This 2s complement number is added to the ts_comp_length value to increase or decrease the TS_COMP length by the nudge amount"]
pub type NudgeR = crate::FieldReader;
#[doc = "Field `NUDGE` writer - 7:0\\]
This 2s complement number is added to the ts_comp_length value to increase or decrease the TS_COMP length by the nudge amount"]
pub type NudgeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
This 2s complement number is added to the ts_comp_length value to increase or decrease the TS_COMP length by the nudge amount"]
    #[inline(always)]
    pub fn nudge(&self) -> NudgeR {
        NudgeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
This 2s complement number is added to the ts_comp_length value to increase or decrease the TS_COMP length by the nudge amount"]
    #[inline(always)]
    #[must_use]
    pub fn nudge(&mut self) -> NudgeW<CptsVbuspTsCompNudgeRegSpec> {
        NudgeW::new(self, 0)
    }
}
#[doc = "Time Stamp Comparison Nudge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_comp_nudge_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_comp_nudge_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspTsCompNudgeRegSpec;
impl crate::RegisterSpec for CptsVbuspTsCompNudgeRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_ts_comp_nudge_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspTsCompNudgeRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_ts_comp_nudge_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspTsCompNudgeRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_TS_COMP_NUDGE_REG to value 0"]
impl crate::Resettable for CptsVbuspTsCompNudgeRegSpec {
    const RESET_VALUE: u32 = 0;
}
