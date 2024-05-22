#[doc = "Register `CPSW_NUSS_VBUSP_EST_TS_DOMAIN_REG` reader"]
pub type R = crate::R<CpswNussVbuspEstTsDomainRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_EST_TS_DOMAIN_REG` writer"]
pub type W = crate::W<CpswNussVbuspEstTsDomainRegSpec>;
#[doc = "Field `EST_TS_DOMAIN` reader - 7:0\\]
Enhanced Scheduled Traffic Host Event Domain"]
pub type EstTsDomainR = crate::FieldReader;
#[doc = "Field `EST_TS_DOMAIN` writer - 7:0\\]
Enhanced Scheduled Traffic Host Event Domain"]
pub type EstTsDomainW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Enhanced Scheduled Traffic Host Event Domain"]
    #[inline(always)]
    pub fn est_ts_domain(&self) -> EstTsDomainR {
        EstTsDomainR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Enhanced Scheduled Traffic Host Event Domain"]
    #[inline(always)]
    #[must_use]
    pub fn est_ts_domain(&mut self) -> EstTsDomainW<CpswNussVbuspEstTsDomainRegSpec> {
        EstTsDomainW::new(self, 0)
    }
}
#[doc = "Enhanced Scheduled Traffic Host Event Domain\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_est_ts_domain_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_est_ts_domain_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspEstTsDomainRegSpec;
impl crate::RegisterSpec for CpswNussVbuspEstTsDomainRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_est_ts_domain_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspEstTsDomainRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_est_ts_domain_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspEstTsDomainRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_EST_TS_DOMAIN_REG to value 0"]
impl crate::Resettable for CpswNussVbuspEstTsDomainRegSpec {
    const RESET_VALUE: u32 = 0;
}
