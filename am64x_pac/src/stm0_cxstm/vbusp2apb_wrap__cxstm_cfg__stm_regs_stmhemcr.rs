#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEMCR` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEMCR` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec>;
#[doc = "Field `EN` reader - "]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - "]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEN` reader - "]
pub type CompenR = crate::BitReader;
#[doc = "Field `COMPEN` writer - "]
pub type CompenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRDETECT` reader - "]
pub type ErrdetectR = crate::BitReader;
#[doc = "Field `ERRDETECT` writer - "]
pub type ErrdetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGCTL` reader - "]
pub type TrigctlR = crate::BitReader;
#[doc = "Field `TRIGCTL` writer - "]
pub type TrigctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGSTATUS` reader - "]
pub type TrigstatusR = crate::BitReader;
#[doc = "Field `TRIGSTATUS` writer - "]
pub type TrigstatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGCLEAR` reader - "]
pub type TrigclearR = crate::BitReader;
#[doc = "Field `TRIGCLEAR` writer - "]
pub type TrigclearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATBTRIGEN` reader - "]
pub type AtbtrigenR = crate::BitReader;
#[doc = "Field `ATBTRIGEN` writer - "]
pub type AtbtrigenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn compen(&self) -> CompenR {
        CompenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn errdetect(&self) -> ErrdetectR {
        ErrdetectR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn trigctl(&self) -> TrigctlR {
        TrigctlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trigstatus(&self) -> TrigstatusR {
        TrigstatusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn trigclear(&self) -> TrigclearR {
        TrigclearR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn atbtrigen(&self) -> AtbtrigenR {
        AtbtrigenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn compen(&mut self) -> CompenW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec> {
        CompenW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn errdetect(&mut self) -> ErrdetectW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec> {
        ErrdetectW::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn trigctl(&mut self) -> TrigctlW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec> {
        TrigctlW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn trigstatus(&mut self) -> TrigstatusW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec> {
        TrigstatusW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn trigclear(&mut self) -> TrigclearW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec> {
        TrigclearW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn atbtrigen(&mut self) -> AtbtrigenW<Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec> {
        AtbtrigenW::new(self, 7)
    }
}
#[doc = "This register is used to control the primary functions of Hardware Event tracing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemcr::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmhemcr::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMHEMCR to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmhemcrSpec {
    const RESET_VALUE: u32 = 0;
}
