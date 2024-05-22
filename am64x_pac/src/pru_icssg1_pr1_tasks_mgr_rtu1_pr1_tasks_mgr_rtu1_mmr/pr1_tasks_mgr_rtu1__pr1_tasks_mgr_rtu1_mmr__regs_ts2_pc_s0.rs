#[doc = "Register `PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_pc_s0` reader"]
pub type R = crate::R<Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS0Spec>;
#[doc = "Register `PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_pc_s0` writer"]
pub type W = crate::W<Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS0Spec>;
#[doc = "Field `TS2_PC_S0` reader - 13:0\\]
TS2 Sub0 PC"]
pub type Ts2PcS0R = crate::FieldReader<u16>;
#[doc = "Field `TS2_PC_S0` writer - 13:0\\]
TS2 Sub0 PC"]
pub type Ts2PcS0W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
TS2 Sub0 PC"]
    #[inline(always)]
    pub fn ts2_pc_s0(&self) -> Ts2PcS0R {
        Ts2PcS0R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
TS2 Sub0 PC"]
    #[inline(always)]
    #[must_use]
    pub fn ts2_pc_s0(&mut self) -> Ts2PcS0W<Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS0Spec> {
        Ts2PcS0W::new(self, 0)
    }
}
#[doc = "TS2 Sub0 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS0Spec;
impl crate::RegisterSpec for Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s0::R`](R) reader structure"]
impl crate::Readable for Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s0::W`](W) writer structure"]
impl crate::Writable for Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_pc_s0 to value 0"]
impl crate::Resettable for Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS0Spec {
    const RESET_VALUE: u32 = 0;
}
