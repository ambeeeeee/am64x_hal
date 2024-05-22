#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT1R` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT1R` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec>;
#[doc = "Field `PROT` reader - "]
pub type ProtR = crate::FieldReader;
#[doc = "Field `PROT` writer - "]
pub type ProtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TS` reader - "]
pub type TsR = crate::FieldReader;
#[doc = "Field `TS` writer - "]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSFREQ` reader - "]
pub type TsfreqR = crate::BitReader;
#[doc = "Field `TSFREQ` writer - "]
pub type TsfreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCETS` reader - "]
pub type ForcetsR = crate::BitReader;
#[doc = "Field `FORCETS` writer - "]
pub type ForcetsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC` reader - "]
pub type SyncR = crate::FieldReader;
#[doc = "Field `SYNC` writer - "]
pub type SyncW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRACEBUS` reader - "]
pub type TracebusR = crate::FieldReader;
#[doc = "Field `TRACEBUS` writer - "]
pub type TracebusW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIGCTL` reader - "]
pub type TrigctlR = crate::FieldReader;
#[doc = "Field `TRIGCTL` writer - "]
pub type TrigctlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSPRESCALE` reader - "]
pub type TsprescaleR = crate::FieldReader;
#[doc = "Field `TSPRESCALE` writer - "]
pub type TsprescaleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HWTEN` reader - "]
pub type HwtenR = crate::FieldReader;
#[doc = "Field `HWTEN` writer - "]
pub type HwtenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNCEN` reader - "]
pub type SyncenR = crate::FieldReader;
#[doc = "Field `SYNCEN` writer - "]
pub type SyncenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWOEN` reader - "]
pub type SwoenR = crate::FieldReader;
#[doc = "Field `SWOEN` writer - "]
pub type SwoenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn prot(&self) -> ProtR {
        ProtR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tsfreq(&self) -> TsfreqR {
        TsfreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn forcets(&self) -> ForcetsR {
        ForcetsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn tracebus(&self) -> TracebusR {
        TracebusR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn trigctl(&self) -> TrigctlR {
        TrigctlR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn tsprescale(&self) -> TsprescaleR {
        TsprescaleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn hwten(&self) -> HwtenR {
        HwtenR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn syncen(&self) -> SyncenR {
        SyncenR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn swoen(&self) -> SwoenR {
        SwoenR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn prot(&mut self) -> ProtW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec> {
        ProtW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TsW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec> {
        TsW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tsfreq(&mut self) -> TsfreqW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec> {
        TsfreqW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn forcets(&mut self) -> ForcetsW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec> {
        ForcetsW::new(self, 7)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SyncW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec> {
        SyncW::new(self, 8)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    #[must_use]
    pub fn tracebus(&mut self) -> TracebusW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec> {
        TracebusW::new(self, 10)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn trigctl(&mut self) -> TrigctlW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec> {
        TrigctlW::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn tsprescale(&mut self) -> TsprescaleW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec> {
        TsprescaleW::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn hwten(&mut self) -> HwtenW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec> {
        HwtenW::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SyncenW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec> {
        SyncenW::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn swoen(&mut self) -> SwoenW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec> {
        SwoenW::new(self, 22)
    }
}
#[doc = "Indicates the features of the STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat1r::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat1r::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT1R to value 0x0065_87d1"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat1rSpec {
    const RESET_VALUE: u32 = 0x0065_87d1;
}
