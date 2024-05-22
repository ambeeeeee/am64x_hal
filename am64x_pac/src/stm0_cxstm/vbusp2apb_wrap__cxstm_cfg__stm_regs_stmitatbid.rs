#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBID` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbidSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBID` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbidSpec>;
#[doc = "Field `ATIDM_W` reader - "]
pub type AtidmWR = crate::FieldReader;
#[doc = "Field `ATIDM_W` writer - "]
pub type AtidmWW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn atidm_w(&self) -> AtidmWR {
        AtidmWR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn atidm_w(&mut self) -> AtidmWW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbidSpec> {
        AtidmWW::new(self, 0)
    }
}
#[doc = "Controls the value of the ATIDM output in integration mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbidSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbid::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbidSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbid::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBID to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbidSpec {
    const RESET_VALUE: u32 = 0;
}
