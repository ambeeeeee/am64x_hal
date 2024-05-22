#[doc = "Register `GTC_CFG1_CNTFID0` reader"]
pub type R = crate::R<GtcCfg1Cntfid0Spec>;
#[doc = "Register `GTC_CFG1_CNTFID0` writer"]
pub type W = crate::W<GtcCfg1Cntfid0Spec>;
#[doc = "Field `CNTFID0_FREQVALUE` reader - 31:0\\]
Indicates the base update frequency of the System Counter in Hz."]
pub type Cntfid0FreqvalueR = crate::FieldReader<u32>;
#[doc = "Field `CNTFID0_FREQVALUE` writer - 31:0\\]
Indicates the base update frequency of the System Counter in Hz."]
pub type Cntfid0FreqvalueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the base update frequency of the System Counter in Hz."]
    #[inline(always)]
    pub fn cntfid0_freqvalue(&self) -> Cntfid0FreqvalueR {
        Cntfid0FreqvalueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the base update frequency of the System Counter in Hz."]
    #[inline(always)]
    #[must_use]
    pub fn cntfid0_freqvalue(&mut self) -> Cntfid0FreqvalueW<GtcCfg1Cntfid0Spec> {
        Cntfid0FreqvalueW::new(self, 0)
    }
}
#[doc = "GTC_CFG1_CNTFID0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg1_cntfid0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg1_cntfid0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtcCfg1Cntfid0Spec;
impl crate::RegisterSpec for GtcCfg1Cntfid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtc_cfg1_cntfid0::R`](R) reader structure"]
impl crate::Readable for GtcCfg1Cntfid0Spec {}
#[doc = "`write(|w| ..)` method takes [`gtc_cfg1_cntfid0::W`](W) writer structure"]
impl crate::Writable for GtcCfg1Cntfid0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTC_CFG1_CNTFID0 to value 0"]
impl crate::Resettable for GtcCfg1Cntfid0Spec {
    const RESET_VALUE: u32 = 0;
}
