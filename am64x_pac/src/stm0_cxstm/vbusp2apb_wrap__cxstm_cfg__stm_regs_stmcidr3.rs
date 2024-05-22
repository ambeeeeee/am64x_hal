#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCIDR3` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr3Spec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCIDR3` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr3Spec>;
#[doc = "Field `PRMBL_3` reader - "]
pub type Prmbl3R = crate::FieldReader;
#[doc = "Field `PRMBL_3` writer - "]
pub type Prmbl3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn prmbl_3(&self) -> Prmbl3R {
        Prmbl3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn prmbl_3(&mut self) -> Prmbl3W<Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr3Spec> {
        Prmbl3W::new(self, 0)
    }
}
#[doc = "A component identification register, that indicates that the identification registers are present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr3Spec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr3::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr3Spec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmcidr3::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMCIDR3 to value 0x0177"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmcidr3Spec {
    const RESET_VALUE: u32 = 0x0177;
}
