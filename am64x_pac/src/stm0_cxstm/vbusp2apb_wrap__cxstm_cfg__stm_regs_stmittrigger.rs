#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITTRIGGER` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmittriggerSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITTRIGGER` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmittriggerSpec>;
#[doc = "Field `TRIGOUTSPTE_W` reader - "]
pub type TrigoutspteWR = crate::BitReader;
#[doc = "Field `TRIGOUTSPTE_W` writer - "]
pub type TrigoutspteWW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGOUTSW_W` reader - "]
pub type TrigoutswWR = crate::BitReader;
#[doc = "Field `TRIGOUTSW_W` writer - "]
pub type TrigoutswWW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGOUTHETE_W` reader - "]
pub type TrigoutheteWR = crate::BitReader;
#[doc = "Field `TRIGOUTHETE_W` writer - "]
pub type TrigoutheteWW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCOUT_W` reader - "]
pub type AsyncoutWR = crate::BitReader;
#[doc = "Field `ASYNCOUT_W` writer - "]
pub type AsyncoutWW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn trigoutspte_w(&self) -> TrigoutspteWR {
        TrigoutspteWR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn trigoutsw_w(&self) -> TrigoutswWR {
        TrigoutswWR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn trigouthete_w(&self) -> TrigoutheteWR {
        TrigoutheteWR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn asyncout_w(&self) -> AsyncoutWR {
        AsyncoutWR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn trigoutspte_w(
        &mut self,
    ) -> TrigoutspteWW<Vbusp2apbWrap_CxstmCfg_StmRegsStmittriggerSpec> {
        TrigoutspteWW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn trigoutsw_w(&mut self) -> TrigoutswWW<Vbusp2apbWrap_CxstmCfg_StmRegsStmittriggerSpec> {
        TrigoutswWW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn trigouthete_w(
        &mut self,
    ) -> TrigoutheteWW<Vbusp2apbWrap_CxstmCfg_StmRegsStmittriggerSpec> {
        TrigoutheteWW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn asyncout_w(&mut self) -> AsyncoutWW<Vbusp2apbWrap_CxstmCfg_StmRegsStmittriggerSpec> {
        AsyncoutWW::new(self, 3)
    }
}
#[doc = "Integration Test for Cross-Trigger Outputs Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmittrigger::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmittrigger::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmittriggerSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmittriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmittrigger::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmittriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmittrigger::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmittriggerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITTRIGGER to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmittriggerSpec {
    const RESET_VALUE: u32 = 0;
}
