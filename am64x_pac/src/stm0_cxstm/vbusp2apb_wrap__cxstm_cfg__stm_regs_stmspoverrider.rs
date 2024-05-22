#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPOVERRIDER` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverriderSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPOVERRIDER` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverriderSpec>;
#[doc = "Field `OVERCTL` reader - "]
pub type OverctlR = crate::FieldReader;
#[doc = "Field `OVERCTL` writer - "]
pub type OverctlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVERTS` reader - "]
pub type OvertsR = crate::BitReader;
#[doc = "Field `OVERTS` writer - "]
pub type OvertsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTSEL` reader - "]
pub type PortselR = crate::FieldReader<u32>;
#[doc = "Field `PORTSEL` writer - "]
pub type PortselW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn overctl(&self) -> OverctlR {
        OverctlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn overts(&self) -> OvertsR {
        OvertsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 15:31"]
    #[inline(always)]
    pub fn portsel(&self) -> PortselR {
        PortselR::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn overctl(&mut self) -> OverctlW<Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverriderSpec> {
        OverctlW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn overts(&mut self) -> OvertsW<Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverriderSpec> {
        OvertsW::new(self, 2)
    }
    #[doc = "Bits 15:31"]
    #[inline(always)]
    #[must_use]
    pub fn portsel(&mut self) -> PortselW<Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverriderSpec> {
        PortselW::new(self, 15)
    }
}
#[doc = "This register allows a debugger to override various features of the STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspoverrider::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspoverrider::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverriderSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverriderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspoverrider::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverriderSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspoverrider::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverriderSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPOVERRIDER to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmspoverriderSpec {
    const RESET_VALUE: u32 = 0;
}
