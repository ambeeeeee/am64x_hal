#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEMASTR` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmhemastrSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEMASTR` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmhemastrSpec>;
#[doc = "Field `MASTER` reader - "]
pub type MasterR = crate::FieldReader<u16>;
#[doc = "Field `MASTER` writer - "]
pub type MasterW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MasterW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhemastrSpec> {
        MasterW::new(self, 0)
    }
}
#[doc = "Indicates the STPv2 master number of hardware event trace. This number is the master number presented in STPv2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemastr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemastr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmhemastrSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmhemastrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemastr::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmhemastrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemastr::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmhemastrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEMASTR to value 0x0128"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmhemastrSpec {
    const RESET_VALUE: u32 = 0x0128;
}
