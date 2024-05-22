#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_smdt1c_cfg` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1cCfgSpec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_smdt1c_cfg` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1cCfgSpec>;
#[doc = "Field `SMDT1C_0` reader - 7:0\\]
SMDT1C0 pattern"]
pub type Smdt1c0R = crate::FieldReader;
#[doc = "Field `SMDT1C_0` writer - 7:0\\]
SMDT1C0 pattern"]
pub type Smdt1c0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SMDT1C_1` reader - 15:8\\]
SMDT1C1 pattern"]
pub type Smdt1c1R = crate::FieldReader;
#[doc = "Field `SMDT1C_1` writer - 15:8\\]
SMDT1C1 pattern"]
pub type Smdt1c1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SMDT1C_2` reader - 23:16\\]
SMDT1C2 pattern"]
pub type Smdt1c2R = crate::FieldReader;
#[doc = "Field `SMDT1C_2` writer - 23:16\\]
SMDT1C2 pattern"]
pub type Smdt1c2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SMDT1C_3` reader - 31:24\\]
SMDT1C3 pattern"]
pub type Smdt1c3R = crate::FieldReader;
#[doc = "Field `SMDT1C_3` writer - 31:24\\]
SMDT1C3 pattern"]
pub type Smdt1c3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
SMDT1C0 pattern"]
    #[inline(always)]
    pub fn smdt1c_0(&self) -> Smdt1c0R {
        Smdt1c0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
SMDT1C1 pattern"]
    #[inline(always)]
    pub fn smdt1c_1(&self) -> Smdt1c1R {
        Smdt1c1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SMDT1C2 pattern"]
    #[inline(always)]
    pub fn smdt1c_2(&self) -> Smdt1c2R {
        Smdt1c2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SMDT1C3 pattern"]
    #[inline(always)]
    pub fn smdt1c_3(&self) -> Smdt1c3R {
        Smdt1c3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
SMDT1C0 pattern"]
    #[inline(always)]
    #[must_use]
    pub fn smdt1c_0(&mut self) -> Smdt1c0W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1cCfgSpec> {
        Smdt1c0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
SMDT1C1 pattern"]
    #[inline(always)]
    #[must_use]
    pub fn smdt1c_1(&mut self) -> Smdt1c1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1cCfgSpec> {
        Smdt1c1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SMDT1C2 pattern"]
    #[inline(always)]
    #[must_use]
    pub fn smdt1c_2(&mut self) -> Smdt1c2W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1cCfgSpec> {
        Smdt1c2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
SMDT1C3 pattern"]
    #[inline(always)]
    #[must_use]
    pub fn smdt1c_3(&mut self) -> Smdt1c3W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1cCfgSpec> {
        Smdt1c3W::new(self, 24)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_smdt1c_cfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_smdt1c_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_smdt1c_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1cCfgSpec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1cCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_smdt1c_cfg::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1cCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_smdt1c_cfg::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1cCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_smdt1c_cfg to value 0x4358_8297"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGSmdt1cCfgSpec {
    const RESET_VALUE: u32 = 0x4358_8297;
}
