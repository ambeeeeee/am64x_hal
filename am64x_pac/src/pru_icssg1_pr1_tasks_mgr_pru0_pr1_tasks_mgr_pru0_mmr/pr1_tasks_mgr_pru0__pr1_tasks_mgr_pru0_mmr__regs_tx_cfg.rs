#[doc = "Register `PR1_TASKS_MGR_PRU0__PR1_TASKS_MGR_PRU0_MMR__REGS_tx_cfg` reader"]
pub type R = crate::R<Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTxCfgSpec>;
#[doc = "Register `PR1_TASKS_MGR_PRU0__PR1_TASKS_MGR_PRU0_MMR__REGS_tx_cfg` writer"]
pub type W = crate::W<Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTxCfgSpec>;
#[doc = "Field `TX_WM` reader - 5:0\\]
TX L2 Water Mark Level 1 to 64 Bytes"]
pub type TxWmR = crate::FieldReader;
#[doc = "Field `TX_WM` writer - 5:0\\]
TX L2 Water Mark Level 1 to 64 Bytes"]
pub type TxWmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
TX L2 Water Mark Level 1 to 64 Bytes"]
    #[inline(always)]
    pub fn tx_wm(&self) -> TxWmR {
        TxWmR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
TX L2 Water Mark Level 1 to 64 Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn tx_wm(&mut self) -> TxWmW<Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTxCfgSpec> {
        TxWmW::new(self, 0)
    }
}
#[doc = "TX Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_pru0__pr1_tasks_mgr_pru0_mmr__regs_tx_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_pru0__pr1_tasks_mgr_pru0_mmr__regs_tx_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTxCfgSpec;
impl crate::RegisterSpec for Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTxCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_tasks_mgr_pru0__pr1_tasks_mgr_pru0_mmr__regs_tx_cfg::R`](R) reader structure"]
impl crate::Readable for Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTxCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_tasks_mgr_pru0__pr1_tasks_mgr_pru0_mmr__regs_tx_cfg::W`](W) writer structure"]
impl crate::Writable for Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTxCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_TASKS_MGR_PRU0__PR1_TASKS_MGR_PRU0_MMR__REGS_tx_cfg to value 0x31"]
impl crate::Resettable for Pr1TasksMgrPru0_Pr1TasksMgrPru0Mmr_RegsTxCfgSpec {
    const RESET_VALUE: u32 = 0x31;
}
