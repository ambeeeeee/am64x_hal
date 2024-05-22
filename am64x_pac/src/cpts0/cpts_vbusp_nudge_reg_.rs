#[doc = "Register `CPTS_VBUSP_NUDGE_REG_` reader"]
pub type R = crate::R<CptsVbuspNudgeReg_Spec>;
#[doc = "Register `CPTS_VBUSP_NUDGE_REG_` writer"]
pub type W = crate::W<CptsVbuspNudgeReg_Spec>;
#[doc = "Field `NUDGE` reader - 7:0\\]
Time Stamp ESTF Generate Function Nudge Value"]
pub type NudgeR = crate::FieldReader;
#[doc = "Field `NUDGE` writer - 7:0\\]
Time Stamp ESTF Generate Function Nudge Value"]
pub type NudgeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Time Stamp ESTF Generate Function Nudge Value"]
    #[inline(always)]
    pub fn nudge(&self) -> NudgeR {
        NudgeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Time Stamp ESTF Generate Function Nudge Value"]
    #[inline(always)]
    #[must_use]
    pub fn nudge(&mut self) -> NudgeW<CptsVbuspNudgeReg_Spec> {
        NudgeW::new(self, 0)
    }
}
#[doc = "Time Stamp ESTF Generate Function Nudge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_nudge_reg_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_nudge_reg_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspNudgeReg_Spec;
impl crate::RegisterSpec for CptsVbuspNudgeReg_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_nudge_reg_::R`](R) reader structure"]
impl crate::Readable for CptsVbuspNudgeReg_Spec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_nudge_reg_::W`](W) writer structure"]
impl crate::Writable for CptsVbuspNudgeReg_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_NUDGE_REG_ to value 0"]
impl crate::Resettable for CptsVbuspNudgeReg_Spec {
    const RESET_VALUE: u32 = 0;
}
