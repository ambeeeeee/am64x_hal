#[doc = "Register `MMR__MMRVBP__MCANSS_REGS_MCANSS_EOI` reader"]
pub type R = crate::R<Mmr_Mmrvbp_McanssRegsMcanssEoiSpec>;
#[doc = "Register `MMR__MMRVBP__MCANSS_REGS_MCANSS_EOI` writer"]
pub type W = crate::W<Mmr_Mmrvbp_McanssRegsMcanssEoiSpec>;
#[doc = "Field `EOI` reader - 7:0\\]
Write with bit position of targeted interrupt. (E.g. Ext TS is bit 0). Upon write, level interrupt will clear and if unserviced interrupt counter > 1 will issue another pulse interrupt"]
pub type EoiR = crate::FieldReader;
#[doc = "Field `EOI` writer - 7:0\\]
Write with bit position of targeted interrupt. (E.g. Ext TS is bit 0). Upon write, level interrupt will clear and if unserviced interrupt counter > 1 will issue another pulse interrupt"]
pub type EoiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Write with bit position of targeted interrupt. (E.g. Ext TS is bit 0). Upon write, level interrupt will clear and if unserviced interrupt counter > 1 will issue another pulse interrupt"]
    #[inline(always)]
    pub fn eoi(&self) -> EoiR {
        EoiR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Write with bit position of targeted interrupt. (E.g. Ext TS is bit 0). Upon write, level interrupt will clear and if unserviced interrupt counter > 1 will issue another pulse interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eoi(&mut self) -> EoiW<Mmr_Mmrvbp_McanssRegsMcanssEoiSpec> {
        EoiW::new(self, 0)
    }
}
#[doc = "End of Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_eoi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_eoi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Mmrvbp_McanssRegsMcanssEoiSpec;
impl crate::RegisterSpec for Mmr_Mmrvbp_McanssRegsMcanssEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__mmrvbp__mcanss_regs_mcanss_eoi::R`](R) reader structure"]
impl crate::Readable for Mmr_Mmrvbp_McanssRegsMcanssEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__mmrvbp__mcanss_regs_mcanss_eoi::W`](W) writer structure"]
impl crate::Writable for Mmr_Mmrvbp_McanssRegsMcanssEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__MMRVBP__MCANSS_REGS_MCANSS_EOI to value 0"]
impl crate::Resettable for Mmr_Mmrvbp_McanssRegsMcanssEoiSpec {
    const RESET_VALUE: u32 = 0;
}
