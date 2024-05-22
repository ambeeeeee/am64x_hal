#[doc = "Register `PR1_TASKS_MGR_PRU1__PR1_TASKS_MGR_PRU1_MMR__REGS_ts2_gen_cfg2` reader"]
pub type R = crate::R<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs2GenCfg2Spec>;
#[doc = "Register `PR1_TASKS_MGR_PRU1__PR1_TASKS_MGR_PRU1_MMR__REGS_ts2_gen_cfg2` writer"]
pub type W = crate::W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs2GenCfg2Spec>;
#[doc = "Field `TS2_GEN_S4_MX` reader - 7:0\\]
TS2 Generic Sub4 MX Select"]
pub type Ts2GenS4MxR = crate::FieldReader;
#[doc = "Field `TS2_GEN_S4_MX` writer - 7:0\\]
TS2 Generic Sub4 MX Select"]
pub type Ts2GenS4MxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TS2 Generic Sub4 MX Select"]
    #[inline(always)]
    pub fn ts2_gen_s4_mx(&self) -> Ts2GenS4MxR {
        Ts2GenS4MxR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TS2 Generic Sub4 MX Select"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_gen_s4_mx(
        &mut self,
    ) -> Ts2GenS4MxW<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs2GenCfg2Spec> {
        Ts2GenS4MxW::new(self, 0)
    }
}
#[doc = "Generic TS2 Configuration2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_pru1__pr1_tasks_mgr_pru1_mmr__regs_ts2_gen_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_pru1__pr1_tasks_mgr_pru1_mmr__regs_ts2_gen_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs2GenCfg2Spec;
impl crate::RegisterSpec for Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs2GenCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_tasks_mgr_pru1__pr1_tasks_mgr_pru1_mmr__regs_ts2_gen_cfg2::R`](R) reader structure"]
impl crate::Readable for Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs2GenCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_tasks_mgr_pru1__pr1_tasks_mgr_pru1_mmr__regs_ts2_gen_cfg2::W`](W) writer structure"]
impl crate::Writable for Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs2GenCfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_TASKS_MGR_PRU1__PR1_TASKS_MGR_PRU1_MMR__REGS_ts2_gen_cfg2 to value 0"]
impl crate::Resettable for Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs2GenCfg2Spec {
    const RESET_VALUE: u32 = 0;
}
