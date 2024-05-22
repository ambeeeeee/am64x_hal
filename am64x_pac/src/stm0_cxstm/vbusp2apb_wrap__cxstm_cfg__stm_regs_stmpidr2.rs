#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR2` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2Spec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR2` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2Spec>;
#[doc = "Field `DES_1` reader - "]
pub type Des1R = crate::FieldReader;
#[doc = "Field `DES_1` writer - "]
pub type Des1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JEDEC` reader - "]
pub type JedecR = crate::BitReader;
#[doc = "Field `JEDEC` writer - "]
pub type JedecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REVISION` reader - "]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - "]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn des_1(&self) -> Des1R {
        Des1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn jedec(&self) -> JedecR {
        JedecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn des_1(&mut self) -> Des1W<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2Spec> {
        Des1W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn jedec(&mut self) -> JedecW<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2Spec> {
        JedecW::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2Spec> {
        RevisionW::new(self, 4)
    }
}
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2Spec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr2::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2Spec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr2::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR2 to value 0x1b"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr2Spec {
    const RESET_VALUE: u32 = 0x1b;
}
