#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT3R` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat3rSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT3R` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat3rSpec>;
#[doc = "Field `NUMMAST` reader - "]
pub type NummastR = crate::FieldReader;
#[doc = "Field `NUMMAST` writer - "]
pub type NummastW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn nummast(&self) -> NummastR {
        NummastR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn nummast(&mut self) -> NummastW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat3rSpec> {
        NummastW::new(self, 0)
    }
}
#[doc = "Indicates the features of the STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat3r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat3r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat3rSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat3rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat3r::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat3rSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat3r::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat3rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT3R to value 0x0127"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat3rSpec {
    const RESET_VALUE: u32 = 0x0127;
}
