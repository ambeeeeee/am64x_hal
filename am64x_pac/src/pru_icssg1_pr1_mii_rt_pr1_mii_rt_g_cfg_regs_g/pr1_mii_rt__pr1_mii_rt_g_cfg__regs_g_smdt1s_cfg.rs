#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_smdt1s_cfg` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1sCfgSpec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_smdt1s_cfg` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1sCfgSpec>;
#[doc = "Field `SMDT1S_0` reader - 7:0\\]
SMDT1S0 pattern"]
pub type Smdt1s0R = crate::FieldReader;
#[doc = "Field `SMDT1S_0` writer - 7:0\\]
SMDT1S0 pattern"]
pub type Smdt1s0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SMDT1S_1` reader - 15:8\\]
SMDT1S1 pattern"]
pub type Smdt1s1R = crate::FieldReader;
#[doc = "Field `SMDT1S_1` writer - 15:8\\]
SMDT1S1 pattern"]
pub type Smdt1s1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SMDT1S_2` reader - 23:16\\]
SMDT1S2 pattern"]
pub type Smdt1s2R = crate::FieldReader;
#[doc = "Field `SMDT1S_2` writer - 23:16\\]
SMDT1S2 pattern"]
pub type Smdt1s2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SMDT1S_3` reader - 31:24\\]
SMDT1S3 pattern"]
pub type Smdt1s3R = crate::FieldReader;
#[doc = "Field `SMDT1S_3` writer - 31:24\\]
SMDT1S3 pattern"]
pub type Smdt1s3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
SMDT1S0 pattern"]
    #[inline(always)]
    pub fn smdt1s_0(&self) -> Smdt1s0R {
        Smdt1s0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
SMDT1S1 pattern"]
    #[inline(always)]
    pub fn smdt1s_1(&self) -> Smdt1s1R {
        Smdt1s1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SMDT1S2 pattern"]
    #[inline(always)]
    pub fn smdt1s_2(&self) -> Smdt1s2R {
        Smdt1s2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SMDT1S3 pattern"]
    #[inline(always)]
    pub fn smdt1s_3(&self) -> Smdt1s3R {
        Smdt1s3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
SMDT1S0 pattern"]
    #[inline(always)]
    #[must_use]
    pub fn smdt1s_0(&mut self) -> Smdt1s0W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1sCfgSpec> {
        Smdt1s0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
SMDT1S1 pattern"]
    #[inline(always)]
    #[must_use]
    pub fn smdt1s_1(&mut self) -> Smdt1s1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1sCfgSpec> {
        Smdt1s1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SMDT1S2 pattern"]
    #[inline(always)]
    #[must_use]
    pub fn smdt1s_2(&mut self) -> Smdt1s2W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1sCfgSpec> {
        Smdt1s2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SMDT1S3 pattern"]
    #[inline(always)]
    #[must_use]
    pub fn smdt1s_3(&mut self) -> Smdt1s3W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1sCfgSpec> {
        Smdt1s3W::new(self, 24)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_smdt1s_cfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_smdt1s_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_smdt1s_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1sCfgSpec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1sCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_smdt1s_cfg::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1sCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_smdt1s_cfg::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1sCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_smdt1s_cfg to value 0x0127_7630"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1sCfgSpec {
    const RESET_VALUE: u32 = 0x0127_7630;
}
