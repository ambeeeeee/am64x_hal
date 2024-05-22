#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVTYPE` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmdevtypeSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVTYPE` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmdevtypeSpec>;
#[doc = "Field `MAJOR` reader - "]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - "]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SUB` reader - "]
pub type SubR = crate::FieldReader;
#[doc = "Field `SUB` writer - "]
pub type SubW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn sub(&self) -> SubR {
        SubR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<Vbusp2apbWrap_CxstmCfg_StmRegsStmdevtypeSpec> {
        MajorW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn sub(&mut self) -> SubW<Vbusp2apbWrap_CxstmCfg_StmRegsStmdevtypeSpec> {
        SubW::new(self, 4)
    }
}
#[doc = "Provides a debugger with information about the component when the part number is not recognized. The debugger can then report this information.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevtype::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevtype::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmdevtypeSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmdevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevtype::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdevtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevtype::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdevtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVTYPE to value 0x63"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdevtypeSpec {
    const RESET_VALUE: u32 = 0x63;
}
