#[doc = "Register `MMR__MMRVBP__MCANSS_REGS_MCANSS_EXT_TS_PRESCALER` reader"]
pub type R = crate::R<Mmr_Mmrvbp_McanssRegsMcanssExtTsPrescalerSpec>;
#[doc = "Register `MMR__MMRVBP__MCANSS_REGS_MCANSS_EXT_TS_PRESCALER` writer"]
pub type W = crate::W<Mmr_Mmrvbp_McanssRegsMcanssExtTsPrescalerSpec>;
#[doc = "Field `PRESCALER` reader - 23:0\\]
External Timestamp Prescaler reload value. External Timestamp count rate is host clock rate divided by this value with one exception: a value of 0 has the same effect as 1"]
pub type PrescalerR = crate::FieldReader<u32>;
#[doc = "Field `PRESCALER` writer - 23:0\\]
External Timestamp Prescaler reload value. External Timestamp count rate is host clock rate divided by this value with one exception: a value of 0 has the same effect as 1"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
External Timestamp Prescaler reload value. External Timestamp count rate is host clock rate divided by this value with one exception: a value of 0 has the same effect as 1"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
External Timestamp Prescaler reload value. External Timestamp count rate is host clock rate divided by this value with one exception: a value of 0 has the same effect as 1"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PrescalerW<Mmr_Mmrvbp_McanssRegsMcanssExtTsPrescalerSpec> {
        PrescalerW::new(self, 0)
    }
}
#[doc = "External TImeStamp PreScaler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_prescaler::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_prescaler::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Mmrvbp_McanssRegsMcanssExtTsPrescalerSpec;
impl crate::RegisterSpec for Mmr_Mmrvbp_McanssRegsMcanssExtTsPrescalerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_prescaler::R`](R) reader structure"]
impl crate::Readable for Mmr_Mmrvbp_McanssRegsMcanssExtTsPrescalerSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_prescaler::W`](W) writer structure"]
impl crate::Writable for Mmr_Mmrvbp_McanssRegsMcanssExtTsPrescalerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__MMRVBP__MCANSS_REGS_MCANSS_EXT_TS_PRESCALER to value 0"]
impl crate::Resettable for Mmr_Mmrvbp_McanssRegsMcanssExtTsPrescalerSpec {
    const RESET_VALUE: u32 = 0;
}
