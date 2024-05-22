#[doc = "Register `PR1_TASKS_MGR_PRU1__PR1_TASKS_MGR_PRU1_MMR__REGS_global_cfg` reader"]
pub type R = crate::R<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec>;
#[doc = "Register `PR1_TASKS_MGR_PRU1__PR1_TASKS_MGR_PRU1_MMR__REGS_global_cfg` writer"]
pub type W = crate::W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec>;
#[doc = "Field `TASKS_MGR_MODE` reader - 1:0\\]
TaskSwap Mode 0: Disabled 1: RXTX 2: General_HW"]
pub type TasksMgrModeR = crate::FieldReader;
#[doc = "Field `TASKS_MGR_MODE` writer - 1:0\\]
TaskSwap Mode 0: Disabled 1: RXTX 2: General_HW"]
pub type TasksMgrModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TS1_EN_S0` reader - 2:2\\]
TS1 Sub0 0: Disabled 1: Enabled"]
pub type Ts1EnS0R = crate::BitReader;
#[doc = "Field `TS1_EN_S0` writer - 2:2\\]
TS1 Sub0 0: Disabled 1: Enabled"]
pub type Ts1EnS0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_EN_S1` reader - 3:3\\]
TS1 Sub1 0: Disabled 1: Enabled"]
pub type Ts1EnS1R = crate::BitReader;
#[doc = "Field `TS1_EN_S1` writer - 3:3\\]
TS1 Sub1 0: Disabled 1: Enabled"]
pub type Ts1EnS1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_EN_S2` reader - 4:4\\]
TS1 Sub2 0: Disabled 1: Enabled"]
pub type Ts1EnS2R = crate::BitReader;
#[doc = "Field `TS1_EN_S2` writer - 4:4\\]
TS1 Sub2 0: Disabled 1: Enabled"]
pub type Ts1EnS2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_EN_S3` reader - 5:5\\]
TS1 Sub3 0: Disabled 1: Enabled"]
pub type Ts1EnS3R = crate::BitReader;
#[doc = "Field `TS1_EN_S3` writer - 5:5\\]
TS1 Sub3 0: Disabled 1: Enabled"]
pub type Ts1EnS3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_EN_S4` reader - 6:6\\]
TS1 Sub4 0: Disabled 1: Enabled"]
pub type Ts1EnS4R = crate::BitReader;
#[doc = "Field `TS1_EN_S4` writer - 6:6\\]
TS1 Sub4 0: Disabled 1: Enabled"]
pub type Ts1EnS4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS2_EN_S0` reader - 7:7\\]
TS2 Sub0 0: Disabled 1: Enabled"]
pub type Ts2EnS0R = crate::BitReader;
#[doc = "Field `TS2_EN_S0` writer - 7:7\\]
TS2 Sub0 0: Disabled 1: Enabled"]
pub type Ts2EnS0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS2_EN_S1` reader - 8:8\\]
TS2 Sub1 0: Disabled 1: Enabled"]
pub type Ts2EnS1R = crate::BitReader;
#[doc = "Field `TS2_EN_S1` writer - 8:8\\]
TS2 Sub1 0: Disabled 1: Enabled"]
pub type Ts2EnS1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS2_EN_S2` reader - 9:9\\]
TS2 Sub2 0: Disabled 1: Enabled"]
pub type Ts2EnS2R = crate::BitReader;
#[doc = "Field `TS2_EN_S2` writer - 9:9\\]
TS2 Sub2 0: Disabled 1: Enabled"]
pub type Ts2EnS2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS2_EN_S3` reader - 10:10\\]
TS2 Sub3 0: Disabled 1: Enabled"]
pub type Ts2EnS3R = crate::BitReader;
#[doc = "Field `TS2_EN_S3` writer - 10:10\\]
TS2 Sub3 0: Disabled 1: Enabled"]
pub type Ts2EnS3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS2_EN_S4` reader - 11:11\\]
TS2 Sub4 0: Disabled 1: Enabled"]
pub type Ts2EnS4R = crate::BitReader;
#[doc = "Field `TS2_EN_S4` writer - 11:11\\]
TS2 Sub4 0: Disabled 1: Enabled"]
pub type Ts2EnS4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
TaskSwap Mode 0: Disabled 1: RXTX 2: General_HW"]
    #[inline(always)]
    pub fn tasks_mgr_mode(&self) -> TasksMgrModeR {
        TasksMgrModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
TS1 Sub0 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn ts1_en_s0(&self) -> Ts1EnS0R {
        Ts1EnS0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
TS1 Sub1 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn ts1_en_s1(&self) -> Ts1EnS1R {
        Ts1EnS1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
TS1 Sub2 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn ts1_en_s2(&self) -> Ts1EnS2R {
        Ts1EnS2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
TS1 Sub3 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn ts1_en_s3(&self) -> Ts1EnS3R {
        Ts1EnS3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
TS1 Sub4 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn ts1_en_s4(&self) -> Ts1EnS4R {
        Ts1EnS4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
TS2 Sub0 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn ts2_en_s0(&self) -> Ts2EnS0R {
        Ts2EnS0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
TS2 Sub1 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn ts2_en_s1(&self) -> Ts2EnS1R {
        Ts2EnS1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
TS2 Sub2 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn ts2_en_s2(&self) -> Ts2EnS2R {
        Ts2EnS2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
TS2 Sub3 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn ts2_en_s3(&self) -> Ts2EnS3R {
        Ts2EnS3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
TS2 Sub4 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn ts2_en_s4(&self) -> Ts2EnS4R {
        Ts2EnS4R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
TaskSwap Mode 0: Disabled 1: RXTX 2: General_HW"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_mgr_mode(
        &mut self,
    ) -> TasksMgrModeW<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec> {
        TasksMgrModeW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TS1 Sub0 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_en_s0(&mut self) -> Ts1EnS0W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec> {
        Ts1EnS0W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
TS1 Sub1 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_en_s1(&mut self) -> Ts1EnS1W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec> {
        Ts1EnS1W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
TS1 Sub2 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_en_s2(&mut self) -> Ts1EnS2W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec> {
        Ts1EnS2W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
TS1 Sub3 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_en_s3(&mut self) -> Ts1EnS3W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec> {
        Ts1EnS3W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
TS1 Sub4 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_en_s4(&mut self) -> Ts1EnS4W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec> {
        Ts1EnS4W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
TS2 Sub0 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_en_s0(&mut self) -> Ts2EnS0W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec> {
        Ts2EnS0W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
TS2 Sub1 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_en_s1(&mut self) -> Ts2EnS1W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec> {
        Ts2EnS1W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
TS2 Sub2 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_en_s2(&mut self) -> Ts2EnS2W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec> {
        Ts2EnS2W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
TS2 Sub3 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_en_s3(&mut self) -> Ts2EnS3W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec> {
        Ts2EnS3W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
TS2 Sub4 0: Disabled 1: Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_en_s4(&mut self) -> Ts2EnS4W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec> {
        Ts2EnS4W::new(self, 11)
    }
}
#[doc = "Global Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_pru1__pr1_tasks_mgr_pru1_mmr__regs_global_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_pru1__pr1_tasks_mgr_pru1_mmr__regs_global_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec;
impl crate::RegisterSpec for Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_tasks_mgr_pru1__pr1_tasks_mgr_pru1_mmr__regs_global_cfg::R`](R) reader structure"]
impl crate::Readable for Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_tasks_mgr_pru1__pr1_tasks_mgr_pru1_mmr__regs_global_cfg::W`](W) writer structure"]
impl crate::Writable for Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_TASKS_MGR_PRU1__PR1_TASKS_MGR_PRU1_MMR__REGS_global_cfg to value 0"]
impl crate::Resettable for Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsGlobalCfgSpec {
    const RESET_VALUE: u32 = 0;
}
