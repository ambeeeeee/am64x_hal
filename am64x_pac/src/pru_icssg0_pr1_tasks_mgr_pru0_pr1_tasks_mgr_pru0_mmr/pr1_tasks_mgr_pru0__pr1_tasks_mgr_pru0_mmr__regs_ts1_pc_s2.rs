#[doc = "Register `PR1_TASKS_MGR_PRU0__PR1_TASKS_MGR_PRU0_MMR__REGS_ts1_pc_s2` reader"]
pub type R = crate::R<Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTs1PcS2Spec>;
#[doc = "Register `PR1_TASKS_MGR_PRU0__PR1_TASKS_MGR_PRU0_MMR__REGS_ts1_pc_s2` writer"]
pub type W = crate::W<Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTs1PcS2Spec>;
#[doc = "Field `TS1_PC_S2` reader - 13:0\\]
TS1 Sub2 PC"]
pub type Ts1PcS2R = crate::FieldReader<u16>;
#[doc = "Field `TS1_PC_S2` writer - 13:0\\]
TS1 Sub2 PC"]
pub type Ts1PcS2W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
TS1 Sub2 PC"]
    #[inline(always)]
    pub fn ts1_pc_s2(&self) -> Ts1PcS2R {
        Ts1PcS2R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
TS1 Sub2 PC"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_pc_s2(&mut self) -> Ts1PcS2W<Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTs1PcS2Spec> {
        Ts1PcS2W::new(self, 0)
    }
}
#[doc = "TS1 Sub2 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_pru0__pr1_tasks_mgr_pru0_mmr__regs_ts1_pc_s2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_pru0__pr1_tasks_mgr_pru0_mmr__regs_ts1_pc_s2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTs1PcS2Spec;
impl crate::RegisterSpec for Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTs1PcS2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_tasks_mgr_pru0__pr1_tasks_mgr_pru0_mmr__regs_ts1_pc_s2::R`](R) reader structure"]
impl crate::Readable for Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTs1PcS2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_tasks_mgr_pru0__pr1_tasks_mgr_pru0_mmr__regs_ts1_pc_s2::W`](W) writer structure"]
impl crate::Writable for Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTs1PcS2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_TASKS_MGR_PRU0__PR1_TASKS_MGR_PRU0_MMR__REGS_ts1_pc_s2 to value 0"]
impl crate::Resettable for Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTs1PcS2Spec {
    const RESET_VALUE: u32 = 0;
}
