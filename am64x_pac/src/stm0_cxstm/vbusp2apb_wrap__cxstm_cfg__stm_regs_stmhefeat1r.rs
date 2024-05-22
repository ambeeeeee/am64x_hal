#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEFEAT1R` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEFEAT1R` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec>;
#[doc = "Field `HETER` reader - "]
pub type HeterR = crate::BitReader;
#[doc = "Field `HETER` writer - "]
pub type HeterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEERR` reader - "]
pub type HeerrR = crate::BitReader;
#[doc = "Field `HEERR` writer - "]
pub type HeerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEMASTR` reader - "]
pub type HemastrR = crate::BitReader;
#[doc = "Field `HEMASTR` writer - "]
pub type HemastrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HECOMP` reader - "]
pub type HecompR = crate::FieldReader;
#[doc = "Field `HECOMP` writer - "]
pub type HecompW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NUMHE` reader - "]
pub type NumheR = crate::FieldReader<u16>;
#[doc = "Field `NUMHE` writer - "]
pub type NumheW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `HEEXTMUXSIZE` reader - "]
pub type HeextmuxsizeR = crate::FieldReader;
#[doc = "Field `HEEXTMUXSIZE` writer - "]
pub type HeextmuxsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn heter(&self) -> HeterR {
        HeterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn heerr(&self) -> HeerrR {
        HeerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn hemastr(&self) -> HemastrR {
        HemastrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn hecomp(&self) -> HecompR {
        HecompR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 15:23"]
    #[inline(always)]
    pub fn numhe(&self) -> NumheR {
        NumheR::new(((self.bits >> 15) & 0x01ff) as u16)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn heextmuxsize(&self) -> HeextmuxsizeR {
        HeextmuxsizeR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn heter(&mut self) -> HeterW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec> {
        HeterW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn heerr(&mut self) -> HeerrW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec> {
        HeerrW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn hemastr(&mut self) -> HemastrW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec> {
        HemastrW::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn hecomp(&mut self) -> HecompW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec> {
        HecompW::new(self, 4)
    }
    #[doc = "Bits 15:23"]
    #[inline(always)]
    #[must_use]
    pub fn numhe(&mut self) -> NumheW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec> {
        NumheW::new(self, 15)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn heextmuxsize(&mut self) -> HeextmuxsizeW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec> {
        HeextmuxsizeW::new(self, 28)
    }
}
#[doc = "Indicates the features of the STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhefeat1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhefeat1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhefeat1r::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhefeat1r::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEFEAT1R to value 0x0032_0035"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmhefeat1rSpec {
    const RESET_VALUE: u32 = 0x0032_0035;
}
