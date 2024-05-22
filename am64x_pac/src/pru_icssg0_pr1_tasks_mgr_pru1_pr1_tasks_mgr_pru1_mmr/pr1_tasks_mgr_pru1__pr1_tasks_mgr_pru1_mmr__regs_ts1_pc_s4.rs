#[doc = "Register `PR1_TASKS_MGR_PRU1__PR1_TASKS_MGR_PRU1_MMR__REGS_ts1_pc_s4` reader"]
pub type R = crate::R<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs1PcS4Spec>;
#[doc = "Register `PR1_TASKS_MGR_PRU1__PR1_TASKS_MGR_PRU1_MMR__REGS_ts1_pc_s4` writer"]
pub type W = crate::W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs1PcS4Spec>;
#[doc = "Field `TS1_PC_S4` reader - 13:0\\]
TS1 Sub4 PC"]
pub type Ts1PcS4R = crate::FieldReader<u16>;
#[doc = "Field `TS1_PC_S4` writer - 13:0\\]
TS1 Sub4 PC"]
pub type Ts1PcS4W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
TS1 Sub4 PC"]
    #[inline(always)]
    pub fn ts1_pc_s4(&self) -> Ts1PcS4R {
        Ts1PcS4R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
TS1 Sub4 PC"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_pc_s4(&mut self) -> Ts1PcS4W<Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs1PcS4Spec> {
        Ts1PcS4W::new(self, 0)
    }
}
#[doc = "TS1 Sub4 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_pru1__pr1_tasks_mgr_pru1_mmr__regs_ts1_pc_s4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_pru1__pr1_tasks_mgr_pru1_mmr__regs_ts1_pc_s4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs1PcS4Spec;
impl crate::RegisterSpec for Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs1PcS4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_tasks_mgr_pru1__pr1_tasks_mgr_pru1_mmr__regs_ts1_pc_s4::R`](R) reader structure"]
impl crate::Readable for Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs1PcS4Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_tasks_mgr_pru1__pr1_tasks_mgr_pru1_mmr__regs_ts1_pc_s4::W`](W) writer structure"]
impl crate::Writable for Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs1PcS4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_TASKS_MGR_PRU1__PR1_TASKS_MGR_PRU1_MMR__REGS_ts1_pc_s4 to value 0"]
impl crate::Resettable for Pr1TasksMgrPru1_Pr1TasksMgrPru1Mmr_RegsTs1PcS4Spec {
    const RESET_VALUE: u32 = 0;
}
