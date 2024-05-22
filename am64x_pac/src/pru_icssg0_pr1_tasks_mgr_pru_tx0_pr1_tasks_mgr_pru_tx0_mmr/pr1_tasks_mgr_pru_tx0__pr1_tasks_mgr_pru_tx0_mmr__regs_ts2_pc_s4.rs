#[doc = "Register `PR1_TASKS_MGR_PRU_TX0__PR1_TASKS_MGR_PRU_TX0_MMR__REGS_ts2_pc_s4` reader"]
pub type R = crate::R<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs2PcS4Spec>;
#[doc = "Register `PR1_TASKS_MGR_PRU_TX0__PR1_TASKS_MGR_PRU_TX0_MMR__REGS_ts2_pc_s4` writer"]
pub type W = crate::W<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs2PcS4Spec>;
#[doc = "Field `TS2_PC_S4` reader - 13:0\\]
TS2 Sub4 PC"]
pub type Ts2PcS4R = crate::FieldReader<u16>;
#[doc = "Field `TS2_PC_S4` writer - 13:0\\]
TS2 Sub4 PC"]
pub type Ts2PcS4W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
TS2 Sub4 PC"]
    #[inline(always)]
    pub fn ts2_pc_s4(&self) -> Ts2PcS4R {
        Ts2PcS4R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
TS2 Sub4 PC"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_pc_s4(
        &mut self,
    ) -> Ts2PcS4W<Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs2PcS4Spec> {
        Ts2PcS4W::new(self, 0)
    }
}
#[doc = "TS2 Sub4 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_pru_tx0__pr1_tasks_mgr_pru_tx0_mmr__regs_ts2_pc_s4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_pru_tx0__pr1_tasks_mgr_pru_tx0_mmr__regs_ts2_pc_s4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs2PcS4Spec;
impl crate::RegisterSpec for Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs2PcS4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_tasks_mgr_pru_tx0__pr1_tasks_mgr_pru_tx0_mmr__regs_ts2_pc_s4::R`](R) reader structure"]
impl crate::Readable for Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs2PcS4Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_tasks_mgr_pru_tx0__pr1_tasks_mgr_pru_tx0_mmr__regs_ts2_pc_s4::W`](W) writer structure"]
impl crate::Writable for Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs2PcS4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_TASKS_MGR_PRU_TX0__PR1_TASKS_MGR_PRU_TX0_MMR__REGS_ts2_pc_s4 to value 0"]
impl crate::Resettable for Pr1TasksMgrPruTx0_Pr1TasksMgrPruTx0Mmr_RegsTs2PcS4Spec {
    const RESET_VALUE: u32 = 0;
}
