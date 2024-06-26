#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMASTOPR` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastoprSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMASTOPR` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastoprSpec>;
#[doc = "Field `STOP` reader - "]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - "]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastoprSpec> {
        StopW::new(self, 0)
    }
}
#[doc = "This write-only register is used to stop a DMA transfer.&lt;p/>A write of one stops an active DMA transfer. A write of zero has no effect. A write of one when the DMA peripheral request interface is idle has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastopr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastopr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastoprSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastoprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastopr::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastoprSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdmastopr::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastoprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDMASTOPR to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdmastoprSpec {
    const RESET_VALUE: u32 = 0;
}
