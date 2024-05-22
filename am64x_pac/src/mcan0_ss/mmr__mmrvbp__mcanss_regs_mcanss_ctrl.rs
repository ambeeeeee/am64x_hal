#[doc = "Register `MMR__MMRVBP__MCANSS_REGS_MCANSS_CTRL` reader"]
pub type R = crate::R<Mmr_Mmrvbp_McanssRegsMcanssCtrlSpec>;
#[doc = "Register `MMR__MMRVBP__MCANSS_REGS_MCANSS_CTRL` writer"]
pub type W = crate::W<Mmr_Mmrvbp_McanssRegsMcanssCtrlSpec>;
#[doc = "Field `DBGSUSP_FREE` reader - 3:3\\]
0-Honor Debug Suspend, 1-Disregard debug suspend"]
pub type DbgsuspFreeR = crate::BitReader;
#[doc = "Field `DBGSUSP_FREE` writer - 3:3\\]
0-Honor Debug Suspend, 1-Disregard debug suspend"]
pub type DbgsuspFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPREQEN` reader - 4:4\\]
Wakeup Request Enable"]
pub type WakeupreqenR = crate::BitReader;
#[doc = "Field `WAKEUPREQEN` writer - 4:4\\]
Wakeup Request Enable"]
pub type WakeupreqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOWAKEUP` reader - 5:5\\]
Automatic Wakeup Enable"]
pub type AutowakeupR = crate::BitReader;
#[doc = "Field `AUTOWAKEUP` writer - 5:5\\]
Automatic Wakeup Enable"]
pub type AutowakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_TS_CNTR_EN` reader - 6:6\\]
External TimeStamp Counter Enable"]
pub type ExtTsCntrEnR = crate::BitReader;
#[doc = "Field `EXT_TS_CNTR_EN` writer - 6:6\\]
External TimeStamp Counter Enable"]
pub type ExtTsCntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 3:3\\]
0-Honor Debug Suspend, 1-Disregard debug suspend"]
    #[inline(always)]
    pub fn dbgsusp_free(&self) -> DbgsuspFreeR {
        DbgsuspFreeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Wakeup Request Enable"]
    #[inline(always)]
    pub fn wakeupreqen(&self) -> WakeupreqenR {
        WakeupreqenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Automatic Wakeup Enable"]
    #[inline(always)]
    pub fn autowakeup(&self) -> AutowakeupR {
        AutowakeupR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
External TimeStamp Counter Enable"]
    #[inline(always)]
    pub fn ext_ts_cntr_en(&self) -> ExtTsCntrEnR {
        ExtTsCntrEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 3:3\\]
0-Honor Debug Suspend, 1-Disregard debug suspend"]
    #[inline(always)]
    #[must_use]
    pub fn dbgsusp_free(&mut self) -> DbgsuspFreeW<Mmr_Mmrvbp_McanssRegsMcanssCtrlSpec> {
        DbgsuspFreeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Wakeup Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupreqen(&mut self) -> WakeupreqenW<Mmr_Mmrvbp_McanssRegsMcanssCtrlSpec> {
        WakeupreqenW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Automatic Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autowakeup(&mut self) -> AutowakeupW<Mmr_Mmrvbp_McanssRegsMcanssCtrlSpec> {
        AutowakeupW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
External TimeStamp Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ext_ts_cntr_en(&mut self) -> ExtTsCntrEnW<Mmr_Mmrvbp_McanssRegsMcanssCtrlSpec> {
        ExtTsCntrEnW::new(self, 6)
    }
}
#[doc = "The Control Register contains general control bits for the MCANSS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__mmrvbp__mcanss_regs_mcanss_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__mmrvbp__mcanss_regs_mcanss_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Mmrvbp_McanssRegsMcanssCtrlSpec;
impl crate::RegisterSpec for Mmr_Mmrvbp_McanssRegsMcanssCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__mmrvbp__mcanss_regs_mcanss_ctrl::R`](R) reader structure"]
impl crate::Readable for Mmr_Mmrvbp_McanssRegsMcanssCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__mmrvbp__mcanss_regs_mcanss_ctrl::W`](W) writer structure"]
impl crate::Writable for Mmr_Mmrvbp_McanssRegsMcanssCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__MMRVBP__MCANSS_REGS_MCANSS_CTRL to value 0x08"]
impl crate::Resettable for Mmr_Mmrvbp_McanssRegsMcanssCtrlSpec {
    const RESET_VALUE: u32 = 0x08;
}
