#[doc = "Register `CPSW_NUSS_VBUSP_GAP_THRESH_REG` reader"]
pub type R = crate::R<CpswNussVbuspGapThreshRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_GAP_THRESH_REG` writer"]
pub type W = crate::W<CpswNussVbuspGapThreshRegSpec>;
#[doc = "Field `GAP_THRESH` reader - 4:0\\]
Short Gap Threshold"]
pub type GapThreshR = crate::FieldReader;
#[doc = "Field `GAP_THRESH` writer - 4:0\\]
Short Gap Threshold"]
pub type GapThreshW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Short Gap Threshold"]
    #[inline(always)]
    pub fn gap_thresh(&self) -> GapThreshR {
        GapThreshR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Short Gap Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn gap_thresh(&mut self) -> GapThreshW<CpswNussVbuspGapThreshRegSpec> {
        GapThreshW::new(self, 0)
    }
}
#[doc = "CPSW Transmit FIFO Short Gap Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_gap_thresh_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_gap_thresh_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspGapThreshRegSpec;
impl crate::RegisterSpec for CpswNussVbuspGapThreshRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_gap_thresh_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspGapThreshRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_gap_thresh_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspGapThreshRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_GAP_THRESH_REG to value 0x11"]
impl crate::Resettable for CpswNussVbuspGapThreshRegSpec {
    const RESET_VALUE: u32 = 0x11;
}
