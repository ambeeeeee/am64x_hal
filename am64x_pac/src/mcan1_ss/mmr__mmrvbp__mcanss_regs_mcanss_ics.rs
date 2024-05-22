#[doc = "Register `MMR__MMRVBP__MCANSS_REGS_MCANSS_ICS` reader"]
pub type R = crate::R<Mmr_Mmrvbp_McanssRegsMcanssIcsSpec>;
#[doc = "Register `MMR__MMRVBP__MCANSS_REGS_MCANSS_ICS` writer"]
pub type W = crate::W<Mmr_Mmrvbp_McanssRegsMcanssIcsSpec>;
#[doc = "Field `EXT_TS_CNTR_OVFL` reader - 0:0\\]
External TimeStamp Counter Overflow Interrupt status. Write '1' to clear bits."]
pub type ExtTsCntrOvflR = crate::BitReader;
#[doc = "Field `EXT_TS_CNTR_OVFL` writer - 0:0\\]
External TimeStamp Counter Overflow Interrupt status. Write '1' to clear bits."]
pub type ExtTsCntrOvflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
External TimeStamp Counter Overflow Interrupt status. Write '1' to clear bits."]
    #[inline(always)]
    pub fn ext_ts_cntr_ovfl(&self) -> ExtTsCntrOvflR {
        ExtTsCntrOvflR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
External TimeStamp Counter Overflow Interrupt status. Write '1' to clear bits."]
    #[inline(always)]
    #[must_use]
    pub fn ext_ts_cntr_ovfl(&mut self) -> ExtTsCntrOvflW<Mmr_Mmrvbp_McanssRegsMcanssIcsSpec> {
        ExtTsCntrOvflW::new(self, 0)
    }
}
#[doc = "Write to clear interrupt bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_ics::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_ics::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Mmrvbp_McanssRegsMcanssIcsSpec;
impl crate::RegisterSpec for Mmr_Mmrvbp_McanssRegsMcanssIcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__mmrvbp__mcanss_regs_mcanss_ics::R`](R) reader structure"]
impl crate::Readable for Mmr_Mmrvbp_McanssRegsMcanssIcsSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__mmrvbp__mcanss_regs_mcanss_ics::W`](W) writer structure"]
impl crate::Writable for Mmr_Mmrvbp_McanssRegsMcanssIcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__MMRVBP__MCANSS_REGS_MCANSS_ICS to value 0"]
impl crate::Resettable for Mmr_Mmrvbp_McanssRegsMcanssIcsSpec {
    const RESET_VALUE: u32 = 0;
}
