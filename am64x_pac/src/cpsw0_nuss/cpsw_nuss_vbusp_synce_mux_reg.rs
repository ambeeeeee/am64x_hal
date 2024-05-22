#[doc = "Register `CPSW_NUSS_VBUSP_SYNCE_MUX_REG` reader"]
pub type R = crate::R<CpswNussVbuspSynceMuxRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_SYNCE_MUX_REG` writer"]
pub type W = crate::W<CpswNussVbuspSynceMuxRegSpec>;
#[doc = "Field `SYNCE_SEL` reader - 5:0\\]
Sync E Select Value"]
pub type SynceSelR = crate::FieldReader;
#[doc = "Field `SYNCE_SEL` writer - 5:0\\]
Sync E Select Value"]
pub type SynceSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Sync E Select Value"]
    #[inline(always)]
    pub fn synce_sel(&self) -> SynceSelR {
        SynceSelR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Sync E Select Value"]
    #[inline(always)]
    #[must_use]
    pub fn synce_sel(&mut self) -> SynceSelW<CpswNussVbuspSynceMuxRegSpec> {
        SynceSelW::new(self, 0)
    }
}
#[doc = "SyncE Mux Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_synce_mux_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_synce_mux_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspSynceMuxRegSpec;
impl crate::RegisterSpec for CpswNussVbuspSynceMuxRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_synce_mux_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspSynceMuxRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_synce_mux_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspSynceMuxRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_SYNCE_MUX_REG to value 0"]
impl crate::Resettable for CpswNussVbuspSynceMuxRegSpec {
    const RESET_VALUE: u32 = 0;
}
