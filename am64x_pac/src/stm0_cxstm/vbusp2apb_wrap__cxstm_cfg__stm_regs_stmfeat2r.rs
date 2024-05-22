#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT2R` reader"]
pub type R = crate::R<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec>;
#[doc = "Register `VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT2R` writer"]
pub type W = crate::W<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec>;
#[doc = "Field `SPTER` reader - "]
pub type SpterR = crate::FieldReader;
#[doc = "Field `SPTER` writer - "]
pub type SpterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPER` reader - "]
pub type SperR = crate::BitReader;
#[doc = "Field `SPER` writer - "]
pub type SperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPCOMP` reader - "]
pub type SpcompR = crate::FieldReader;
#[doc = "Field `SPCOMP` writer - "]
pub type SpcompW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPOVERRIDE` reader - "]
pub type SpoverrideR = crate::BitReader;
#[doc = "Field `SPOVERRIDE` writer - "]
pub type SpoverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIVMASK` reader - "]
pub type PrivmaskR = crate::FieldReader;
#[doc = "Field `PRIVMASK` writer - "]
pub type PrivmaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPTRTYPE` reader - "]
pub type SptrtypeR = crate::FieldReader;
#[doc = "Field `SPTRTYPE` writer - "]
pub type SptrtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DSIZE` reader - "]
pub type DsizeR = crate::FieldReader;
#[doc = "Field `DSIZE` writer - "]
pub type DsizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPTYPE` reader - "]
pub type SptypeR = crate::FieldReader;
#[doc = "Field `SPTYPE` writer - "]
pub type SptypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spter(&self) -> SpterR {
        SpterR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sper(&self) -> SperR {
        SperR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn spcomp(&self) -> SpcompR {
        SpcompR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spoverride(&self) -> SpoverrideR {
        SpoverrideR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn privmask(&self) -> PrivmaskR {
        PrivmaskR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn sptrtype(&self) -> SptrtypeR {
        SptrtypeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn dsize(&self) -> DsizeR {
        DsizeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sptype(&self) -> SptypeR {
        SptypeR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn spter(&mut self) -> SpterW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec> {
        SpterW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sper(&mut self) -> SperW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec> {
        SperW::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn spcomp(&mut self) -> SpcompW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec> {
        SpcompW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn spoverride(&mut self) -> SpoverrideW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec> {
        SpoverrideW::new(self, 6)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    #[must_use]
    pub fn privmask(&mut self) -> PrivmaskW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec> {
        PrivmaskW::new(self, 7)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    #[must_use]
    pub fn sptrtype(&mut self) -> SptrtypeW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec> {
        SptrtypeW::new(self, 9)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DsizeW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec> {
        DsizeW::new(self, 12)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn sptype(&mut self) -> SptypeW<Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec> {
        SptypeW::new(self, 16)
    }
}
#[doc = "Indicates the features of the STM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec;
impl crate::RegisterSpec for Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat2r::R`](R) reader structure"]
impl crate::Readable for Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec {}
#[doc = "`write(|w| ..)` method takes [`vbusp2apb_wrap__cxstm_cfg__stm_regs_stmfeat2r::W`](W) writer structure"]
impl crate::Writable for Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUSP2APB_WRAP__CXSTM_CFG__STM_REGS_STMFEAT2R to value 0x0001_14f2"]
impl crate::Resettable for Vbusp2apbWrap_CxstmCfg_StmRegsStmfeat2rSpec {
    const RESET_VALUE: u32 = 0x0001_14f2;
}
