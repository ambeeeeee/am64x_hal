#[doc = "Register `BCDMA_GCFG_CAP2` reader"]
pub type R = crate::R<BcdmaGcfgCap2Spec>;
#[doc = "Register `BCDMA_GCFG_CAP2` writer"]
pub type W = crate::W<BcdmaGcfgCap2Spec>;
#[doc = "Field `CHAN_CNT` reader - 8:0\\]
BC channel count"]
pub type ChanCntR = crate::FieldReader<u16>;
#[doc = "Field `CHAN_CNT` writer - 8:0\\]
BC channel count"]
pub type ChanCntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TCHAN_CNT` reader - 17:9\\]
Tx split channel count"]
pub type TchanCntR = crate::FieldReader<u16>;
#[doc = "Field `TCHAN_CNT` writer - 17:9\\]
Tx split channel count"]
pub type TchanCntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RCHAN_CNT` reader - 26:18\\]
Rx split channel count"]
pub type RchanCntR = crate::FieldReader<u16>;
#[doc = "Field `RCHAN_CNT` writer - 26:18\\]
Rx split channel count"]
pub type RchanCntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
BC channel count"]
    #[inline(always)]
    pub fn chan_cnt(&self) -> ChanCntR {
        ChanCntR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - 17:9\\]
Tx split channel count"]
    #[inline(always)]
    pub fn tchan_cnt(&self) -> TchanCntR {
        TchanCntR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
    #[doc = "Bits 18:26 - 26:18\\]
Rx split channel count"]
    #[inline(always)]
    pub fn rchan_cnt(&self) -> RchanCntR {
        RchanCntR::new(((self.bits >> 18) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
BC channel count"]
    #[inline(always)]
    #[must_use]
    pub fn chan_cnt(&mut self) -> ChanCntW<BcdmaGcfgCap2Spec> {
        ChanCntW::new(self, 0)
    }
    #[doc = "Bits 9:17 - 17:9\\]
Tx split channel count"]
    #[inline(always)]
    #[must_use]
    pub fn tchan_cnt(&mut self) -> TchanCntW<BcdmaGcfgCap2Spec> {
        TchanCntW::new(self, 9)
    }
    #[doc = "Bits 18:26 - 26:18\\]
Rx split channel count"]
    #[inline(always)]
    #[must_use]
    pub fn rchan_cnt(&mut self) -> RchanCntW<BcdmaGcfgCap2Spec> {
        RchanCntW::new(self, 18)
    }
}
#[doc = "The Capabilities Register 2 specifies how many resources this BCDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_cap2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_cap2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaGcfgCap2Spec;
impl crate::RegisterSpec for BcdmaGcfgCap2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_gcfg_cap2::R`](R) reader structure"]
impl crate::Readable for BcdmaGcfgCap2Spec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_gcfg_cap2::W`](W) writer structure"]
impl crate::Writable for BcdmaGcfgCap2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_GCFG_CAP2 to value 0x0080_4028"]
impl crate::Resettable for BcdmaGcfgCap2Spec {
    const RESET_VALUE: u32 = 0x0080_4028;
}
