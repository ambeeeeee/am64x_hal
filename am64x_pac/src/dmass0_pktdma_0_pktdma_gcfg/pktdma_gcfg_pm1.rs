#[doc = "Register `PKTDMA_GCFG_PM1` reader"]
pub type R = crate::R<PktdmaGcfgPm1Spec>;
#[doc = "Register `PKTDMA_GCFG_PM1` writer"]
pub type W = crate::W<PktdmaGcfgPm1Spec>;
#[doc = "Field `NOGATE_RSVD5` reader - 11:0\\]
Reserved PM signals."]
pub type NogateRsvd5R = crate::FieldReader<u16>;
#[doc = "Field `NOGATE_RSVD5` writer - 11:0\\]
Reserved PM signals."]
pub type NogateRsvd5W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `NOGATE_TCU` reader - 12:12\\]
When set inhibits automatic gating of clock."]
pub type NogateTcuR = crate::BitReader;
#[doc = "Field `NOGATE_TCU` writer - 12:12\\]
When set inhibits automatic gating of clock."]
pub type NogateTcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RCU` reader - 13:13\\]
When set inhibits automatic gating of clock."]
pub type NogateRcuR = crate::BitReader;
#[doc = "Field `NOGATE_RCU` writer - 13:13\\]
When set inhibits automatic gating of clock."]
pub type NogateRcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD6` reader - 14:14\\]
Reserved PM signals."]
pub type NogateRsvd6R = crate::BitReader;
#[doc = "Field `NOGATE_RSVD6` writer - 14:14\\]
Reserved PM signals."]
pub type NogateRsvd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RFLOWFW` reader - 15:15\\]
When set inhibits automatic gating of clock."]
pub type NogateRflowfwR = crate::BitReader;
#[doc = "Field `NOGATE_RFLOWFW` writer - 15:15\\]
When set inhibits automatic gating of clock."]
pub type NogateRflowfwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD7` reader - 17:16\\]
Reserved PM signals."]
pub type NogateRsvd7R = crate::FieldReader;
#[doc = "Field `NOGATE_RSVD7` writer - 17:16\\]
Reserved PM signals."]
pub type NogateRsvd7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOGATE_CFG` reader - 18:18\\]
When set inhibits automatic gating of clock."]
pub type NogateCfgR = crate::BitReader;
#[doc = "Field `NOGATE_CFG` writer - 18:18\\]
When set inhibits automatic gating of clock."]
pub type NogateCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD8` reader - 21:19\\]
Reserved PM signals."]
pub type NogateRsvd8R = crate::FieldReader;
#[doc = "Field `NOGATE_RSVD8` writer - 21:19\\]
Reserved PM signals."]
pub type NogateRsvd8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NOGATE_TPCF` reader - 22:22\\]
When set inhibits automatic gating of clock."]
pub type NogateTpcfR = crate::BitReader;
#[doc = "Field `NOGATE_TPCF` writer - 22:22\\]
When set inhibits automatic gating of clock."]
pub type NogateTpcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD9` reader - 23:23\\]
Reserved PM signals."]
pub type NogateRsvd9R = crate::BitReader;
#[doc = "Field `NOGATE_RSVD9` writer - 23:23\\]
Reserved PM signals."]
pub type NogateRsvd9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RINGOCC` reader - 24:24\\]
When set inhibits automatic gating of clock."]
pub type NogateRingoccR = crate::BitReader;
#[doc = "Field `NOGATE_RINGOCC` writer - 24:24\\]
When set inhibits automatic gating of clock."]
pub type NogateRingoccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_EHANDLER` reader - 25:25\\]
When set inhibits automatic gating of clock."]
pub type NogateEhandlerR = crate::BitReader;
#[doc = "Field `NOGATE_EHANDLER` writer - 25:25\\]
When set inhibits automatic gating of clock."]
pub type NogateEhandlerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD10` reader - 26:26\\]
Reserved PM signals."]
pub type NogateRsvd10R = crate::BitReader;
#[doc = "Field `NOGATE_RSVD10` writer - 26:26\\]
Reserved PM signals."]
pub type NogateRsvd10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_P2P` reader - 27:27\\]
When set inhibits automatic gating of clock."]
pub type NogateP2pR = crate::BitReader;
#[doc = "Field `NOGATE_P2P` writer - 27:27\\]
When set inhibits automatic gating of clock."]
pub type NogateP2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD11` reader - 28:28\\]
Reserved PM signals."]
pub type NogateRsvd11R = crate::BitReader;
#[doc = "Field `NOGATE_RSVD11` writer - 28:28\\]
Reserved PM signals."]
pub type NogateRsvd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_PROXY` reader - 29:29\\]
When set inhibits automatic gating of clock."]
pub type NogateProxyR = crate::BitReader;
#[doc = "Field `NOGATE_PROXY` writer - 29:29\\]
When set inhibits automatic gating of clock."]
pub type NogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_STATS` reader - 30:30\\]
When set inhibits automatic gating of clock."]
pub type NogateStatsR = crate::BitReader;
#[doc = "Field `NOGATE_STATS` writer - 30:30\\]
When set inhibits automatic gating of clock."]
pub type NogateStatsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD12` reader - 31:31\\]
Reserved PM signals."]
pub type NogateRsvd12R = crate::BitReader;
#[doc = "Field `NOGATE_RSVD12` writer - 31:31\\]
Reserved PM signals."]
pub type NogateRsvd12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd5(&self) -> NogateRsvd5R {
        NogateRsvd5R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - 12:12\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_tcu(&self) -> NogateTcuR {
        NogateTcuR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_rcu(&self) -> NogateRcuR {
        NogateRcuR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd6(&self) -> NogateRsvd6R {
        NogateRsvd6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_rflowfw(&self) -> NogateRflowfwR {
        NogateRflowfwR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd7(&self) -> NogateRsvd7R {
        NogateRsvd7R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_cfg(&self) -> NogateCfgR {
        NogateCfgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd8(&self) -> NogateRsvd8R {
        NogateRsvd8R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_tpcf(&self) -> NogateTpcfR {
        NogateTpcfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd9(&self) -> NogateRsvd9R {
        NogateRsvd9R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_ringocc(&self) -> NogateRingoccR {
        NogateRingoccR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_ehandler(&self) -> NogateEhandlerR {
        NogateEhandlerR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd10(&self) -> NogateRsvd10R {
        NogateRsvd10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_p2p(&self) -> NogateP2pR {
        NogateP2pR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd11(&self) -> NogateRsvd11R {
        NogateRsvd11R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_proxy(&self) -> NogateProxyR {
        NogateProxyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_stats(&self) -> NogateStatsR {
        NogateStatsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd12(&self) -> NogateRsvd12R {
        NogateRsvd12R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd5(&mut self) -> NogateRsvd5W<PktdmaGcfgPm1Spec> {
        NogateRsvd5W::new(self, 0)
    }
    #[doc = "Bit 12 - 12:12\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_tcu(&mut self) -> NogateTcuW<PktdmaGcfgPm1Spec> {
        NogateTcuW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rcu(&mut self) -> NogateRcuW<PktdmaGcfgPm1Spec> {
        NogateRcuW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd6(&mut self) -> NogateRsvd6W<PktdmaGcfgPm1Spec> {
        NogateRsvd6W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rflowfw(&mut self) -> NogateRflowfwW<PktdmaGcfgPm1Spec> {
        NogateRflowfwW::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd7(&mut self) -> NogateRsvd7W<PktdmaGcfgPm1Spec> {
        NogateRsvd7W::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_cfg(&mut self) -> NogateCfgW<PktdmaGcfgPm1Spec> {
        NogateCfgW::new(self, 18)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd8(&mut self) -> NogateRsvd8W<PktdmaGcfgPm1Spec> {
        NogateRsvd8W::new(self, 19)
    }
    #[doc = "Bit 22 - 22:22\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_tpcf(&mut self) -> NogateTpcfW<PktdmaGcfgPm1Spec> {
        NogateTpcfW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd9(&mut self) -> NogateRsvd9W<PktdmaGcfgPm1Spec> {
        NogateRsvd9W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_ringocc(&mut self) -> NogateRingoccW<PktdmaGcfgPm1Spec> {
        NogateRingoccW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_ehandler(&mut self) -> NogateEhandlerW<PktdmaGcfgPm1Spec> {
        NogateEhandlerW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd10(&mut self) -> NogateRsvd10W<PktdmaGcfgPm1Spec> {
        NogateRsvd10W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_p2p(&mut self) -> NogateP2pW<PktdmaGcfgPm1Spec> {
        NogateP2pW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd11(&mut self) -> NogateRsvd11W<PktdmaGcfgPm1Spec> {
        NogateRsvd11W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_proxy(&mut self) -> NogateProxyW<PktdmaGcfgPm1Spec> {
        NogateProxyW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_stats(&mut self) -> NogateStatsW<PktdmaGcfgPm1Spec> {
        NogateStatsW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd12(&mut self) -> NogateRsvd12W<PktdmaGcfgPm1Spec> {
        NogateRsvd12W::new(self, 31)
    }
}
#[doc = "This register enables or inhibits automatic clock gating to individual sub-blocks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_pm1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_pm1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaGcfgPm1Spec;
impl crate::RegisterSpec for PktdmaGcfgPm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_gcfg_pm1::R`](R) reader structure"]
impl crate::Readable for PktdmaGcfgPm1Spec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_gcfg_pm1::W`](W) writer structure"]
impl crate::Writable for PktdmaGcfgPm1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_GCFG_PM1 to value 0"]
impl crate::Resettable for PktdmaGcfgPm1Spec {
    const RESET_VALUE: u32 = 0;
}
