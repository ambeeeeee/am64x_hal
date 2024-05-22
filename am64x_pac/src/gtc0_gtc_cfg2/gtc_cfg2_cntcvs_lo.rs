#[doc = "Register `GTC_CFG2_CNTCVS_LO` reader"]
pub type R = crate::R<GtcCfg2CntcvsLoSpec>;
#[doc = "Register `GTC_CFG2_CNTCVS_LO` writer"]
pub type W = crate::W<GtcCfg2CntcvsLoSpec>;
#[doc = "Field `CNTCVS_LO_COUNTVALUE` reader - 31:0\\]
Indicates bits \\[31:0\\]
of the System Counter value."]
pub type CntcvsLoCountvalueR = crate::FieldReader<u32>;
#[doc = "Field `CNTCVS_LO_COUNTVALUE` writer - 31:0\\]
Indicates bits \\[31:0\\]
of the System Counter value."]
pub type CntcvsLoCountvalueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates bits \\[31:0\\]
of the System Counter value."]
    #[inline(always)]
    pub fn cntcvs_lo_countvalue(&self) -> CntcvsLoCountvalueR {
        CntcvsLoCountvalueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates bits \\[31:0\\]
of the System Counter value."]
    #[inline(always)]
    #[must_use]
    pub fn cntcvs_lo_countvalue(&mut self) -> CntcvsLoCountvalueW<GtcCfg2CntcvsLoSpec> {
        CntcvsLoCountvalueW::new(self, 0)
    }
}
#[doc = "GTC_CFG2_CNTCVS_LO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg2_cntcvs_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg2_cntcvs_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtcCfg2CntcvsLoSpec;
impl crate::RegisterSpec for GtcCfg2CntcvsLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtc_cfg2_cntcvs_lo::R`](R) reader structure"]
impl crate::Readable for GtcCfg2CntcvsLoSpec {}
#[doc = "`write(|w| ..)` method takes [`gtc_cfg2_cntcvs_lo::W`](W) writer structure"]
impl crate::Writable for GtcCfg2CntcvsLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTC_CFG2_CNTCVS_LO to value 0"]
impl crate::Resettable for GtcCfg2CntcvsLoSpec {
    const RESET_VALUE: u32 = 0;
}
