#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBCTR2` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr2Spec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBCTR2` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr2Spec>;
#[doc = "Field `ATREADYM_R` reader - "]
pub type AtreadymRR = crate::BitReader;
#[doc = "Field `ATREADYM_R` writer - "]
pub type AtreadymRW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFVALIDM_R` reader - "]
pub type AfvalidmRR = crate::BitReader;
#[doc = "Field `AFVALIDM_R` writer - "]
pub type AfvalidmRW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn atreadym_r(&self) -> AtreadymRR {
        AtreadymRR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn afvalidm_r(&self) -> AfvalidmRR {
        AfvalidmRR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn atreadym_r(&mut self) -> AtreadymRW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr2Spec> {
        AtreadymRW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn afvalidm_r(&mut self) -> AfvalidmRW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr2Spec> {
        AfvalidmRW::new(self, 1)
    }
}
#[doc = "Returns the value of the ATREADYM and AFVALIDM inputs in integration mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr2Spec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr2::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr2Spec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr2::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBCTR2 to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr2Spec {
    const RESET_VALUE: u32 = 0;
}
