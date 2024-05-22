#[doc = "Register `GTC_CFG1_CNTCR` reader"]
pub type R = crate::R<GtcCfg1CntcrSpec>;
#[doc = "Register `GTC_CFG1_CNTCR` writer"]
pub type W = crate::W<GtcCfg1CntcrSpec>;
#[doc = "Field `CNTCR_EN` reader - 0:0\\]
Enable System Counter"]
pub type CntcrEnR = crate::BitReader;
#[doc = "Field `CNTCR_EN` writer - 0:0\\]
Enable System Counter"]
pub type CntcrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTCR_HDBG` reader - 1:1\\]
Halt on Debug"]
pub type CntcrHdbgR = crate::BitReader;
#[doc = "Field `CNTCR_HDBG` writer - 1:1\\]
Halt on Debug"]
pub type CntcrHdbgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTCR_FCREQ` reader - 31:8\\]
Frequency Change Request"]
pub type CntcrFcreqR = crate::FieldReader<u32>;
#[doc = "Field `CNTCR_FCREQ` writer - 31:8\\]
Frequency Change Request"]
pub type CntcrFcreqW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable System Counter"]
    #[inline(always)]
    pub fn cntcr_en(&self) -> CntcrEnR {
        CntcrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Halt on Debug"]
    #[inline(always)]
    pub fn cntcr_hdbg(&self) -> CntcrHdbgR {
        CntcrHdbgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Frequency Change Request"]
    #[inline(always)]
    pub fn cntcr_fcreq(&self) -> CntcrFcreqR {
        CntcrFcreqR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable System Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cntcr_en(&mut self) -> CntcrEnW<GtcCfg1CntcrSpec> {
        CntcrEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Halt on Debug"]
    #[inline(always)]
    #[must_use]
    pub fn cntcr_hdbg(&mut self) -> CntcrHdbgW<GtcCfg1CntcrSpec> {
        CntcrHdbgW::new(self, 1)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Frequency Change Request"]
    #[inline(always)]
    #[must_use]
    pub fn cntcr_fcreq(&mut self) -> CntcrFcreqW<GtcCfg1CntcrSpec> {
        CntcrFcreqW::new(self, 8)
    }
}
#[doc = "GTC_CFG1_CNTCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg1_cntcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg1_cntcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GtcCfg1CntcrSpec;
impl crate::RegisterSpec for GtcCfg1CntcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gtc_cfg1_cntcr::R`](R) reader structure"]
impl crate::Readable for GtcCfg1CntcrSpec {}
#[doc = "`write(|w| ..)` method takes [`gtc_cfg1_cntcr::W`](W) writer structure"]
impl crate::Writable for GtcCfg1CntcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GTC_CFG1_CNTCR to value 0"]
impl crate::Resettable for GtcCfg1CntcrSpec {
    const RESET_VALUE: u32 = 0;
}
