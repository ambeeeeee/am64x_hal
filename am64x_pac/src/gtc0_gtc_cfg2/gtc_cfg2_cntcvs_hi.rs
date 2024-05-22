#[doc = "Register `GTC_CFG2_CNTCVS_HI` reader"]
pub type R = crate::R<GtcCfg2CntcvsHiSpec>;
#[doc = "Register `GTC_CFG2_CNTCVS_HI` writer"]
pub type W = crate::W<GtcCfg2CntcvsHiSpec>;
#[doc = "Field `CNTCVS_HI_COUNTVALUE` reader - 31:0\\]
Indicates bits \\[63:32\\]
of the System Counter value."]
pub type CntcvsHiCountvalueR = crate::FieldReader<u32>;
#[doc = "Field `CNTCVS_HI_COUNTVALUE` writer - 31:0\\]
Indicates bits \\[63:32\\]
of the System Counter value."]
pub type CntcvsHiCountvalueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates bits \\[63:32\\]
of the System Counter value."]
    #[inline(always)]
    pub fn cntcvs_hi_countvalue(&self) -> CntcvsHiCountvalueR {
        CntcvsHiCountvalueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates bits \\[63:32\\]
of the System Counter value."]
    #[inline(always)]
    #[must_use]
    pub fn cntcvs_hi_countvalue(&mut self) -> CntcvsHiCountvalueW<GtcCfg2CntcvsHiSpec> {
        CntcvsHiCountvalueW::new(self, 0)
    }
}
#[doc = "GTC_CFG2_CNTCVS_HI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg2_cntcvs_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg2_cntcvs_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtcCfg2CntcvsHiSpec;
impl crate::RegisterSpec for GtcCfg2CntcvsHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtc_cfg2_cntcvs_hi::R`](R) reader structure"]
impl crate::Readable for GtcCfg2CntcvsHiSpec {}
#[doc = "`write(|w| ..)` method takes [`gtc_cfg2_cntcvs_hi::W`](W) writer structure"]
impl crate::Writable for GtcCfg2CntcvsHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTC_CFG2_CNTCVS_HI to value 0"]
impl crate::Resettable for GtcCfg2CntcvsHiSpec {
    const RESET_VALUE: u32 = 0;
}
