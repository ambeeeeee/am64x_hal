#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPSCR` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmspscrSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPSCR` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmspscrSpec>;
#[doc = "Field `PORTCTL` reader - "]
pub type PortctlR = crate::FieldReader;
#[doc = "Field `PORTCTL` writer - "]
pub type PortctlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PORTSEL` reader - "]
pub type PortselR = crate::FieldReader<u16>;
#[doc = "Field `PORTSEL` writer - "]
pub type PortselW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn portctl(&self) -> PortctlR {
        PortctlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn portsel(&self) -> PortselR {
        PortselR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn portctl(&mut self) -> PortctlW<Vbusp2apbWrap_CxstmCfg_StmRegsStmspscrSpec> {
        PortctlW::new(self, 0)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    #[must_use]
    pub fn portsel(&mut self) -> PortselW<Vbusp2apbWrap_CxstmCfg_StmRegsStmspscrSpec> {
        PortselW::new(self, 20)
    }
}
#[doc = "This register allows a debugger to program which stimulus ports the STMSPER and STMSPTER apply to.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmspscrSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmspscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspscr::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmspscrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspscr::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmspscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPSCR to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmspscrSpec {
    const RESET_VALUE: u32 = 0;
}
