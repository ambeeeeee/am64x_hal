#[doc = "Register `BCDMA_GCFG_CAP3` reader"]
pub type R = crate::R<BcdmaGcfgCap3Spec>;
#[doc = "Register `BCDMA_GCFG_CAP3` writer"]
pub type W = crate::W<BcdmaGcfgCap3Spec>;
#[doc = "Field `HCHAN_CNT` reader - 22:14\\]
BC high capacity internal channel count"]
pub type HchanCntR = crate::FieldReader<u16>;
#[doc = "Field `HCHAN_CNT` writer - 22:14\\]
BC high capacity internal channel count"]
pub type HchanCntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `UCHAN_CNT` reader - 31:23\\]
BC ultra high capacity internal channel count"]
pub type UchanCntR = crate::FieldReader<u16>;
#[doc = "Field `UCHAN_CNT` writer - 31:23\\]
BC ultra high capacity internal channel count"]
pub type UchanCntW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 14:22 - 22:14\\]
BC high capacity internal channel count"]
    #[inline(always)]
    pub fn hchan_cnt(&self) -> HchanCntR {
        HchanCntR::new(((self.bits >> 14) & 0x01ff) as u16)
    }
    #[doc = "Bits 23:31 - 31:23\\]
BC ultra high capacity internal channel count"]
    #[inline(always)]
    pub fn uchan_cnt(&self) -> UchanCntR {
        UchanCntR::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 14:22 - 22:14\\]
BC high capacity internal channel count"]
    #[inline(always)]
    #[must_use]
    pub fn hchan_cnt(&mut self) -> HchanCntW<BcdmaGcfgCap3Spec> {
        HchanCntW::new(self, 14)
    }
    #[doc = "Bits 23:31 - 31:23\\]
BC ultra high capacity internal channel count"]
    #[inline(always)]
    #[must_use]
    pub fn uchan_cnt(&mut self) -> UchanCntW<BcdmaGcfgCap3Spec> {
        UchanCntW::new(self, 23)
    }
}
#[doc = "The Capabilities Register 3 specifies how many resources this BCDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_cap3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_cap3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaGcfgCap3Spec;
impl crate::RegisterSpec for BcdmaGcfgCap3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_gcfg_cap3::R`](R) reader structure"]
impl crate::Readable for BcdmaGcfgCap3Spec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_gcfg_cap3::W`](W) writer structure"]
impl crate::Writable for BcdmaGcfgCap3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_GCFG_CAP3 to value 0"]
impl crate::Resettable for BcdmaGcfgCap3Spec {
    const RESET_VALUE: u32 = 0;
}
