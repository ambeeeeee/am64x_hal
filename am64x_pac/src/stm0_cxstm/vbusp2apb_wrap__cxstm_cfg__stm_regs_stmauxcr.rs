#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMAUXCR` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcrSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMAUXCR` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcrSpec>;
#[doc = "Field `FIFOAF` reader - "]
pub type FifoafR = crate::BitReader;
#[doc = "Field `FIFOAF` writer - "]
pub type FifoafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCPE` reader - "]
pub type AsyncpeR = crate::BitReader;
#[doc = "Field `ASYNCPE` writer - "]
pub type AsyncpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIORINVDIS` reader - "]
pub type PriorinvdisR = crate::BitReader;
#[doc = "Field `PRIORINVDIS` writer - "]
pub type PriorinvdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QHWEVOVERRIDE` reader - "]
pub type QhwevoverrideR = crate::BitReader;
#[doc = "Field `QHWEVOVERRIDE` writer - "]
pub type QhwevoverrideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifoaf(&self) -> FifoafR {
        FifoafR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn asyncpe(&self) -> AsyncpeR {
        AsyncpeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn priorinvdis(&self) -> PriorinvdisR {
        PriorinvdisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn qhwevoverride(&self) -> QhwevoverrideR {
        QhwevoverrideR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fifoaf(&mut self) -> FifoafW<Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcrSpec> {
        FifoafW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn asyncpe(&mut self) -> AsyncpeW<Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcrSpec> {
        AsyncpeW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn priorinvdis(&mut self) -> PriorinvdisW<Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcrSpec> {
        PriorinvdisW::new(self, 2)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn qhwevoverride(&mut self) -> QhwevoverrideW<Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcrSpec> {
        QhwevoverrideW::new(self, 7)
    }
}
#[doc = "Used for IMPLEMENTATION DEFINED STM controls.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauxcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauxcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcrSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauxcr::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmauxcr::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMAUXCR to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmauxcrSpec {
    const RESET_VALUE: u32 = 0;
}
