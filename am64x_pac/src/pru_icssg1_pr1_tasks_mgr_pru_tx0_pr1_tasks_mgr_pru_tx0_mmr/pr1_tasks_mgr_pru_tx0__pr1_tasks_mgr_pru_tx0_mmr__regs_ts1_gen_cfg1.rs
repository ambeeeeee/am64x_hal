#[doc = "Register `PR1_TASKS_MGR_PRU_TX0__PR1_TASKS_MGR_PRU_TX0_MMR__REGS_ts1_gen_cfg1` reader"]
pub type R = crate::R<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs1GenCfg1Spec>;
#[doc = "Register `PR1_TASKS_MGR_PRU_TX0__PR1_TASKS_MGR_PRU_TX0_MMR__REGS_ts1_gen_cfg1` writer"]
pub type W = crate::W<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs1GenCfg1Spec>;
#[doc = "Field `TS1_GEN_S0_MX` reader - 7:0\\]
TS1 Generic Sub0 MX Select"]
pub type Ts1GenS0MxR = crate::FieldReader;
#[doc = "Field `TS1_GEN_S0_MX` writer - 7:0\\]
TS1 Generic Sub0 MX Select"]
pub type Ts1GenS0MxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TS1_GEN_S1_MX` reader - 15:8\\]
TS1 Generic Sub1 MX Select"]
pub type Ts1GenS1MxR = crate::FieldReader;
#[doc = "Field `TS1_GEN_S1_MX` writer - 15:8\\]
TS1 Generic Sub1 MX Select"]
pub type Ts1GenS1MxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TS1_GEN_S2_MX` reader - 23:16\\]
TS1 Generic Sub2 MX Select"]
pub type Ts1GenS2MxR = crate::FieldReader;
#[doc = "Field `TS1_GEN_S2_MX` writer - 23:16\\]
TS1 Generic Sub2 MX Select"]
pub type Ts1GenS2MxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TS1_GEN_S3_MX` reader - 31:24\\]
TS1 Generic Sub3 MX Select"]
pub type Ts1GenS3MxR = crate::FieldReader;
#[doc = "Field `TS1_GEN_S3_MX` writer - 31:24\\]
TS1 Generic Sub3 MX Select"]
pub type Ts1GenS3MxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TS1 Generic Sub0 MX Select"]
    #[inline(always)]
    pub fn ts1_gen_s0_mx(&self) -> Ts1GenS0MxR {
        Ts1GenS0MxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TS1 Generic Sub1 MX Select"]
    #[inline(always)]
    pub fn ts1_gen_s1_mx(&self) -> Ts1GenS1MxR {
        Ts1GenS1MxR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TS1 Generic Sub2 MX Select"]
    #[inline(always)]
    pub fn ts1_gen_s2_mx(&self) -> Ts1GenS2MxR {
        Ts1GenS2MxR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TS1 Generic Sub3 MX Select"]
    #[inline(always)]
    pub fn ts1_gen_s3_mx(&self) -> Ts1GenS3MxR {
        Ts1GenS3MxR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TS1 Generic Sub0 MX Select"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_gen_s0_mx(
        &mut self,
    ) -> Ts1GenS0MxW<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs1GenCfg1Spec> {
        Ts1GenS0MxW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
TS1 Generic Sub1 MX Select"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_gen_s1_mx(
        &mut self,
    ) -> Ts1GenS1MxW<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs1GenCfg1Spec> {
        Ts1GenS1MxW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
TS1 Generic Sub2 MX Select"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_gen_s2_mx(
        &mut self,
    ) -> Ts1GenS2MxW<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs1GenCfg1Spec> {
        Ts1GenS2MxW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
TS1 Generic Sub3 MX Select"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_gen_s3_mx(
        &mut self,
    ) -> Ts1GenS3MxW<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs1GenCfg1Spec> {
        Ts1GenS3MxW::new(self, 24)
    }
}
#[doc = "Generic TS1 Configuration1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_pru_tx0__pr1_tasks_mgr_pru_tx0_mmr__regs_ts1_gen_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_pru_tx0__pr1_tasks_mgr_pru_tx0_mmr__regs_ts1_gen_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs1GenCfg1Spec;
impl crate::RegisterSpec for Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs1GenCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_tasks_mgr_pru_tx0__pr1_tasks_mgr_pru_tx0_mmr__regs_ts1_gen_cfg1::R`](R) reader structure"]
impl crate::Readable for Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs1GenCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_tasks_mgr_pru_tx0__pr1_tasks_mgr_pru_tx0_mmr__regs_ts1_gen_cfg1::W`](W) writer structure"]
impl crate::Writable for Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs1GenCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_TASKS_MGR_PRU_TX0__PR1_TASKS_MGR_PRU_TX0_MMR__REGS_ts1_gen_cfg1 to value 0"]
impl crate::Resettable for Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs1GenCfg1Spec {
    const RESET_VALUE: u32 = 0;
}
