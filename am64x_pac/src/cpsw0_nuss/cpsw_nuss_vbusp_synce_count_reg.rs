#[doc = "Register `CPSW_NUSS_VBUSP_SYNCE_COUNT_REG` reader"]
pub type R = crate::R<CpswNussVbuspSynceCountRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_SYNCE_COUNT_REG` writer"]
pub type W = crate::W<CpswNussVbuspSynceCountRegSpec>;
#[doc = "Field `SYNCE_CNT` reader - 31:0\\]
Sync E Count Value"]
pub type SynceCntR = crate::FieldReader<u32>;
#[doc = "Field `SYNCE_CNT` writer - 31:0\\]
Sync E Count Value"]
pub type SynceCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Sync E Count Value"]
    #[inline(always)]
    pub fn synce_cnt(&self) -> SynceCntR {
        SynceCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Sync E Count Value"]
    #[inline(always)]
    #[must_use]
    pub fn synce_cnt(&mut self) -> SynceCntW<CpswNussVbuspSynceCountRegSpec> {
        SynceCntW::new(self, 0)
    }
}
#[doc = "SyncE Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_synce_count_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_synce_count_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspSynceCountRegSpec;
impl crate::RegisterSpec for CpswNussVbuspSynceCountRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_synce_count_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspSynceCountRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_synce_count_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspSynceCountRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_SYNCE_COUNT_REG to value 0"]
impl crate::Resettable for CpswNussVbuspSynceCountRegSpec {
    const RESET_VALUE: u32 = 0;
}
