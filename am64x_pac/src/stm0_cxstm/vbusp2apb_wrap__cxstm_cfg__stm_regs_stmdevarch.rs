#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVARCH` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarchSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVARCH` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarchSpec>;
#[doc = "Field `ARCHID` reader - "]
pub type ArchidR = crate::FieldReader<u16>;
#[doc = "Field `ARCHID` writer - "]
pub type ArchidW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `REVISION` reader - "]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `REVISION` writer - "]
pub type RevisionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRESENT` reader - "]
pub type PresentR = crate::BitReader;
#[doc = "Field `PRESENT` writer - "]
pub type PresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARCHITECT` reader - "]
pub type ArchitectR = crate::FieldReader<u16>;
#[doc = "Field `ARCHITECT` writer - "]
pub type ArchitectW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    pub fn archid(&self) -> ArchidR {
        ArchidR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn present(&self) -> PresentR {
        PresentR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn architect(&self) -> ArchitectR {
        ArchitectR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    #[must_use]
    pub fn archid(&mut self) -> ArchidW<Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarchSpec> {
        ArchidW::new(self, 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> RevisionW<Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarchSpec> {
        RevisionW::new(self, 16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PresentW<Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarchSpec> {
        PresentW::new(self, 20)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    #[must_use]
    pub fn architect(&mut self) -> ArchitectW<Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarchSpec> {
        ArchitectW::new(self, 21)
    }
}
#[doc = "Indicates the architect and architecture of the STM. For the STM-500, the architect is ARM, and the architecture is STMv1.1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevarch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevarch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarchSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevarch::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarchSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmdevarch::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMDEVARCH to value 0xae31_2659"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmdevarchSpec {
    const RESET_VALUE: u32 = 0xae31_2659;
}
