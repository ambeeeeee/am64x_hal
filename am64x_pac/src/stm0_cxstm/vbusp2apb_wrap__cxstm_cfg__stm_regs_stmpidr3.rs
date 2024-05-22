#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR3` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr3Spec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR3` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr3Spec>;
#[doc = "Field `CMOD` reader - "]
pub type CmodR = crate::FieldReader;
#[doc = "Field `CMOD` writer - "]
pub type CmodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REVAND` reader - "]
pub type RevandR = crate::FieldReader;
#[doc = "Field `REVAND` writer - "]
pub type RevandW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cmod(&self) -> CmodR {
        CmodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn revand(&self) -> RevandR {
        RevandR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn cmod(&mut self) -> CmodW<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr3Spec> {
        CmodW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn revand(&mut self) -> RevandW<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr3Spec> {
        RevandW::new(self, 4)
    }
}
#[doc = "Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer_Modified bit fields.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr3Spec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr3::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr3Spec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr3::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR3 to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr3Spec {
    const RESET_VALUE: u32 = 0;
}
