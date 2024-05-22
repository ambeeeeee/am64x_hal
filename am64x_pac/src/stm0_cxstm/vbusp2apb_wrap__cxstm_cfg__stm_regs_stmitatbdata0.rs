#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBDATA0` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBDATA0` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec>;
#[doc = "Field `ATDATAM0_W` reader - "]
pub type Atdatam0WR = crate::BitReader;
#[doc = "Field `ATDATAM0_W` writer - "]
pub type Atdatam0WW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATDATAM7_W` reader - "]
pub type Atdatam7WR = crate::BitReader;
#[doc = "Field `ATDATAM7_W` writer - "]
pub type Atdatam7WW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATDATAM15_W` reader - "]
pub type Atdatam15WR = crate::BitReader;
#[doc = "Field `ATDATAM15_W` writer - "]
pub type Atdatam15WW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATDATAM23_W` reader - "]
pub type Atdatam23WR = crate::BitReader;
#[doc = "Field `ATDATAM23_W` writer - "]
pub type Atdatam23WW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATDATAM31_W` reader - "]
pub type Atdatam31WR = crate::BitReader;
#[doc = "Field `ATDATAM31_W` writer - "]
pub type Atdatam31WW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATDATAM39_W` reader - "]
pub type Atdatam39WR = crate::BitReader;
#[doc = "Field `ATDATAM39_W` writer - "]
pub type Atdatam39WW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATDATAM47_W` reader - "]
pub type Atdatam47WR = crate::BitReader;
#[doc = "Field `ATDATAM47_W` writer - "]
pub type Atdatam47WW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATDATAM55_W` reader - "]
pub type Atdatam55WR = crate::BitReader;
#[doc = "Field `ATDATAM55_W` writer - "]
pub type Atdatam55WW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATDATAM63_W` reader - "]
pub type Atdatam63WR = crate::BitReader;
#[doc = "Field `ATDATAM63_W` writer - "]
pub type Atdatam63WW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn atdatam0_w(&self) -> Atdatam0WR {
        Atdatam0WR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn atdatam7_w(&self) -> Atdatam7WR {
        Atdatam7WR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn atdatam15_w(&self) -> Atdatam15WR {
        Atdatam15WR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn atdatam23_w(&self) -> Atdatam23WR {
        Atdatam23WR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn atdatam31_w(&self) -> Atdatam31WR {
        Atdatam31WR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn atdatam39_w(&self) -> Atdatam39WR {
        Atdatam39WR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn atdatam47_w(&self) -> Atdatam47WR {
        Atdatam47WR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn atdatam55_w(&self) -> Atdatam55WR {
        Atdatam55WR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn atdatam63_w(&self) -> Atdatam63WR {
        Atdatam63WR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam0_w(&mut self) -> Atdatam0WW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec> {
        Atdatam0WW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam7_w(&mut self) -> Atdatam7WW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec> {
        Atdatam7WW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam15_w(&mut self) -> Atdatam15WW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec> {
        Atdatam15WW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam23_w(&mut self) -> Atdatam23WW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec> {
        Atdatam23WW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam31_w(&mut self) -> Atdatam31WW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec> {
        Atdatam31WW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam39_w(&mut self) -> Atdatam39WW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec> {
        Atdatam39WW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam47_w(&mut self) -> Atdatam47WW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec> {
        Atdatam47WW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam55_w(&mut self) -> Atdatam55WW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec> {
        Atdatam55WW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn atdatam63_w(&mut self) -> Atdatam63WW<Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec> {
        Atdatam63WW::new(self, 8)
    }
}
#[doc = "Controls the value of the ATDATAM output in integration mode:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbdata0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbdata0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbdata0::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmitatbdata0::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMITATBDATA0 to value 0"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmitatbdata0Spec {
    const RESET_VALUE: u32 = 0;
}
