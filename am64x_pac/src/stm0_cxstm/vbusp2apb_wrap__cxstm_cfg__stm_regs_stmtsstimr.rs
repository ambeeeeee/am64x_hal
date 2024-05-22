#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTSSTIMR` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmtsstimrSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTSSTIMR` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmtsstimrSpec>;
#[doc = "Field `FORCETS` reader - "]
pub type ForcetsR = crate::BitReader;
#[doc = "Field `FORCETS` writer - "]
pub type ForcetsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn forcets(&self) -> ForcetsR {
        ForcetsR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn forcets(&mut self) -> ForcetsW<Vbusp2apbWrap_CxstmCfg_StmRegsStmtsstimrSpec> {
        ForcetsW::new(self, 0)
    }
}
#[doc = "This write-only register is used to force the next packet caused by a stimulus port write to have a timestamp output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsstimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsstimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmtsstimrSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmtsstimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsstimr::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmtsstimrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsstimr::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmtsstimrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTSSTIMR to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmtsstimrSpec {
    const RESET_VALUE: u32 = 0;
}
