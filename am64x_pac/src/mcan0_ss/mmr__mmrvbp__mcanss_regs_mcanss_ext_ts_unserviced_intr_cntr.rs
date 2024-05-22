#[doc = "Register `MMR__MMRVBP__MCANSS_REGS_MCANSS_EXT_TS_UNSERVICED_INTR_CNTR` reader"]
pub type R = crate::R<Mmr_Mmrvbp_McanssRegsMcanssExtTsUnservicedIntrCntrSpec>;
#[doc = "Register `MMR__MMRVBP__MCANSS_REGS_MCANSS_EXT_TS_UNSERVICED_INTR_CNTR` writer"]
pub type W = crate::W<Mmr_Mmrvbp_McanssRegsMcanssExtTsUnservicedIntrCntrSpec>;
#[doc = "Field `EXT_TS_INTR_CNTR` reader - 4:0\\]
Number of unserviced rollover interrupts. If >1 an EOI write will issue another pulse interrupt"]
pub type ExtTsIntrCntrR = crate::FieldReader;
#[doc = "Field `EXT_TS_INTR_CNTR` writer - 4:0\\]
Number of unserviced rollover interrupts. If >1 an EOI write will issue another pulse interrupt"]
pub type ExtTsIntrCntrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Number of unserviced rollover interrupts. If >1 an EOI write will issue another pulse interrupt"]
    #[inline(always)]
    pub fn ext_ts_intr_cntr(&self) -> ExtTsIntrCntrR {
        ExtTsIntrCntrR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Number of unserviced rollover interrupts. If >1 an EOI write will issue another pulse interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ext_ts_intr_cntr(
        &mut self,
    ) -> ExtTsIntrCntrW<Mmr_Mmrvbp_McanssRegsMcanssExtTsUnservicedIntrCntrSpec> {
        ExtTsIntrCntrW::new(self, 0)
    }
}
#[doc = "External TImeStamp Unserviced Interrupts Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_unserviced_intr_cntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_unserviced_intr_cntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Mmrvbp_McanssRegsMcanssExtTsUnservicedIntrCntrSpec;
impl crate::RegisterSpec for Mmr_Mmrvbp_McanssRegsMcanssExtTsUnservicedIntrCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_unserviced_intr_cntr::R`](R) reader structure"]
impl crate::Readable for Mmr_Mmrvbp_McanssRegsMcanssExtTsUnservicedIntrCntrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__mmrvbp__mcanss_regs_mcanss_ext_ts_unserviced_intr_cntr::W`](W) writer structure"]
impl crate::Writable for Mmr_Mmrvbp_McanssRegsMcanssExtTsUnservicedIntrCntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__MMRVBP__MCANSS_REGS_MCANSS_EXT_TS_UNSERVICED_INTR_CNTR to value 0"]
impl crate::Resettable for Mmr_Mmrvbp_McanssRegsMcanssExtTsUnservicedIntrCntrSpec {
    const RESET_VALUE: u32 = 0;
}
