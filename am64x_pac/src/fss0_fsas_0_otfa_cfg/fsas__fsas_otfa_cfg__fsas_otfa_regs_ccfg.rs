#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ccfg` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ccfg` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec>;
#[doc = "Field `RD_WRT_OPT` reader - 3:0\\]
This register defines the static allocation of the AES cores to read transactions. The remainder will be allocated to write transactions"]
pub type RdWrtOptR = crate::FieldReader;
#[doc = "Field `RD_WRT_OPT` writer - 3:0\\]
This register defines the static allocation of the AES cores to read transactions. The remainder will be allocated to write transactions"]
pub type RdWrtOptW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `KEY_SIZE` reader - 4:4\\]
Key Size, 0 128 Bit 1 256 Bit"]
pub type KeySizeR = crate::BitReader;
#[doc = "Field `KEY_SIZE` writer - 4:4\\]
Key Size, 0 128 Bit 1 256 Bit"]
pub type KeySizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_EVICT_MODE` reader - 5:5\\]
cache evict mode"]
pub type CacheEvictModeR = crate::BitReader;
#[doc = "Field `CACHE_EVICT_MODE` writer - 5:5\\]
cache evict mode"]
pub type CacheEvictModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_ENABLE` reader - 6:6\\]
MAC cache enable"]
pub type CacheEnableR = crate::BitReader;
#[doc = "Field `CACHE_ENABLE` writer - 6:6\\]
MAC cache enable"]
pub type CacheEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTFA_WAIT` reader - 8:8\\]
This register allows the ability to stop accepting any new transactions from getting accepted and allow the current transactions to complete"]
pub type OtfaWaitR = crate::BitReader;
#[doc = "Field `OTFA_WAIT` writer - 8:8\\]
This register allows the ability to stop accepting any new transactions from getting accepted and allow the current transactions to complete"]
pub type OtfaWaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_RESP_EN` reader - 9:9\\]
This register controls the enabling the the ocp error response for mac errors"]
pub type ErrorRespEnR = crate::BitReader;
#[doc = "Field `ERROR_RESP_EN` writer - 9:9\\]
This register controls the enabling the the ocp error response for mac errors"]
pub type ErrorRespEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_EN_RD` reader - 31:31\\]
This register controls the enabling the functionality of this IP Disabled and Bypass mode active"]
pub type MasterEnRdR = crate::BitReader;
#[doc = "Field `MASTER_EN_RD` writer - 31:31\\]
This register controls the enabling the functionality of this IP Disabled and Bypass mode active"]
pub type MasterEnRdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This register defines the static allocation of the AES cores to read transactions. The remainder will be allocated to write transactions"]
    #[inline(always)]
    pub fn rd_wrt_opt(&self) -> RdWrtOptR {
        RdWrtOptR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Key Size, 0 128 Bit 1 256 Bit"]
    #[inline(always)]
    pub fn key_size(&self) -> KeySizeR {
        KeySizeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
cache evict mode"]
    #[inline(always)]
    pub fn cache_evict_mode(&self) -> CacheEvictModeR {
        CacheEvictModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
MAC cache enable"]
    #[inline(always)]
    pub fn cache_enable(&self) -> CacheEnableR {
        CacheEnableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
This register allows the ability to stop accepting any new transactions from getting accepted and allow the current transactions to complete"]
    #[inline(always)]
    pub fn otfa_wait(&self) -> OtfaWaitR {
        OtfaWaitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
This register controls the enabling the the ocp error response for mac errors"]
    #[inline(always)]
    pub fn error_resp_en(&self) -> ErrorRespEnR {
        ErrorRespEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
This register controls the enabling the functionality of this IP Disabled and Bypass mode active"]
    #[inline(always)]
    pub fn master_en_rd(&self) -> MasterEnRdR {
        MasterEnRdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This register defines the static allocation of the AES cores to read transactions. The remainder will be allocated to write transactions"]
    #[inline(always)]
    #[must_use]
    pub fn rd_wrt_opt(&mut self) -> RdWrtOptW<Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec> {
        RdWrtOptW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Key Size, 0 128 Bit 1 256 Bit"]
    #[inline(always)]
    #[must_use]
    pub fn key_size(&mut self) -> KeySizeW<Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec> {
        KeySizeW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
cache evict mode"]
    #[inline(always)]
    #[must_use]
    pub fn cache_evict_mode(&mut self) -> CacheEvictModeW<Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec> {
        CacheEvictModeW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
MAC cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn cache_enable(&mut self) -> CacheEnableW<Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec> {
        CacheEnableW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
This register allows the ability to stop accepting any new transactions from getting accepted and allow the current transactions to complete"]
    #[inline(always)]
    #[must_use]
    pub fn otfa_wait(&mut self) -> OtfaWaitW<Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec> {
        OtfaWaitW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
This register controls the enabling the the ocp error response for mac errors"]
    #[inline(always)]
    #[must_use]
    pub fn error_resp_en(&mut self) -> ErrorRespEnW<Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec> {
        ErrorRespEnW::new(self, 9)
    }
    #[doc = "Bit 31 - 31:31\\]
This register controls the enabling the functionality of this IP Disabled and Bypass mode active"]
    #[inline(always)]
    #[must_use]
    pub fn master_en_rd(&mut self) -> MasterEnRdW<Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec> {
        MasterEnRdW::new(self, 31)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ccfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_ccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_ccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_ccfg::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_ccfg::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_ccfg to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsCcfgSpec {
    const RESET_VALUE: u32 = 0;
}
