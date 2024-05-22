#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBCTR0` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0Spec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBCTR0` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0Spec>;
#[doc = "Field `ATVALIDM_W` reader - "]
pub type AtvalidmWR = crate::BitReader;
#[doc = "Field `ATVALIDM_W` writer - "]
pub type AtvalidmWW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFREADYM_W` reader - "]
pub type AfreadymWR = crate::BitReader;
#[doc = "Field `AFREADYM_W` writer - "]
pub type AfreadymWW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATBYTESM_W` reader - "]
pub type AtbytesmWR = crate::FieldReader;
#[doc = "Field `ATBYTESM_W` writer - "]
pub type AtbytesmWW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn atvalidm_w(&self) -> AtvalidmWR {
        AtvalidmWR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn afreadym_w(&self) -> AfreadymWR {
        AfreadymWR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn atbytesm_w(&self) -> AtbytesmWR {
        AtbytesmWR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn atvalidm_w(&mut self) -> AtvalidmWW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0Spec> {
        AtvalidmWW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn afreadym_w(&mut self) -> AfreadymWW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0Spec> {
        AfreadymWW::new(self, 1)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn atbytesm_w(&mut self) -> AtbytesmWW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0Spec> {
        AtbytesmWW::new(self, 8)
    }
}
#[doc = "Controls the value of the ATVALIDM, AFREADYM, and ATBYTESM outputs in integration mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0Spec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr0::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0Spec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbctr0::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBCTR0 to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbctr0Spec {
    const RESET_VALUE: u32 = 0;
}
