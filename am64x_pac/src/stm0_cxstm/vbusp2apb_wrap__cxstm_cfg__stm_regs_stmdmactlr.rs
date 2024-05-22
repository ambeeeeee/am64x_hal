#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMACTLR` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmdmactlrSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMACTLR` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmdmactlrSpec>;
#[doc = "Field `SENS` reader - "]
pub type SensR = crate::FieldReader;
#[doc = "Field `SENS` writer - "]
pub type SensW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sens(&self) -> SensR {
        SensR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn sens(&mut self) -> SensW<Vbusp2apbWrap_CxstmCfg_StmRegsStmdmactlrSpec> {
        SensW::new(self, 2)
    }
}
#[doc = "Controls the DMA transfer request mechanism.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmactlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmactlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmdmactlrSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmdmactlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmactlr::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdmactlrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmactlr::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdmactlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMACTLR to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdmactlrSpec {
    const RESET_VALUE: u32 = 0;
}
