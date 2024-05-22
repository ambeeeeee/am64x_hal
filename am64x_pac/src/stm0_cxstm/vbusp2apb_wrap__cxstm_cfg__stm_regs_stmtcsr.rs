#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTCSR` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTCSR` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec>;
#[doc = "Field `EN` reader - "]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - "]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEN` reader - "]
pub type TsenR = crate::BitReader;
#[doc = "Field `TSEN` writer - "]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCEN` reader - "]
pub type SyncenR = crate::BitReader;
#[doc = "Field `SYNCEN` writer - "]
pub type SyncenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEN` reader - "]
pub type CompenR = crate::BitReader;
#[doc = "Field `COMPEN` writer - "]
pub type CompenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACEID` reader - "]
pub type TraceidR = crate::FieldReader;
#[doc = "Field `TRACEID` writer - "]
pub type TraceidW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BUSY` reader - "]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - "]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn syncen(&self) -> SyncenR {
        SyncenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn compen(&self) -> CompenR {
        CompenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn traceid(&self) -> TraceidR {
        TraceidR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TsenW<Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec> {
        TsenW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SyncenW<Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec> {
        SyncenW::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn compen(&mut self) -> CompenW<Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec> {
        CompenW::new(self, 5)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    #[must_use]
    pub fn traceid(&mut self) -> TraceidW<Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec> {
        TraceidW::new(self, 16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec> {
        BusyW::new(self, 23)
    }
}
#[doc = "Controls the STM settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtcsr::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmtcsr::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMTCSR to value 0x04"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmtcsrSpec {
    const RESET_VALUE: u32 = 0x04;
}
