#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_frag_cnt_cfg` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFragCntCfgSpec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_frag_cnt_cfg` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFragCntCfgSpec>;
#[doc = "Field `FRAG_CNT_0` reader - 7:0\\]
FRAG Cnt0 pattern"]
pub type FragCnt0R = crate::FieldReader;
#[doc = "Field `FRAG_CNT_0` writer - 7:0\\]
FRAG Cnt0 pattern"]
pub type FragCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FRAG_CNT_1` reader - 15:8\\]
FRAG Cnt1 pattern"]
pub type FragCnt1R = crate::FieldReader;
#[doc = "Field `FRAG_CNT_1` writer - 15:8\\]
FRAG Cnt1 pattern"]
pub type FragCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FRAG_CNT_2` reader - 23:16\\]
FRAG Cnt2 pattern"]
pub type FragCnt2R = crate::FieldReader;
#[doc = "Field `FRAG_CNT_2` writer - 23:16\\]
FRAG Cnt2 pattern"]
pub type FragCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FRAG_CNT_3` reader - 31:24\\]
FRAG Cnt3 pattern"]
pub type FragCnt3R = crate::FieldReader;
#[doc = "Field `FRAG_CNT_3` writer - 31:24\\]
FRAG Cnt3 pattern"]
pub type FragCnt3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
FRAG Cnt0 pattern"]
    #[inline(always)]
    pub fn frag_cnt_0(&self) -> FragCnt0R {
        FragCnt0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
FRAG Cnt1 pattern"]
    #[inline(always)]
    pub fn frag_cnt_1(&self) -> FragCnt1R {
        FragCnt1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
FRAG Cnt2 pattern"]
    #[inline(always)]
    pub fn frag_cnt_2(&self) -> FragCnt2R {
        FragCnt2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
FRAG Cnt3 pattern"]
    #[inline(always)]
    pub fn frag_cnt_3(&self) -> FragCnt3R {
        FragCnt3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
FRAG Cnt0 pattern"]
    #[inline(always)]
    #[must_use]
    pub fn frag_cnt_0(&mut self) -> FragCnt0W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFragCntCfgSpec> {
        FragCnt0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
FRAG Cnt1 pattern"]
    #[inline(always)]
    #[must_use]
    pub fn frag_cnt_1(&mut self) -> FragCnt1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFragCntCfgSpec> {
        FragCnt1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
FRAG Cnt2 pattern"]
    #[inline(always)]
    #[must_use]
    pub fn frag_cnt_2(&mut self) -> FragCnt2W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFragCntCfgSpec> {
        FragCnt2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
FRAG Cnt3 pattern"]
    #[inline(always)]
    #[must_use]
    pub fn frag_cnt_3(&mut self) -> FragCnt3W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFragCntCfgSpec> {
        FragCnt3W::new(self, 24)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_frag_cnt_cfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_frag_cnt_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_frag_cnt_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFragCntCfgSpec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFragCntCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_frag_cnt_cfg::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFragCntCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_frag_cnt_cfg::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFragCntCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_frag_cnt_cfg to value 0x0127_7630"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFragCntCfgSpec {
    const RESET_VALUE: u32 = 0x0127_7630;
}
