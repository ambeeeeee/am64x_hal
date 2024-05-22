#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPER` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmsperSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPER` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmsperSpec>;
#[doc = "Field `SPE` reader - "]
pub type SpeR = crate::FieldReader<u32>;
#[doc = "Field `SPE` writer - "]
pub type SpeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spe(&self) -> SpeR {
        SpeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SpeW<Vbusp2apbWrap_CxstmCfg_StmRegsStmsperSpec> {
        SpeW::new(self, 0)
    }
}
#[doc = "This read/write only register is used to enable the stimulus registers to generate trace.&lt;p/>The register defines one bit per stimulus register. Writing 1 enables the appropriate stimulus port, writing 0 disables the appropriate stimulus port. This register is used in conjunction with the Software Enable Bank Select Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmsperSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmsperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsper::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmsperSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsper::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmsperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPER to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmsperSpec {
    const RESET_VALUE: u32 = 0;
}
