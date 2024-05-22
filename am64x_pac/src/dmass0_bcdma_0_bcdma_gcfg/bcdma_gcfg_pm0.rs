#[doc = "Register `BCDMA_GCFG_PM0` reader"]
pub type R = crate::R<BcdmaGcfgPm0Spec>;
#[doc = "Register `BCDMA_GCFG_PM0` writer"]
pub type W = crate::W<BcdmaGcfgPm0Spec>;
#[doc = "Field `NOGATE_RSVD0` reader - 1:0\\]
Reserved PM signals."]
pub type NogateRsvd0R = crate::FieldReader;
#[doc = "Field `NOGATE_RSVD0` writer - 1:0\\]
Reserved PM signals."]
pub type NogateRsvd0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOGATE_CARB2` reader - 2:2\\]
When set inhibits automatic gating of clock."]
pub type NogateCarb2R = crate::BitReader;
#[doc = "Field `NOGATE_CARB2` writer - 2:2\\]
When set inhibits automatic gating of clock."]
pub type NogateCarb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_CARB3` reader - 3:3\\]
When set inhibits automatic gating of clock."]
pub type NogateCarb3R = crate::BitReader;
#[doc = "Field `NOGATE_CARB3` writer - 3:3\\]
When set inhibits automatic gating of clock."]
pub type NogateCarb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD1` reader - 6:4\\]
Reserved PM signals."]
pub type NogateRsvd1R = crate::FieldReader;
#[doc = "Field `NOGATE_RSVD1` writer - 6:4\\]
Reserved PM signals."]
pub type NogateRsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NOGATE_WARB3` reader - 7:7\\]
When set inhibits automatic gating of clock."]
pub type NogateWarb3R = crate::BitReader;
#[doc = "Field `NOGATE_WARB3` writer - 7:7\\]
When set inhibits automatic gating of clock."]
pub type NogateWarb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD2` reader - 10:8\\]
Reserved PM signals."]
pub type NogateRsvd2R = crate::FieldReader;
#[doc = "Field `NOGATE_RSVD2` writer - 10:8\\]
Reserved PM signals."]
pub type NogateRsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NOGATE_SDEC3` reader - 11:11\\]
When set inhibits automatic gating of clock."]
pub type NogateSdec3R = crate::BitReader;
#[doc = "Field `NOGATE_SDEC3` writer - 11:11\\]
When set inhibits automatic gating of clock."]
pub type NogateSdec3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD3` reader - 13:12\\]
Reserved PM signals."]
pub type NogateRsvd3R = crate::FieldReader;
#[doc = "Field `NOGATE_RSVD3` writer - 13:12\\]
Reserved PM signals."]
pub type NogateRsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOGATE_RDEC2` reader - 14:14\\]
When set inhibits automatic gating of clock."]
pub type NogateRdec2R = crate::BitReader;
#[doc = "Field `NOGATE_RDEC2` writer - 14:14\\]
When set inhibits automatic gating of clock."]
pub type NogateRdec2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD4` reader - 31:15\\]
Reserved PM signals."]
pub type NogateRsvd4R = crate::FieldReader<u32>;
#[doc = "Field `NOGATE_RSVD4` writer - 31:15\\]
Reserved PM signals."]
pub type NogateRsvd4W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd0(&self) -> NogateRsvd0R {
        NogateRsvd0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_carb2(&self) -> NogateCarb2R {
        NogateCarb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_carb3(&self) -> NogateCarb3R {
        NogateCarb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd1(&self) -> NogateRsvd1R {
        NogateRsvd1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_warb3(&self) -> NogateWarb3R {
        NogateWarb3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd2(&self) -> NogateRsvd2R {
        NogateRsvd2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_sdec3(&self) -> NogateSdec3R {
        NogateSdec3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd3(&self) -> NogateRsvd3R {
        NogateRsvd3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_rdec2(&self) -> NogateRdec2R {
        NogateRdec2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd4(&self) -> NogateRsvd4R {
        NogateRsvd4R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd0(&mut self) -> NogateRsvd0W<BcdmaGcfgPm0Spec> {
        NogateRsvd0W::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_carb2(&mut self) -> NogateCarb2W<BcdmaGcfgPm0Spec> {
        NogateCarb2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_carb3(&mut self) -> NogateCarb3W<BcdmaGcfgPm0Spec> {
        NogateCarb3W::new(self, 3)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd1(&mut self) -> NogateRsvd1W<BcdmaGcfgPm0Spec> {
        NogateRsvd1W::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_warb3(&mut self) -> NogateWarb3W<BcdmaGcfgPm0Spec> {
        NogateWarb3W::new(self, 7)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd2(&mut self) -> NogateRsvd2W<BcdmaGcfgPm0Spec> {
        NogateRsvd2W::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_sdec3(&mut self) -> NogateSdec3W<BcdmaGcfgPm0Spec> {
        NogateSdec3W::new(self, 11)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd3(&mut self) -> NogateRsvd3W<BcdmaGcfgPm0Spec> {
        NogateRsvd3W::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rdec2(&mut self) -> NogateRdec2W<BcdmaGcfgPm0Spec> {
        NogateRdec2W::new(self, 14)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd4(&mut self) -> NogateRsvd4W<BcdmaGcfgPm0Spec> {
        NogateRsvd4W::new(self, 15)
    }
}
#[doc = "This register enables or inhibits automatic clock gating to individual sub-blocks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_pm0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_pm0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaGcfgPm0Spec;
impl crate::RegisterSpec for BcdmaGcfgPm0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_gcfg_pm0::R`](R) reader structure"]
impl crate::Readable for BcdmaGcfgPm0Spec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_gcfg_pm0::W`](W) writer structure"]
impl crate::Writable for BcdmaGcfgPm0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_GCFG_PM0 to value 0"]
impl crate::Resettable for BcdmaGcfgPm0Spec {
    const RESET_VALUE: u32 = 0;
}
