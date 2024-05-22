#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPTRIGCSR` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsrSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPTRIGCSR` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsrSpec>;
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
#[doc = "Field `ATBTRIGEN_TE` reader - "]
pub type AtbtrigenTeR = crate::BitReader;
#[doc = "Field `ATBTRIGEN_TE` writer - "]
pub type AtbtrigenTeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATBTRIGEN_DIR` reader - "]
pub type AtbtrigenDirR = crate::BitReader;
#[doc = "Field `ATBTRIGEN_DIR` writer - "]
pub type AtbtrigenDirW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn trigctl(&self) -> TrigctlR {
        TrigctlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn trigstatus(&self) -> TrigstatusR {
        TrigstatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn trigclear(&self) -> TrigclearR {
        TrigclearR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn atbtrigen_te(&self) -> AtbtrigenTeR {
        AtbtrigenTeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn atbtrigen_dir(&self) -> AtbtrigenDirR {
        AtbtrigenDirR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn trigctl(&mut self) -> TrigctlW<Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsrSpec> {
        TrigctlW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn trigstatus(&mut self) -> TrigstatusW<Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsrSpec> {
        TrigstatusW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn trigclear(&mut self) -> TrigclearW<Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsrSpec> {
        TrigclearW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn atbtrigen_te(&mut self) -> AtbtrigenTeW<Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsrSpec> {
        AtbtrigenTeW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn atbtrigen_dir(
        &mut self,
    ) -> AtbtrigenDirW<Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsrSpec> {
        AtbtrigenDirW::new(self, 4)
    }
}
#[doc = "This register is used to control the STM triggers caused by STMSPTER.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsptrigcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsptrigcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsrSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsptrigcsr::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmsptrigcsr::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPTRIGCSR to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmsptrigcsrSpec {
    const RESET_VALUE: u32 = 0;
}
