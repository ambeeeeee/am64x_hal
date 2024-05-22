#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVID` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmdevidSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVID` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmdevidSpec>;
#[doc = "Field `NUMSP` reader - "]
pub type NumspR = crate::FieldReader<u32>;
#[doc = "Field `NUMSP` writer - "]
pub type NumspW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    pub fn numsp(&self) -> NumspR {
        NumspR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16"]
    #[inline(always)]
    #[must_use]
    pub fn numsp(&mut self) -> NumspW<Vbusp2apbWrap_CxstmCfg_StmRegsStmdevidSpec> {
        NumspW::new(self, 0)
    }
}
#[doc = "Indicates the capabilities of the CoreSight STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmdevidSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmdevidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevid::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdevidSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevid::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdevidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVID to value 0x0006_5536"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdevidSpec {
    const RESET_VALUE: u32 = 0x0006_5536;
}
