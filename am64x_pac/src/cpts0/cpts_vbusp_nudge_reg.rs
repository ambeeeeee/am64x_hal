#[doc = "Register `CPTS_VBUSP_NUDGE_REG` reader"]
pub type R = crate::R<CptsVbuspNudgeRegSpec>;
#[doc = "Register `CPTS_VBUSP_NUDGE_REG` writer"]
pub type W = crate::W<CptsVbuspNudgeRegSpec>;
#[doc = "Field `NUDGE` reader - 7:0\\]
Time Stamp Generate Function Nudge Value"]
pub type NudgeR = crate::FieldReader;
#[doc = "Field `NUDGE` writer - 7:0\\]
Time Stamp Generate Function Nudge Value"]
pub type NudgeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Time Stamp Generate Function Nudge Value"]
    #[inline(always)]
    pub fn nudge(&self) -> NudgeR {
        NudgeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Time Stamp Generate Function Nudge Value"]
    #[inline(always)]
    #[must_use]
    pub fn nudge(&mut self) -> NudgeW<CptsVbuspNudgeRegSpec> {
        NudgeW::new(self, 0)
    }
}
#[doc = "Time Stamp Generate Function Nudge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_nudge_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_nudge_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspNudgeRegSpec;
impl crate::RegisterSpec for CptsVbuspNudgeRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_nudge_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspNudgeRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_nudge_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspNudgeRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_NUDGE_REG to value 0"]
impl crate::Resettable for CptsVbuspNudgeRegSpec {
    const RESET_VALUE: u32 = 0;
}
