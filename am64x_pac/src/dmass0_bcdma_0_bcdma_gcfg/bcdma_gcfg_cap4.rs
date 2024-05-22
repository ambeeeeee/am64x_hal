#[doc = "Register `BCDMA_GCFG_CAP4` reader"]
pub type R = crate::R<BcdmaGcfgCap4Spec>;
#[doc = "Register `BCDMA_GCFG_CAP4` writer"]
pub type W = crate::W<BcdmaGcfgCap4Spec>;
#[doc = "Field `RHCHAN_CNT` reader - 7:0\\]
RX high capacity internal channel count"]
pub type RhchanCntR = crate::FieldReader;
#[doc = "Field `RHCHAN_CNT` writer - 7:0\\]
RX high capacity internal channel count"]
pub type RhchanCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RUCHAN_CNT` reader - 15:8\\]
RX ultra high capacity internal channel count"]
pub type RuchanCntR = crate::FieldReader;
#[doc = "Field `RUCHAN_CNT` writer - 15:8\\]
RX ultra high capacity internal channel count"]
pub type RuchanCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THCHAN_CNT` reader - 23:16\\]
TX high capacity internal channel count"]
pub type ThchanCntR = crate::FieldReader;
#[doc = "Field `THCHAN_CNT` writer - 23:16\\]
TX high capacity internal channel count"]
pub type ThchanCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TUCHAN_CNT` reader - 31:24\\]
TX ultra high capacity internal channel count"]
pub type TuchanCntR = crate::FieldReader;
#[doc = "Field `TUCHAN_CNT` writer - 31:24\\]
TX ultra high capacity internal channel count"]
pub type TuchanCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
RX high capacity internal channel count"]
    #[inline(always)]
    pub fn rhchan_cnt(&self) -> RhchanCntR {
        RhchanCntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
RX ultra high capacity internal channel count"]
    #[inline(always)]
    pub fn ruchan_cnt(&self) -> RuchanCntR {
        RuchanCntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TX high capacity internal channel count"]
    #[inline(always)]
    pub fn thchan_cnt(&self) -> ThchanCntR {
        ThchanCntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TX ultra high capacity internal channel count"]
    #[inline(always)]
    pub fn tuchan_cnt(&self) -> TuchanCntR {
        TuchanCntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
RX high capacity internal channel count"]
    #[inline(always)]
    #[must_use]
    pub fn rhchan_cnt(&mut self) -> RhchanCntW<BcdmaGcfgCap4Spec> {
        RhchanCntW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
RX ultra high capacity internal channel count"]
    #[inline(always)]
    #[must_use]
    pub fn ruchan_cnt(&mut self) -> RuchanCntW<BcdmaGcfgCap4Spec> {
        RuchanCntW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TX high capacity internal channel count"]
    #[inline(always)]
    #[must_use]
    pub fn thchan_cnt(&mut self) -> ThchanCntW<BcdmaGcfgCap4Spec> {
        ThchanCntW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TX ultra high capacity internal channel count"]
    #[inline(always)]
    #[must_use]
    pub fn tuchan_cnt(&mut self) -> TuchanCntW<BcdmaGcfgCap4Spec> {
        TuchanCntW::new(self, 24)
    }
}
#[doc = "The Capabilities Register 4 specifies how many resources this BCDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_cap4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_cap4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaGcfgCap4Spec;
impl crate::RegisterSpec for BcdmaGcfgCap4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_gcfg_cap4::R`](R) reader structure"]
impl crate::Readable for BcdmaGcfgCap4Spec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_gcfg_cap4::W`](W) writer structure"]
impl crate::Writable for BcdmaGcfgCap4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_GCFG_CAP4 to value 0"]
impl crate::Resettable for BcdmaGcfgCap4Spec {
    const RESET_VALUE: u32 = 0;
}
