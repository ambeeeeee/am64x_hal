#[doc = "Register `PR1_TASKS_MGR_RTU0__PR1_TASKS_MGR_RTU0_MMR__REGS_cap_en_cfg` reader"]
pub type R = crate::R<Pr1TasksMgrRtu0_Pr1TasksMgrRtu0Mmr_RegsCapEnCfgSpec>;
#[doc = "Register `PR1_TASKS_MGR_RTU0__PR1_TASKS_MGR_RTU0_MMR__REGS_cap_en_cfg` writer"]
pub type W = crate::W<Pr1TasksMgrRtu0_Pr1TasksMgrRtu0Mmr_RegsCapEnCfgSpec>;
#[doc = "Field `NEW_CAP_EN` reader - 9:0\\]
Capture new event while in the same task Enable TS1_S0 = \\[0\\]
.. TS2_S4 = \\[9\\]"]
pub type NewCapEnR = crate::FieldReader<u16>;
#[doc = "Field `NEW_CAP_EN` writer - 9:0\\]
Capture new event while in the same task Enable TS1_S0 = \\[0\\]
.. TS2_S4 = \\[9\\]"]
pub type NewCapEnW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Capture new event while in the same task Enable TS1_S0 = \\[0\\]
.. TS2_S4 = \\[9\\]"]
    #[inline(always)]
    pub fn new_cap_en(&self) -> NewCapEnR {
        NewCapEnR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Capture new event while in the same task Enable TS1_S0 = \\[0\\]
.. TS2_S4 = \\[9\\]"]
    #[inline(always)]
    #[must_use]
    pub fn new_cap_en(&mut self) -> NewCapEnW<Pr1TasksMgrRtu0_Pr1TasksMgrRtu0Mmr_RegsCapEnCfgSpec> {
        NewCapEnW::new(self, 0)
    }
}
#[doc = "Enable capture new event cfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu0__pr1_tasks_mgr_rtu0_mmr__regs_cap_en_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu0__pr1_tasks_mgr_rtu0_mmr__regs_cap_en_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1TasksMgrRtu0_Pr1TasksMgrRtu0Mmr_RegsCapEnCfgSpec;
impl crate::RegisterSpec for Pr1TasksMgrRtu0_Pr1TasksMgrRtu0Mmr_RegsCapEnCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_tasks_mgr_rtu0__pr1_tasks_mgr_rtu0_mmr__regs_cap_en_cfg::R`](R) reader structure"]
impl crate::Readable for Pr1TasksMgrRtu0_Pr1TasksMgrRtu0Mmr_RegsCapEnCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_tasks_mgr_rtu0__pr1_tasks_mgr_rtu0_mmr__regs_cap_en_cfg::W`](W) writer structure"]
impl crate::Writable for Pr1TasksMgrRtu0_Pr1TasksMgrRtu0Mmr_RegsCapEnCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_TASKS_MGR_RTU0__PR1_TASKS_MGR_RTU0_MMR__REGS_cap_en_cfg to value 0"]
impl crate::Resettable for Pr1TasksMgrRtu0_Pr1TasksMgrRtu0Mmr_RegsCapEnCfgSpec {
    const RESET_VALUE: u32 = 0;
}
