#[doc = "Register `CPSW_NUSS_VBUSP_FREQUENCY_REG` reader"]
pub type R = crate::R<CpswNussVbuspFrequencyRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_FREQUENCY_REG` writer"]
pub type W = crate::W<CpswNussVbuspFrequencyRegSpec>;
#[doc = "Field `CUT_THRESH` reader - 9:0\\]
CPSW CPPI_CLK Frequency in Mhz"]
pub type CutThreshR = crate::FieldReader<u16>;
#[doc = "Field `CUT_THRESH` writer - 9:0\\]
CPSW CPPI_CLK Frequency in Mhz"]
pub type CutThreshW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
CPSW CPPI_CLK Frequency in Mhz"]
    #[inline(always)]
    pub fn cut_thresh(&self) -> CutThreshR {
        CutThreshR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
CPSW CPPI_CLK Frequency in Mhz"]
    #[inline(always)]
    #[must_use]
    pub fn cut_thresh(&mut self) -> CutThreshW<CpswNussVbuspFrequencyRegSpec> {
        CutThreshW::new(self, 0)
    }
}
#[doc = "CPSW CPPI_CLK Frequency in Mhz\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_frequency_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_frequency_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspFrequencyRegSpec;
impl crate::RegisterSpec for CpswNussVbuspFrequencyRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_frequency_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspFrequencyRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_frequency_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspFrequencyRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_FREQUENCY_REG to value 0"]
impl crate::Resettable for CpswNussVbuspFrequencyRegSpec {
    const RESET_VALUE: u32 = 0;
}
