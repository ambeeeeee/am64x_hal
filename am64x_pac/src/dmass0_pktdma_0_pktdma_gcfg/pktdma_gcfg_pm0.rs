#[doc = "Register `PKTDMA_GCFG_PM0` reader"]
pub type R = crate::R<PktdmaGcfgPm0Spec>;
#[doc = "Register `PKTDMA_GCFG_PM0` writer"]
pub type W = crate::W<PktdmaGcfgPm0Spec>;
#[doc = "Field `NOGATE_CARB` reader - 0:0\\]
When set inhibits automatic gating of clock."]
pub type NogateCarbR = crate::BitReader;
#[doc = "Field `NOGATE_CARB` writer - 0:0\\]
When set inhibits automatic gating of clock."]
pub type NogateCarbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD1` reader - 3:1\\]
Reserved PM signals."]
pub type NogateRsvd1R = crate::FieldReader;
#[doc = "Field `NOGATE_RSVD1` writer - 3:1\\]
Reserved PM signals."]
pub type NogateRsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NOGATE_WARB` reader - 4:4\\]
When set inhibits automatic gating of clock."]
pub type NogateWarbR = crate::BitReader;
#[doc = "Field `NOGATE_WARB` writer - 4:4\\]
When set inhibits automatic gating of clock."]
pub type NogateWarbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD2` reader - 7:5\\]
Reserved PM signals."]
pub type NogateRsvd2R = crate::FieldReader;
#[doc = "Field `NOGATE_RSVD2` writer - 7:5\\]
Reserved PM signals."]
pub type NogateRsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NOGATE_SDEC` reader - 8:8\\]
When set inhibits automatic gating of clock."]
pub type NogateSdecR = crate::BitReader;
#[doc = "Field `NOGATE_SDEC` writer - 8:8\\]
When set inhibits automatic gating of clock."]
pub type NogateSdecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD3` reader - 11:9\\]
Reserved PM signals."]
pub type NogateRsvd3R = crate::FieldReader;
#[doc = "Field `NOGATE_RSVD3` writer - 11:9\\]
Reserved PM signals."]
pub type NogateRsvd3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NOGATE_RDEC` reader - 12:12\\]
When set inhibits automatic gating of clock."]
pub type NogateRdecR = crate::BitReader;
#[doc = "Field `NOGATE_RDEC` writer - 12:12\\]
When set inhibits automatic gating of clock."]
pub type NogateRdecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD4` reader - 23:13\\]
Reserved PM signals."]
pub type NogateRsvd4R = crate::FieldReader<u16>;
#[doc = "Field `NOGATE_RSVD4` writer - 23:13\\]
Reserved PM signals."]
pub type NogateRsvd4W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NOGATE_TDU0` reader - 24:24\\]
When set inhibits automatic gating of clock."]
pub type NogateTdu0R = crate::BitReader;
#[doc = "Field `NOGATE_TDU0` writer - 24:24\\]
When set inhibits automatic gating of clock."]
pub type NogateTdu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_TDU1` reader - 25:25\\]
When set inhibits automatic gating of clock."]
pub type NogateTdu1R = crate::BitReader;
#[doc = "Field `NOGATE_TDU1` writer - 25:25\\]
When set inhibits automatic gating of clock."]
pub type NogateTdu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_TDU2` reader - 26:26\\]
When set inhibits automatic gating of clock."]
pub type NogateTdu2R = crate::BitReader;
#[doc = "Field `NOGATE_TDU2` writer - 26:26\\]
When set inhibits automatic gating of clock."]
pub type NogateTdu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_TDU3` reader - 27:27\\]
When set inhibits automatic gating of clock."]
pub type NogateTdu3R = crate::BitReader;
#[doc = "Field `NOGATE_TDU3` writer - 27:27\\]
When set inhibits automatic gating of clock."]
pub type NogateTdu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RDU0` reader - 28:28\\]
When set inhibits automatic gating of clock."]
pub type NogateRdu0R = crate::BitReader;
#[doc = "Field `NOGATE_RDU0` writer - 28:28\\]
When set inhibits automatic gating of clock."]
pub type NogateRdu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RDU1` reader - 29:29\\]
When set inhibits automatic gating of clock."]
pub type NogateRdu1R = crate::BitReader;
#[doc = "Field `NOGATE_RDU1` writer - 29:29\\]
When set inhibits automatic gating of clock."]
pub type NogateRdu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RDU2` reader - 30:30\\]
When set inhibits automatic gating of clock."]
pub type NogateRdu2R = crate::BitReader;
#[doc = "Field `NOGATE_RDU2` writer - 30:30\\]
When set inhibits automatic gating of clock."]
pub type NogateRdu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RDU3` reader - 31:31\\]
When set inhibits automatic gating of clock."]
pub type NogateRdu3R = crate::BitReader;
#[doc = "Field `NOGATE_RDU3` writer - 31:31\\]
When set inhibits automatic gating of clock."]
pub type NogateRdu3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_carb(&self) -> NogateCarbR {
        NogateCarbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd1(&self) -> NogateRsvd1R {
        NogateRsvd1R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_warb(&self) -> NogateWarbR {
        NogateWarbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd2(&self) -> NogateRsvd2R {
        NogateRsvd2R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_sdec(&self) -> NogateSdecR {
        NogateSdecR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd3(&self) -> NogateRsvd3R {
        NogateRsvd3R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_rdec(&self) -> NogateRdecR {
        NogateRdecR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:23 - 23:13\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd4(&self) -> NogateRsvd4R {
        NogateRsvd4R::new(((self.bits >> 13) & 0x07ff) as u16)
    }
    #[doc = "Bit 24 - 24:24\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_tdu0(&self) -> NogateTdu0R {
        NogateTdu0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_tdu1(&self) -> NogateTdu1R {
        NogateTdu1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_tdu2(&self) -> NogateTdu2R {
        NogateTdu2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_tdu3(&self) -> NogateTdu3R {
        NogateTdu3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_rdu0(&self) -> NogateRdu0R {
        NogateRdu0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_rdu1(&self) -> NogateRdu1R {
        NogateRdu1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_rdu2(&self) -> NogateRdu2R {
        NogateRdu2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_rdu3(&self) -> NogateRdu3R {
        NogateRdu3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_carb(&mut self) -> NogateCarbW<PktdmaGcfgPm0Spec> {
        NogateCarbW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd1(&mut self) -> NogateRsvd1W<PktdmaGcfgPm0Spec> {
        NogateRsvd1W::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_warb(&mut self) -> NogateWarbW<PktdmaGcfgPm0Spec> {
        NogateWarbW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd2(&mut self) -> NogateRsvd2W<PktdmaGcfgPm0Spec> {
        NogateRsvd2W::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_sdec(&mut self) -> NogateSdecW<PktdmaGcfgPm0Spec> {
        NogateSdecW::new(self, 8)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd3(&mut self) -> NogateRsvd3W<PktdmaGcfgPm0Spec> {
        NogateRsvd3W::new(self, 9)
    }
    #[doc = "Bit 12 - 12:12\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rdec(&mut self) -> NogateRdecW<PktdmaGcfgPm0Spec> {
        NogateRdecW::new(self, 12)
    }
    #[doc = "Bits 13:23 - 23:13\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd4(&mut self) -> NogateRsvd4W<PktdmaGcfgPm0Spec> {
        NogateRsvd4W::new(self, 13)
    }
    #[doc = "Bit 24 - 24:24\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_tdu0(&mut self) -> NogateTdu0W<PktdmaGcfgPm0Spec> {
        NogateTdu0W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_tdu1(&mut self) -> NogateTdu1W<PktdmaGcfgPm0Spec> {
        NogateTdu1W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_tdu2(&mut self) -> NogateTdu2W<PktdmaGcfgPm0Spec> {
        NogateTdu2W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_tdu3(&mut self) -> NogateTdu3W<PktdmaGcfgPm0Spec> {
        NogateTdu3W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rdu0(&mut self) -> NogateRdu0W<PktdmaGcfgPm0Spec> {
        NogateRdu0W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rdu1(&mut self) -> NogateRdu1W<PktdmaGcfgPm0Spec> {
        NogateRdu1W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rdu2(&mut self) -> NogateRdu2W<PktdmaGcfgPm0Spec> {
        NogateRdu2W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rdu3(&mut self) -> NogateRdu3W<PktdmaGcfgPm0Spec> {
        NogateRdu3W::new(self, 31)
    }
}
#[doc = "This register enables or inhibits automatic clock gating to individual sub-blocks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_pm0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_pm0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaGcfgPm0Spec;
impl crate::RegisterSpec for PktdmaGcfgPm0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_gcfg_pm0::R`](R) reader structure"]
impl crate::Readable for PktdmaGcfgPm0Spec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_gcfg_pm0::W`](W) writer structure"]
impl crate::Writable for PktdmaGcfgPm0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_GCFG_PM0 to value 0"]
impl crate::Resettable for PktdmaGcfgPm0Spec {
    const RESET_VALUE: u32 = 0;
}
