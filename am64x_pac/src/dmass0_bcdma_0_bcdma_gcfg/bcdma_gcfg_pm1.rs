#[doc = "Register `BCDMA_GCFG_PM1` reader"]
pub type R = crate::R<BcdmaGcfgPm1Spec>;
#[doc = "Register `BCDMA_GCFG_PM1` writer"]
pub type W = crate::W<BcdmaGcfgPm1Spec>;
#[doc = "Field `NOGATE_TRU0` reader - 0:0\\]
When set inhibits automatic gating of clock."]
pub type NogateTru0R = crate::BitReader;
#[doc = "Field `NOGATE_TRU0` writer - 0:0\\]
When set inhibits automatic gating of clock."]
pub type NogateTru0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_TRU1` reader - 1:1\\]
When set inhibits automatic gating of clock."]
pub type NogateTru1R = crate::BitReader;
#[doc = "Field `NOGATE_TRU1` writer - 1:1\\]
When set inhibits automatic gating of clock."]
pub type NogateTru1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_TRU2` reader - 2:2\\]
When set inhibits automatic gating of clock."]
pub type NogateTru2R = crate::BitReader;
#[doc = "Field `NOGATE_TRU2` writer - 2:2\\]
When set inhibits automatic gating of clock."]
pub type NogateTru2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_TRU3` reader - 3:3\\]
When set inhibits automatic gating of clock."]
pub type NogateTru3R = crate::BitReader;
#[doc = "Field `NOGATE_TRU3` writer - 3:3\\]
When set inhibits automatic gating of clock."]
pub type NogateTru3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RWU0` reader - 4:4\\]
When set inhibits automatic gating of clock."]
pub type NogateRwu0R = crate::BitReader;
#[doc = "Field `NOGATE_RWU0` writer - 4:4\\]
When set inhibits automatic gating of clock."]
pub type NogateRwu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RWU1` reader - 5:5\\]
When set inhibits automatic gating of clock."]
pub type NogateRwu1R = crate::BitReader;
#[doc = "Field `NOGATE_RWU1` writer - 5:5\\]
When set inhibits automatic gating of clock."]
pub type NogateRwu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RWU2` reader - 6:6\\]
When set inhibits automatic gating of clock."]
pub type NogateRwu2R = crate::BitReader;
#[doc = "Field `NOGATE_RWU2` writer - 6:6\\]
When set inhibits automatic gating of clock."]
pub type NogateRwu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RWU3` reader - 7:7\\]
When set inhibits automatic gating of clock."]
pub type NogateRwu3R = crate::BitReader;
#[doc = "Field `NOGATE_RWU3` writer - 7:7\\]
When set inhibits automatic gating of clock."]
pub type NogateRwu3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_EVTCU` reader - 8:8\\]
When set inhibits automatic gating of clock."]
pub type NogateEvtcuR = crate::BitReader;
#[doc = "Field `NOGATE_EVTCU` writer - 8:8\\]
When set inhibits automatic gating of clock."]
pub type NogateEvtcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD5` reader - 9:9\\]
Reserved PM signals."]
pub type NogateRsvd5R = crate::BitReader;
#[doc = "Field `NOGATE_RSVD5` writer - 9:9\\]
Reserved PM signals."]
pub type NogateRsvd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_TRCU` reader - 10:10\\]
When set inhibits automatic gating of clock."]
pub type NogateTrcuR = crate::BitReader;
#[doc = "Field `NOGATE_TRCU` writer - 10:10\\]
When set inhibits automatic gating of clock."]
pub type NogateTrcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD6` reader - 17:11\\]
Reserved PM signals."]
pub type NogateRsvd6R = crate::FieldReader;
#[doc = "Field `NOGATE_RSVD6` writer - 17:11\\]
Reserved PM signals."]
pub type NogateRsvd6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NOGATE_CFG` reader - 18:18\\]
When set inhibits automatic gating of clock."]
pub type NogateCfgR = crate::BitReader;
#[doc = "Field `NOGATE_CFG` writer - 18:18\\]
When set inhibits automatic gating of clock."]
pub type NogateCfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RSVD7` reader - 20:19\\]
Reserved PM signals."]
pub type NogateRsvd7R = crate::FieldReader;
#[doc = "Field `NOGATE_RSVD7` writer - 20:19\\]
Reserved PM signals."]
pub type NogateRsvd7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOGATE_PCF` reader - 21:21\\]
When set inhibits automatic gating of clock."]
pub type NogatePcfR = crate::BitReader;
#[doc = "Field `NOGATE_PCF` writer - 21:21\\]
When set inhibits automatic gating of clock."]
pub type NogatePcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_TPCF` reader - 22:22\\]
When set inhibits automatic gating of clock."]
pub type NogateTpcfR = crate::BitReader;
#[doc = "Field `NOGATE_TPCF` writer - 22:22\\]
When set inhibits automatic gating of clock."]
pub type NogateTpcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_RPCF` reader - 23:23\\]
When set inhibits automatic gating of clock."]
pub type NogateRpcfR = crate::BitReader;
#[doc = "Field `NOGATE_RPCF` writer - 23:23\\]
When set inhibits automatic gating of clock."]
pub type NogateRpcfW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `NOGATE_RSVD8` reader - 26:26\\]
Reserved PM signals."]
pub type NogateRsvd8R = crate::BitReader;
#[doc = "Field `NOGATE_RSVD8` writer - 26:26\\]
Reserved PM signals."]
pub type NogateRsvd8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_P2P` reader - 27:27\\]
When set inhibits automatic gating of clock."]
pub type NogateP2pR = crate::BitReader;
#[doc = "Field `NOGATE_P2P` writer - 27:27\\]
When set inhibits automatic gating of clock."]
pub type NogateP2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOGATE_PSILIF` reader - 28:28\\]
When set inhibits automatic gating of clock."]
pub type NogatePsilifR = crate::BitReader;
#[doc = "Field `NOGATE_PSILIF` writer - 28:28\\]
When set inhibits automatic gating of clock."]
pub type NogatePsilifW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `NOGATE_EDC` reader - 31:31\\]
When set inhibits automatic gating of clock."]
pub type NogateEdcR = crate::BitReader;
#[doc = "Field `NOGATE_EDC` writer - 31:31\\]
When set inhibits automatic gating of clock."]
pub type NogateEdcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_tru0(&self) -> NogateTru0R {
        NogateTru0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_tru1(&self) -> NogateTru1R {
        NogateTru1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_tru2(&self) -> NogateTru2R {
        NogateTru2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_tru3(&self) -> NogateTru3R {
        NogateTru3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_rwu0(&self) -> NogateRwu0R {
        NogateRwu0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_rwu1(&self) -> NogateRwu1R {
        NogateRwu1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_rwu2(&self) -> NogateRwu2R {
        NogateRwu2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_rwu3(&self) -> NogateRwu3R {
        NogateRwu3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_evtcu(&self) -> NogateEvtcuR {
        NogateEvtcuR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd5(&self) -> NogateRsvd5R {
        NogateRsvd5R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_trcu(&self) -> NogateTrcuR {
        NogateTrcuR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:17 - 17:11\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd6(&self) -> NogateRsvd6R {
        NogateRsvd6R::new(((self.bits >> 11) & 0x7f) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_cfg(&self) -> NogateCfgR {
        NogateCfgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Reserved PM signals."]
    #[inline(always)]
    pub fn nogate_rsvd7(&self) -> NogateRsvd7R {
        NogateRsvd7R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_pcf(&self) -> NogatePcfR {
        NogatePcfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_tpcf(&self) -> NogateTpcfR {
        NogateTpcfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_rpcf(&self) -> NogateRpcfR {
        NogateRpcfR::new(((self.bits >> 23) & 1) != 0)
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
    pub fn nogate_rsvd8(&self) -> NogateRsvd8R {
        NogateRsvd8R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_p2p(&self) -> NogateP2pR {
        NogateP2pR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_psilif(&self) -> NogatePsilifR {
        NogatePsilifR::new(((self.bits >> 28) & 1) != 0)
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
When set inhibits automatic gating of clock."]
    #[inline(always)]
    pub fn nogate_edc(&self) -> NogateEdcR {
        NogateEdcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_tru0(&mut self) -> NogateTru0W<BcdmaGcfgPm1Spec> {
        NogateTru0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_tru1(&mut self) -> NogateTru1W<BcdmaGcfgPm1Spec> {
        NogateTru1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_tru2(&mut self) -> NogateTru2W<BcdmaGcfgPm1Spec> {
        NogateTru2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_tru3(&mut self) -> NogateTru3W<BcdmaGcfgPm1Spec> {
        NogateTru3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rwu0(&mut self) -> NogateRwu0W<BcdmaGcfgPm1Spec> {
        NogateRwu0W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rwu1(&mut self) -> NogateRwu1W<BcdmaGcfgPm1Spec> {
        NogateRwu1W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rwu2(&mut self) -> NogateRwu2W<BcdmaGcfgPm1Spec> {
        NogateRwu2W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rwu3(&mut self) -> NogateRwu3W<BcdmaGcfgPm1Spec> {
        NogateRwu3W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_evtcu(&mut self) -> NogateEvtcuW<BcdmaGcfgPm1Spec> {
        NogateEvtcuW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd5(&mut self) -> NogateRsvd5W<BcdmaGcfgPm1Spec> {
        NogateRsvd5W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_trcu(&mut self) -> NogateTrcuW<BcdmaGcfgPm1Spec> {
        NogateTrcuW::new(self, 10)
    }
    #[doc = "Bits 11:17 - 17:11\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd6(&mut self) -> NogateRsvd6W<BcdmaGcfgPm1Spec> {
        NogateRsvd6W::new(self, 11)
    }
    #[doc = "Bit 18 - 18:18\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_cfg(&mut self) -> NogateCfgW<BcdmaGcfgPm1Spec> {
        NogateCfgW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd7(&mut self) -> NogateRsvd7W<BcdmaGcfgPm1Spec> {
        NogateRsvd7W::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_pcf(&mut self) -> NogatePcfW<BcdmaGcfgPm1Spec> {
        NogatePcfW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_tpcf(&mut self) -> NogateTpcfW<BcdmaGcfgPm1Spec> {
        NogateTpcfW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rpcf(&mut self) -> NogateRpcfW<BcdmaGcfgPm1Spec> {
        NogateRpcfW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_ringocc(&mut self) -> NogateRingoccW<BcdmaGcfgPm1Spec> {
        NogateRingoccW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_ehandler(&mut self) -> NogateEhandlerW<BcdmaGcfgPm1Spec> {
        NogateEhandlerW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Reserved PM signals."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_rsvd8(&mut self) -> NogateRsvd8W<BcdmaGcfgPm1Spec> {
        NogateRsvd8W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_p2p(&mut self) -> NogateP2pW<BcdmaGcfgPm1Spec> {
        NogateP2pW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_psilif(&mut self) -> NogatePsilifW<BcdmaGcfgPm1Spec> {
        NogatePsilifW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_proxy(&mut self) -> NogateProxyW<BcdmaGcfgPm1Spec> {
        NogateProxyW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_stats(&mut self) -> NogateStatsW<BcdmaGcfgPm1Spec> {
        NogateStatsW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
When set inhibits automatic gating of clock."]
    #[inline(always)]
    #[must_use]
    pub fn nogate_edc(&mut self) -> NogateEdcW<BcdmaGcfgPm1Spec> {
        NogateEdcW::new(self, 31)
    }
}
#[doc = "This register enables or inhibits automatic clock gating to individual sub-blocks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_pm1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_pm1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaGcfgPm1Spec;
impl crate::RegisterSpec for BcdmaGcfgPm1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_gcfg_pm1::R`](R) reader structure"]
impl crate::Readable for BcdmaGcfgPm1Spec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_gcfg_pm1::W`](W) writer structure"]
impl crate::Writable for BcdmaGcfgPm1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_GCFG_PM1 to value 0"]
impl crate::Resettable for BcdmaGcfgPm1Spec {
    const RESET_VALUE: u32 = 0;
}
