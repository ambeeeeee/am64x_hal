#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTSFREQR` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmtsfreqrSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTSFREQR` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmtsfreqrSpec>;
#[doc = "Field `FREQ` reader - "]
pub type FreqR = crate::FieldReader<u32>;
#[doc = "Field `FREQ` writer - "]
pub type FreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn freq(&self) -> FreqR {
        FreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn freq(&mut self) -> FreqW<Vbusp2apbWrap_CxstmCfg_StmRegsStmtsfreqrSpec> {
        FreqW::new(self, 0)
    }
}
#[doc = "This read-write register is used to indicate the frequency of the timestamp counter. The unit of measurement is increments per second. When the STPv2 protocol is used, this register contains the value output in the FREQ and FREQ_TS packets. The timestamp frequency is output in the STPv2 protocol at every synchronization point when STMTCSR.TSEN is b1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsfreqr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsfreqr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmtsfreqrSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmtsfreqrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsfreqr::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmtsfreqrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtsfreqr::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmtsfreqrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTSFREQR to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmtsfreqrSpec {
    const RESET_VALUE: u32 = 0;
}
