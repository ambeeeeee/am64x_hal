#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMLSR` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmlsrSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMLSR` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmlsrSpec>;
#[doc = "Field `SLI` reader - "]
pub type SliR = crate::BitReader;
#[doc = "Field `SLI` writer - "]
pub type SliW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLK` reader - "]
pub type SlkR = crate::BitReader;
#[doc = "Field `SLK` writer - "]
pub type SlkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NTT` reader - "]
pub type NttR = crate::BitReader;
#[doc = "Field `NTT` writer - "]
pub type NttW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sli(&self) -> SliR {
        SliR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slk(&self) -> SlkR {
        SlkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ntt(&self) -> NttR {
        NttR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sli(&mut self) -> SliW<Vbusp2apbWrap_CxstmCfg_StmRegsStmlsrSpec> {
        SliW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn slk(&mut self) -> SlkW<Vbusp2apbWrap_CxstmCfg_StmRegsStmlsrSpec> {
        SlkW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ntt(&mut self) -> NttW<Vbusp2apbWrap_CxstmCfg_StmRegsStmlsrSpec> {
        NttW::new(self, 2)
    }
}
#[doc = "Indicates the status of the lock control mechanism. This lock prevents accidental writes by code under debug. The lock mechanism does not impact accesses to the extended stimulus port registers. This register must always be present although there might not be any lock access control mechanism. The lock mechanism, where present and locked, blocks write accesses to any register, except the STMLAR. The lock mechanism is only present for accesses with the PADDRDBG31 signal LOW.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmlsrSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmlsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlsr::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmlsrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmlsr::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmlsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMLSR to value 0x03"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmlsrSpec {
    const RESET_VALUE: u32 = 0x03;
}
