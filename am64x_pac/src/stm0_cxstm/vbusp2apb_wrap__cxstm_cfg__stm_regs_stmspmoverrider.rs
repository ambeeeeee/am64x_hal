#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPMOVERRIDER` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmspmoverriderSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPMOVERRIDER` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmspmoverriderSpec>;
#[doc = "Field `MASTCTL` reader - "]
pub type MastctlR = crate::BitReader;
#[doc = "Field `MASTCTL` writer - "]
pub type MastctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTSEL` reader - "]
pub type MastselR = crate::FieldReader;
#[doc = "Field `MASTSEL` writer - "]
pub type MastselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mastctl(&self) -> MastctlR {
        MastctlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 15:22"]
    #[inline(always)]
    pub fn mastsel(&self) -> MastselR {
        MastselR::new(((self.bits >> 15) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn mastctl(&mut self) -> MastctlW<Vbusp2apbWrap_CxstmCfg_StmRegsStmspmoverriderSpec> {
        MastctlW::new(self, 0)
    }
    #[doc = "Bits 15:22"]
    #[inline(always)]
    #[must_use]
    pub fn mastsel(&mut self) -> MastselW<Vbusp2apbWrap_CxstmCfg_StmRegsStmspmoverriderSpec> {
        MastselW::new(self, 15)
    }
}
#[doc = "This register allows a debugger to choose which masters the STMSPOVERRIDERR applies to.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmoverrider::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmoverrider::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmspmoverriderSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmspmoverriderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmoverrider::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmspmoverriderSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmspmoverrider::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmspmoverriderSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMSPMOVERRIDER to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmspmoverriderSpec {
    const RESET_VALUE: u32 = 0;
}
