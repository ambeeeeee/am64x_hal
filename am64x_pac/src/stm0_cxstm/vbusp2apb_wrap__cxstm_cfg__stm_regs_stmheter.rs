#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHETER` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmheterSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHETER` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmheterSpec>;
#[doc = "Field `HETE` reader - "]
pub type HeteR = crate::FieldReader<u32>;
#[doc = "Field `HETE` writer - "]
pub type HeteW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hete(&self) -> HeteR {
        HeteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn hete(&mut self) -> HeteW<Vbusp2apbWrap_CxstmCfg_StmRegsStmheterSpec> {
        HeteW::new(self, 0)
    }
}
#[doc = "This register is used to enable trigger generation on hardware events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmheterSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmheterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheter::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmheterSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmheter::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmheterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHETER to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmheterSpec {
    const RESET_VALUE: u32 = 0;
}
