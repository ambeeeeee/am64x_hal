#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSYNCR` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmsyncrSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSYNCR` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmsyncrSpec>;
#[doc = "Field `COUNT` reader - "]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `COUNT` writer - "]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `MODE` reader - "]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - "]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 3:11"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:11"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<Vbusp2apbWrap_CxstmCfg_StmRegsStmsyncrSpec> {
        CountW::new(self, 3)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<Vbusp2apbWrap_CxstmCfg_StmRegsStmsyncrSpec> {
        ModeW::new(self, 12)
    }
}
#[doc = "This register controls the interval between synchronization packets, in terms of the number of bytes of trace generated.&lt;p/>This register only provides a hint of the desired synchronization frequency, implementations are permitted to be inaccurate.&lt;p/>Writing a value of 0x00000000 to this register disables the synchronization counter however any other IMPLEMENTATION DEFINED synchronizations mechanism continue to operate independently.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsyncr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsyncr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmsyncrSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmsyncrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsyncr::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmsyncrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsyncr::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmsyncrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSYNCR to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmsyncrSpec {
    const RESET_VALUE: u32 = 0;
}
