#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR1` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr1Spec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR1` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr1Spec>;
#[doc = "Field `PART_1` reader - "]
pub type Part1R = crate::FieldReader;
#[doc = "Field `PART_1` writer - "]
pub type Part1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DES_0` reader - "]
pub type Des0R = crate::FieldReader;
#[doc = "Field `DES_0` writer - "]
pub type Des0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn part_1(&self) -> Part1R {
        Part1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn des_0(&self) -> Des0R {
        Des0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn part_1(&mut self) -> Part1W<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr1Spec> {
        Part1W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn des_0(&mut self) -> Des0W<Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr1Spec> {
        Des0W::new(self, 4)
    }
}
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr1Spec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr1::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr1Spec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmpidr1::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMPIDR1 to value 0x0119"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmpidr1Spec {
    const RESET_VALUE: u32 = 0x0119;
}
