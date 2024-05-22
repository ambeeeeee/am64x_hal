#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR4` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr4Spec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR4` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr4Spec>;
#[doc = "Field `DES_2` reader - "]
pub type Des2R = crate::FieldReader;
#[doc = "Field `DES_2` writer - "]
pub type Des2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIZE` reader - "]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - "]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn des_2(&self) -> Des2R {
        Des2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn des_2(&mut self) -> Des2W<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr4Spec> {
        Des2W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr4Spec> {
        SizeW::new(self, 4)
    }
}
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr4Spec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr4::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr4Spec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr4::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR4 to value 0x04"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr4Spec {
    const RESET_VALUE: u32 = 0x04;
}
