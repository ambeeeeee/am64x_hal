#[doc = "Register `GTC_CFG1_CNTCV_LO` reader"]
pub type R = crate::R<GtcCfg1CntcvLoSpec>;
#[doc = "Register `GTC_CFG1_CNTCV_LO` writer"]
pub type W = crate::W<GtcCfg1CntcvLoSpec>;
#[doc = "Field `CNTCV_LO_COUNTVALUE` reader - 31:0\\]
Indicates bits \\[31:0\\]
of the System Counter value."]
pub type CntcvLoCountvalueR = crate::FieldReader<u32>;
#[doc = "Field `CNTCV_LO_COUNTVALUE` writer - 31:0\\]
Indicates bits \\[31:0\\]
of the System Counter value."]
pub type CntcvLoCountvalueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates bits \\[31:0\\]
of the System Counter value."]
    #[inline(always)]
    pub fn cntcv_lo_countvalue(&self) -> CntcvLoCountvalueR {
        CntcvLoCountvalueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates bits \\[31:0\\]
of the System Counter value."]
    #[inline(always)]
    #[must_use]
    pub fn cntcv_lo_countvalue(&mut self) -> CntcvLoCountvalueW<GtcCfg1CntcvLoSpec> {
        CntcvLoCountvalueW::new(self, 0)
    }
}
#[doc = "GTC_CFG1_CNTCV_LO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg1_cntcv_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg1_cntcv_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtcCfg1CntcvLoSpec;
impl crate::RegisterSpec for GtcCfg1CntcvLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtc_cfg1_cntcv_lo::R`](R) reader structure"]
impl crate::Readable for GtcCfg1CntcvLoSpec {}
#[doc = "`write(|w| ..)` method takes [`gtc_cfg1_cntcv_lo::W`](W) writer structure"]
impl crate::Writable for GtcCfg1CntcvLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTC_CFG1_CNTCV_LO to value 0"]
impl crate::Resettable for GtcCfg1CntcvLoSpec {
    const RESET_VALUE: u32 = 0;
}
