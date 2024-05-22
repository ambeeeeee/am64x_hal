#[doc = "Register `CPSW_NUSS_VBUSP_CUT_THRESHOLD_REG` reader"]
pub type R = crate::R<CpswNussVbuspCutThresholdRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_CUT_THRESHOLD_REG` writer"]
pub type W = crate::W<CpswNussVbuspCutThresholdRegSpec>;
#[doc = "Field `CUT_THRESH` reader - 3:0\\]
Cut-thru Threshold"]
pub type CutThreshR = crate::FieldReader;
#[doc = "Field `CUT_THRESH` writer - 3:0\\]
Cut-thru Threshold"]
pub type CutThreshW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Cut-thru Threshold"]
    #[inline(always)]
    pub fn cut_thresh(&self) -> CutThreshR {
        CutThreshR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Cut-thru Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn cut_thresh(&mut self) -> CutThreshW<CpswNussVbuspCutThresholdRegSpec> {
        CutThreshW::new(self, 0)
    }
}
#[doc = "Cut-thru Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_cut_threshold_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_cut_threshold_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspCutThresholdRegSpec;
impl crate::RegisterSpec for CpswNussVbuspCutThresholdRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_cut_threshold_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspCutThresholdRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_cut_threshold_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspCutThresholdRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_CUT_THRESHOLD_REG to value 0"]
impl crate::Resettable for CpswNussVbuspCutThresholdRegSpec {
    const RESET_VALUE: u32 = 0;
}
