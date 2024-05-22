#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft1_cfg_pru0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft1_cfg_pru0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec>;
#[doc = "Field `FT1_0CFG` reader - 1:0\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_0cfgR = crate::FieldReader;
#[doc = "Field `FT1_0CFG` writer - 1:0\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_0cfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FT1_1CFG` reader - 3:2\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_1cfgR = crate::FieldReader;
#[doc = "Field `FT1_1CFG` writer - 3:2\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_1cfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FT1_2CFG` reader - 5:4\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_2cfgR = crate::FieldReader;
#[doc = "Field `FT1_2CFG` writer - 5:4\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_2cfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FT1_3CFG` reader - 7:6\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_3cfgR = crate::FieldReader;
#[doc = "Field `FT1_3CFG` writer - 7:6\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_3cfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FT1_4CFG` reader - 9:8\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_4cfgR = crate::FieldReader;
#[doc = "Field `FT1_4CFG` writer - 9:8\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_4cfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FT1_5CFG` reader - 11:10\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_5cfgR = crate::FieldReader;
#[doc = "Field `FT1_5CFG` writer - 11:10\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_5cfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FT1_6CFG` reader - 13:12\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_6cfgR = crate::FieldReader;
#[doc = "Field `FT1_6CFG` writer - 13:12\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_6cfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FT1_7CFG` reader - 15:14\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_7cfgR = crate::FieldReader;
#[doc = "Field `FT1_7CFG` writer - 15:14\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft1_7cfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    pub fn ft1_0cfg(&self) -> Ft1_0cfgR {
        Ft1_0cfgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    pub fn ft1_1cfg(&self) -> Ft1_1cfgR {
        Ft1_1cfgR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    pub fn ft1_2cfg(&self) -> Ft1_2cfgR {
        Ft1_2cfgR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    pub fn ft1_3cfg(&self) -> Ft1_3cfgR {
        Ft1_3cfgR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    pub fn ft1_4cfg(&self) -> Ft1_4cfgR {
        Ft1_4cfgR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    pub fn ft1_5cfg(&self) -> Ft1_5cfgR {
        Ft1_5cfgR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    pub fn ft1_6cfg(&self) -> Ft1_6cfgR {
        Ft1_6cfgR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    pub fn ft1_7cfg(&self) -> Ft1_7cfgR {
        Ft1_7cfgR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    #[must_use]
    pub fn ft1_0cfg(&mut self) -> Ft1_0cfgW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec> {
        Ft1_0cfgW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    #[must_use]
    pub fn ft1_1cfg(&mut self) -> Ft1_1cfgW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec> {
        Ft1_1cfgW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    #[must_use]
    pub fn ft1_2cfg(&mut self) -> Ft1_2cfgW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec> {
        Ft1_2cfgW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    #[must_use]
    pub fn ft1_3cfg(&mut self) -> Ft1_3cfgW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec> {
        Ft1_3cfgW::new(self, 6)
    }
    #[doc = "Bits 8:9 - 9:8\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    #[must_use]
    pub fn ft1_4cfg(&mut self) -> Ft1_4cfgW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec> {
        Ft1_4cfgW::new(self, 8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    #[must_use]
    pub fn ft1_5cfg(&mut self) -> Ft1_5cfgW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec> {
        Ft1_5cfgW::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    #[must_use]
    pub fn ft1_6cfg(&mut self) -> Ft1_6cfgW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec> {
        Ft1_6cfgW::new(self, 12)
    }
    #[doc = "Bits 14:15 - 15:14\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    #[must_use]
    pub fn ft1_7cfg(&mut self) -> Ft1_7cfgW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec> {
        Ft1_7cfgW::new(self, 14)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft1_cfg_pru0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft1_cfg_pru0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft1_cfg_pru0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft1_cfg_pru0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft1_cfg_pru0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft1_cfg_pru0 to value 0x5555"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1CfgPru0Spec {
    const RESET_VALUE: u32 = 0x5555;
}
