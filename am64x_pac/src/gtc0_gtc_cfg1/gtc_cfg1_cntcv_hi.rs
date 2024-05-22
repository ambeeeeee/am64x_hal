#[doc = "Register `GTC_CFG1_CNTCV_HI` reader"]
pub type R = crate::R<GtcCfg1CntcvHiSpec>;
#[doc = "Register `GTC_CFG1_CNTCV_HI` writer"]
pub type W = crate::W<GtcCfg1CntcvHiSpec>;
#[doc = "Field `CNTCV_HI_COUNTVALUE` reader - 31:0\\]
Indicates bits \\[63:32\\]
of the System Counter value."]
pub type CntcvHiCountvalueR = crate::FieldReader<u32>;
#[doc = "Field `CNTCV_HI_COUNTVALUE` writer - 31:0\\]
Indicates bits \\[63:32\\]
of the System Counter value."]
pub type CntcvHiCountvalueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates bits \\[63:32\\]
of the System Counter value."]
    #[inline(always)]
    pub fn cntcv_hi_countvalue(&self) -> CntcvHiCountvalueR {
        CntcvHiCountvalueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates bits \\[63:32\\]
of the System Counter value."]
    #[inline(always)]
    #[must_use]
    pub fn cntcv_hi_countvalue(&mut self) -> CntcvHiCountvalueW<GtcCfg1CntcvHiSpec> {
        CntcvHiCountvalueW::new(self, 0)
    }
}
#[doc = "GTC_CFG1_CNTCV_HI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg1_cntcv_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg1_cntcv_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtcCfg1CntcvHiSpec;
impl crate::RegisterSpec for GtcCfg1CntcvHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtc_cfg1_cntcv_hi::R`](R) reader structure"]
impl crate::Readable for GtcCfg1CntcvHiSpec {}
#[doc = "`write(|w| ..)` method takes [`gtc_cfg1_cntcv_hi::W`](W) writer structure"]
impl crate::Writable for GtcCfg1CntcvHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTC_CFG1_CNTCV_HI to value 0"]
impl crate::Resettable for GtcCfg1CntcvHiSpec {
    const RESET_VALUE: u32 = 0;
}
