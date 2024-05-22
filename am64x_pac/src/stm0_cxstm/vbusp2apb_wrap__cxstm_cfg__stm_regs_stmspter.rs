#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPTER` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmspterSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPTER` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmspterSpec>;
#[doc = "Field `SPTE` reader - "]
pub type SpteR = crate::FieldReader<u32>;
#[doc = "Field `SPTE` writer - "]
pub type SpteW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spte(&self) -> SpteR {
        SpteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn spte(&mut self) -> SpteW<Vbusp2apbWrap_CxstmCfg_StmRegsStmspterSpec> {
        SpteW::new(self, 0)
    }
}
#[doc = "This register is used to enable trigger generation on writes to enabled stimulus port registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmspterSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmspterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspter::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmspterSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspter::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmspterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPTER to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmspterSpec {
    const RESET_VALUE: u32 = 0;
}
